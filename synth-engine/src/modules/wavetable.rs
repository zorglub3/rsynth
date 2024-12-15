use super::control_to_frequency;
use crate::event::ControllerEvent;
use crate::interpolation::Interpolation;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::sinc_filter::downsample_half;
use crate::stack_program::*;
use alloc::vec::Vec;
use core::f32::consts::PI;

const FREQUENCY_LIMIT: f32 = 18_000.0;

pub struct WavetableData {
    pub samples: Vec<f32>,
    pub len_f32: f32,
}

impl WavetableData {
    fn from_slice(samples: &[f32]) -> Self {
        let mut samples_vec: Vec<f32> = Vec::new();
        let len_f32 = samples.len() as f32;

        samples_vec.extend_from_slice(samples);

        Self {
            samples: samples_vec,
            len_f32,
        }
    }

    fn downsample(&self) -> Option<Self> {
        if self.samples.len() < 4 {
            None
        } else {
            let m = self.samples.len().min(128);
            let samples_downsampled = downsample_half(m, &self.samples);
            Some(WavetableData::from_slice(&samples_downsampled))
        }
    }

    fn eval(&self, x: f32) -> f32 {
        let x = x * self.len_f32;
        self.samples.cubic_interpolate(x)
    }
}

pub struct WavetableEntry {
    pub data: Vec<WavetableData>,
    pub base_data_len: usize,
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
            if samples_per_step <= 1. {
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
}

pub struct Wavetable {
    f0: f32,
    position_state: usize,
    signal_output: usize,
    pitch_control: StackProgram,
    linear_modulation: StackProgram,
    wavetable_select: StackProgram,
    wavetables: Vec<WavetableEntry>,
    amp: f32,
}

impl Wavetable {
    pub fn new(
        f0: f32,
        position_state: usize,
        signal_output: usize,
        pitch_control: StackProgram,
        linear_modulation: StackProgram,
        wavetable_select: StackProgram,
        wavetables: Vec<Vec<f32>>,
    ) -> Self {
        Self {
            f0,
            position_state,
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

    pub fn precompute_wavetables(table_data: &Vec<Vec<f32>>) -> Vec<WavetableEntry> {
        table_data
            .into_iter()
            .map(|samples| WavetableEntry::from_slice(&samples))
            .collect()
    }

    pub fn new_with_precompute(
        f0: f32,
        position_state: usize,
        signal_output: usize,
        pitch_control: StackProgram,
        linear_modulation: StackProgram,
        wavetable_select: StackProgram,
        wavetables: Vec<WavetableEntry>,
    ) -> Self {
        Self {
            f0,
            position_state,
            signal_output,
            pitch_control,
            linear_modulation,
            wavetable_select,
            wavetables,
            amp: 2. * PI * FREQUENCY_LIMIT,
        }
    }
}

impl Module for Wavetable {
    fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]) {
        let velocity = control_to_frequency(
            self.f0,
            self.pitch_control.run(state, stack).unwrap_or(0.),
            self.linear_modulation.run(state, stack).unwrap_or(0.),
        );
        let position = state.get(self.position_state);
        let distance = update.get_time_step() * velocity;

        let wavetable_sample = if self.wavetables.len() == 1 {
            self.wavetables[0].eval(distance, position)
        } else if self.wavetables.len() > 1 {
            let scan = self
                .wavetable_select
                .run(state, stack)
                .unwrap_or(0.)
                .min(1.)
                .max(0.);
            let scan_select = scan * ((self.wavetables.len() - 1) as f32);
            let index = scan_select.floor() as usize;
            let x = scan_select.fract();
            let index0 = index.min(self.wavetables.len() - 1);
            let index1 = (index + 1).min(self.wavetables.len() - 1);

            let v1 = self.wavetables[index0].eval(distance, position);
            let v2 = self.wavetables[index1].eval(distance, position);

            v1 + (v2 - v1) * x
        } else {
            0.
        };

        update.set(
            self.signal_output,
            self.amp * (wavetable_sample - state.get(self.signal_output)),
            UpdateType::Differentiable,
        );
        update.set(self.position_state, velocity, UpdateType::Differentiable);
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State, _time_step: f32, _stack: &mut [f32]) {
        let p = ((state.get(self.position_state) % 1.) + 1.) % 1.;

        state.set(self.position_state, p);
    }
}
