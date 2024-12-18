use super::control_to_frequency;
use crate::event::ControllerEvent;
use crate::interpolation::Interpolation;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;
use core::f32::consts::PI;
use libm::floorf;

#[cfg(any(feature = "allocator", test))]
use alloc::vec::Vec;

#[cfg(any(feature = "allocator", test))]
use crate::sinc_filter::downsample_half;

pub const FREQUENCY_LIMIT: f32 = 18_000.0;

pub struct WavetableData<'a> {
    pub samples: &'a [f32],
    pub len_f32: f32,
}

impl<'a> WavetableData<'a> {
    fn from_slice(samples: &'a [f32]) -> Self {
        let len_f32 = samples.len() as f32;

        Self {
            samples,
            len_f32,
        }
    }

    #[cfg(any(feature = "allocator", test))]
    fn downsample(&self) -> Option<Vec<f32>> {
        if self.samples.len() < 4 {
            None
        } else {
            let m = (self.samples.len() / 2).min(32);
            Some(downsample_half(m, &self.samples))
        }
    }

    pub fn eval(&self, x: f32) -> f32 {
        let x = x * self.len_f32;
        self.samples.cubic_interpolate(x)
    }
}

pub struct WavetableEntry<'a> {
    pub data: &'a [WavetableData<'a>],
    pub base_data_len: usize,
}

impl<'a> WavetableEntry<'a> {
    /*
    #[cfg(any(feature = "allocator", test))]
    pub fn from_slice(samples: &'a [f32]) -> Self {
        let mut data = Vec::new();

        let mut current_wavetable_data = WavetableData::from_slice(samples);

        while let Some(next_wavetable_data) = current_wavetable_data.downsample() {
            data.push(current_wavetable_data);
            current_wavetable_data = next_wavetable_data;
        }

        Self {
            data: &data,
            base_data_len: samples.len(),
        }
    }
    */

    #[cfg(any(feature = "allocator", test))]
    pub fn downsample(data: &[f32]) -> Option<Vec<f32>> {
        if data.len() < 8 {
            None
        } else {
            let m = (data.len() / 2).min(32);
            Some(downsample_half(m, data))
        }
    }
 
    #[cfg(any(feature = "allocator", test))]
    pub fn make_data(data: &[f32]) -> Vec<Vec<f32>> {
        let mut result = Vec::new();
        
        let mut current_entry = data.to_vec();

        result.push(current_entry.clone());

        while let Some(next_wavetable_data) = Self::downsample(&current_entry) {
            result.push(next_wavetable_data.clone());
            current_entry = next_wavetable_data;
        }

        result
    }

    #[cfg(any(features = "allocator", test))]
    pub fn make_wavetable_data(vecs: &'a Vec<Vec<f32>>) -> Vec<WavetableData<'a>> {
        let mut data = Vec::new();

        for v in vecs {
            data.push(WavetableData::from_slice(&v));
        }

        data
    }

    #[cfg(any(feature = "allocator", test))]
    pub fn from_data(data: &'a Vec<WavetableData<'a>>) -> Self {
        let base_data_len = data[0].samples.len();

        Self { data, base_data_len }
    }

    fn get_data_by_frequency(&self, cycles_per_step: f32) -> Option<&WavetableData> {
        let mut samples_per_step = (self.base_data_len as f32) * cycles_per_step;

        for i in 0 .. self.data.len() {
            if samples_per_step <= 1. {
                return Some(&self.data[i]);
            } else {
                samples_per_step /= 2.;
            }
        }

        None
    }

    pub fn eval(&self, cycles_per_step: f32, x: f32) -> f32 {
        if let Some(data) = self.get_data_by_frequency(cycles_per_step) {
            data.eval(x)
        } else {
            0.
        }
    }
}

pub struct Wavetable<'a, 'b> {
    f0: f32,
    position_state: usize,
    signal_output: usize,
    pitch_control: StackProgram<'b>,
    linear_modulation: StackProgram<'b>,
    wavetable_select: StackProgram<'b>,
    wavetables: &'a [WavetableEntry<'a>],
    amp: f32,
}

impl<'a, 'b> Wavetable<'a, 'b> {
    /*
    #[cfg(any(feature = "allocator", test))]
    pub fn new(
        f0: f32,
        position_state: usize,
        signal_output: usize,
        pitch_control: StackProgram<'b>,
        linear_modulation: StackProgram<'b>,
        wavetable_select: StackProgram<'b>,
        wavetables: Vec<Vec<f32>>,
    ) -> Self {
        let wavetable_entries: Vec<WavetableEntry<'a>> = 
            wavetables
                .into_iter()
                .map(|samples| WavetableEntry::from_slice(&samples))
                .collect();

        Self {
            f0,
            position_state,
            signal_output,
            pitch_control,
            linear_modulation,
            wavetable_select,
            wavetables: &wavetable_entries,
            amp: 2. * PI * FREQUENCY_LIMIT,
        }
    }
    */

    /*
    #[cfg(any(feature = "allocator", test))]
    pub fn precompute_wavetables(table_data: &Vec<Vec<f32>>) -> Vec<WavetableEntry> {
        table_data
            .into_iter()
            .map(|samples| WavetableEntry::from_slice(&samples))
            .collect()
    }
    */

    pub fn new_with_precompute(
        f0: f32,
        position_state: usize,
        signal_output: usize,
        pitch_control: StackProgram<'b>,
        linear_modulation: StackProgram<'b>,
        wavetable_select: StackProgram<'b>,
        wavetables: &'a [WavetableEntry<'a>],
    ) -> Self {
        Self {
            f0,
            position_state,
            signal_output,
            pitch_control,
            linear_modulation,
            wavetable_select,
            wavetables: &wavetables,
            amp: 2. * PI * FREQUENCY_LIMIT,
        }
    }
}

impl<'a, 'b> Module for Wavetable<'a, 'b> {
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
                .clamp(0., 1.);
            let scan_select = scan * ((self.wavetables.len() - 1) as f32);
            let scan_select_floor = floorf(scan_select);
            let index = scan_select_floor as usize;
            let x = scan_select - scan_select_floor;
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
