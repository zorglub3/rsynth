use super::control_to_frequency;
use crate::event::ControllerEvent;
use crate::interpolation::Interpolation;
use crate::modules::input_expr::InputExpr;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::sinc_filter::downsample_half;
use std::f32::consts::PI;

const FREQUENCY_LIMIT: f32 = 20_000.0;
const RESONANCE_FEEDBACK: f32 = 1.5;

// TODO susceptible to offset errors - preprocess each wavetable to make average=0

fn compute_cumulative_sums(samples: &[f32]) -> Vec<f32> {
    if samples.len() == 0 {
        Vec::new()
    } else {
        let mut result = Vec::with_capacity(samples.len());
        let len_inv = 1. / (samples.len() as f32);

        result.push(0.0);

        for index in 1..samples.len() {
            let a = index - 1;
            let b = index;

            let f_a = samples[a];
            let f_b = samples[b];
            let f_mid = samples.cubic_interpolate((index as f32) - 0.5);

            // Simpsons rule, see
            // https://en.wikipedia.org/wiki/Simpson%27s_rule
            // let v = (((b - a) as f32) / 6.) * (f_a + 4. * f_mid + f_b);
            let v = (1. / 6.) * (f_a + 4. * f_mid + f_b);

            result.push(v * len_inv);
        }

        for index in 1..result.len() {
            result[index] += result[index - 1];
        }

        result
    }
}

struct WavetableData {
    samples: Vec<f32>,
    cumulative_sums: Vec<f32>,
    len_f32: f32,
    len_inv: f32,
}

impl WavetableData {
    fn from_slice(samples: &[f32]) -> Self {
        let mut samples_vec: Vec<f32> = Vec::new();
        let cumulative_sums = compute_cumulative_sums(samples);
        let len_f32 = samples.len() as f32;
        let len_inv = 1. / len_f32;

        samples_vec.extend_from_slice(samples);

        Self {
            samples: samples_vec,
            cumulative_sums,
            len_f32,
            len_inv,
        }
    }

    fn downsample(&self) -> Option<Self> {
        if self.samples.len() < 4 {
            None
        } else {
            let m = self.samples.len() / 2;
            let samples_downsampled = downsample_half(m, &self.samples);
            Some(WavetableData::from_slice(&samples_downsampled))
        }
    }

    fn eval(&self, x: f32) -> f32 {
        let x = x * self.len_f32;
        self.samples.cubic_interpolate(x)
    }

    fn integral(&self, start: f32, end: f32) -> f32 {
        let start = start * self.len_f32;
        let end = end * self.len_f32;

        let start_frac = ((start % 1.) + 1.) % 1.;
        let end_frac = ((end % 1.) + 1.) % 1.;
        let start_floor = (start.floor() as usize) % self.samples.len();
        let end_floor = (end.floor() as usize) % self.samples.len();

        let start_integral = self.cumulative_sums[start_floor]
            + self.len_inv
                * (start_frac / 6.)
                * (self.samples[start_floor]
                    + 4. * self.samples.cubic_interpolate(start - start_frac / 2.)
                    + self.samples.cubic_interpolate(start));

        let end_integral = self.cumulative_sums[end_floor]
            + self.len_inv
                * (end_frac / 6.)
                * (self.samples[end_floor]
                    + 4. * self.samples.cubic_interpolate(end - end_frac / 2.)
                    + self.samples.cubic_interpolate(end));

        end_integral - start_integral
    }
}

struct WavetableEntry {
    data: Vec<WavetableData>,
    base_data_len: usize,
}

impl WavetableEntry {
    fn from_slice(samples: &[f32]) -> Self {
        let mut data = Vec::new();

        let mut current_wavetable_data = WavetableData::from_slice(samples);

        while let Some(next_wavetable_data) = current_wavetable_data.downsample() {
            data.push(current_wavetable_data);
            current_wavetable_data = next_wavetable_data;
        }

        Self {
            data,
            base_data_len: samples.len(),
        }
    }

    fn get_data_by_frequency(&self, cycles_per_step: f32) -> Option<&WavetableData> {
        let mut samples_per_step = (self.base_data_len as f32) * cycles_per_step;

        for i in 0..self.data.len() {
            if samples_per_step < 1. {
                return Some(&self.data[i]);
            } else {
                samples_per_step /= 2.;
            }
        }

        None
    }

    fn eval(&self, cycles_per_step: f32, x: f32) -> f32 {
        if let Some(data) = self.get_data_by_frequency(cycles_per_step) {
            data.eval(x)
        } else {
            0.
        }
    }

    fn integral(&self, cycles_per_step: f32, start: f32, end: f32) -> f32 {
        if let Some(data) = self.get_data_by_frequency(cycles_per_step) {
            data.integral(start, end)
        } else {
            0.
        }
    }
}

pub struct Wavetable {
    f0: f32,
    position_state: usize,
    filter_state: usize,
    signal_output: usize,
    pitch_control: InputExpr,
    linear_modulation: InputExpr,
    wavetable_select: InputExpr,
    wavetables: Vec<WavetableEntry>,
    amp: f32,
}

impl Wavetable {
    pub fn new(
        f0: f32,
        position_state: usize,
        filter_state: usize,
        signal_output: usize,
        pitch_control: InputExpr,
        linear_modulation: InputExpr,
        wavetable_select: InputExpr,
        wavetables: Vec<Vec<f32>>,
    ) -> Self {
        Self {
            f0,
            position_state,
            filter_state,
            signal_output,
            pitch_control,
            linear_modulation,
            wavetable_select,
            wavetables: wavetables
                .into_iter()
                .map(|samples| WavetableEntry::from_slice(&samples))
                .collect(),
            amp: 2. * PI * FREQUENCY_LIMIT,
        }
    }
}

impl Module for Wavetable {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let velocity = control_to_frequency(
            self.f0,
            self.pitch_control.from_state(state),
            self.linear_modulation.from_state(state),
        );
        let distance = update.get_time_step() * velocity;
        let start = state.get(self.position_state);
        let end = start + distance;

        let integral = if self.wavetables.len() == 1 {
            if distance.abs() < f32::EPSILON {
                self.wavetables[0].eval(distance, start)
            } else {
                self.wavetables[0].integral(distance, start, end) / distance
            }
        } else if self.wavetables.len() > 1 {
            let scan = self.wavetable_select.from_state(state).min(1.).max(0.);
            let scan_select = scan * ((self.wavetables.len() - 1) as f32);
            let index = scan_select.floor() as usize;
            let x = scan_select.fract();
            let index0 = index.min(self.wavetables.len() - 1);
            let index1 = (index + 1).min(self.wavetables.len() - 1);

            if distance.abs() < f32::EPSILON {
                let v1 = self.wavetables[index0].eval(distance, start);
                let v2 = self.wavetables[index1].eval(distance, start);

                v1 + (v2 - v1) * x
            } else {
                let v1 = self.wavetables[index0].integral(distance, start, end) / distance;
                let v2 = self.wavetables[index1].integral(distance, start, end) / distance;

                v1 + (v2 - v1) * x
            }
        } else {
            0.
        };

        let b = RESONANCE_FEEDBACK;

        update.set(
            self.filter_state,
            self.amp
                * (integral - b * state.get(self.filter_state) - state.get(self.signal_output)),
            UpdateType::Differentiable,
        );
        update.set(
            self.signal_output,
            self.amp * state.get(self.filter_state),
            UpdateType::Differentiable,
        );
        update.set(self.position_state, velocity, UpdateType::Differentiable);

        /*
        let velocity =
            control_to_frequency(
                self.f0,
                self.pitch_control.from_state(state),
                self.linear_modulation.from_state(state));

        let distance = update.get_time_step() * velocity;
        let start = self.current_position + update.get_delta_time() * velocity;
        let end = start + distance;

        let i =
            if self.wavetables.len() == 1 {
                if distance.abs() < f32::EPSILON {
                    self.eval(0, start)
                } else {
                    self.integral(0, start, end) / distance
                }
            } else {
                let scan =
                    self.wavetable_select.from_state(state).min(1.).max(0.);
                let scan_index = scan * (self.wavetables.len() as f32);
                let index = scan_index.floor() as usize;
                let x = scan_index.fract();
                let index0 = index.min(self.wavetables.len() - 1);
                let index1 = (index + 1).min(self.wavetables.len() - 1);

                if distance.abs() < f32::EPSILON {
                    let v1 = self.eval(index0, start);
                    let v2 = self.eval(index1, start);

                    v1 + (v2 - v1) * x
                } else {
                    let v1 = self.integral(index0, start, end) / distance;
                    let v2 = self.integral(index1, start, end) / distance;

                    v1 + (v2 - v1) * x
                }
            };

        let a = self.amp;
        let b = 1.5;

        update.set(
            self.filter_state,
            a * (i - b * state.get(self.filter_state) - state.get(self.signal_output)),
            UpdateType::Differentiable);
        update.set(
            self.signal_output,
            a * state.get(self.filter_state),
            UpdateType::Differentiable);
            */
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State, time_step: f32) {
        let p = ((state.get(self.position_state) % 1.) + 1.) % 1.;

        state.set(self.position_state, p);

        /*
        let velocity =
            control_to_frequency(
                self.f0,
                self.pitch_control.from_state(state),
                self.linear_modulation.from_state(state));

        let p = self.current_position + velocity * time_step;

        self.current_position = ((p % 1.) + 1.) % 1.;
        */
    }
}
