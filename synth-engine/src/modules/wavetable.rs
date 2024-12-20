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
pub fn precompute_wavetable(table_data: &[f32]) -> Vec<Vec<f32>> {
    let mut entry_data = Vec::new();

    let mut current_entry = table_data.to_vec();

    while let Some(next_entry) = downsample(&current_entry) {
        entry_data.push(current_entry);
        current_entry = next_entry;
    }

    entry_data.push(current_entry.to_vec());

    entry_data
}

#[cfg(any(feature = "allocator", test))]
pub fn build_wavetable(samples: &[f32]) -> (Vec<f32>, Vec<WavetableDataEntry>) {
    let mut data_vec: Vec<f32> = Vec::new();
    let mut entry_vec: Vec<WavetableDataEntry> = Vec::new();
    let mut start: usize = 0;
    let mut end: usize;

    for e in precompute_wavetable(samples) {
        end = start + e.len();
        let len_f32 = e.len() as f32;

        entry_vec.push(WavetableDataEntry {
            start,
            end,
            len_f32,
        });
        data_vec.extend_from_slice(&e);

        start = end;
    }

    (data_vec, entry_vec)
}

pub struct WavetableDataEntry {
    pub start: usize,
    pub end: usize,
    pub len_f32: f32,
}

impl WavetableDataEntry {
    pub fn new(start: usize, end: usize, len_f32: f32) -> Self {
        Self {
            start,
            end,
            len_f32,
        }
    }
}

pub struct Wavetable<'a> {
    pub data: &'a [f32],
    pub entries: &'a [WavetableDataEntry],
    pub base_data_size: f32,
}

impl<'a> Wavetable<'a> {
    pub fn eval(&self, cycles_per_step: f32, x: f32) -> f32 {
        let mut samples_per_step = cycles_per_step * self.base_data_size;

        for entry in self.entries {
            if samples_per_step <= 1. {
                let data = &self.data[entry.start..entry.end];
                let x = x * entry.len_f32;
                return data.cubic_interpolate(x);
            } else {
                samples_per_step /= 2.;
            }
        }

        0.
    }
}

pub struct WavetableOscillator<'a, 'b> {
    f0: f32,
    position_state: usize,
    signal_output: usize,
    pitch_control: StackProgram<'b>,
    linear_modulation: StackProgram<'b>,
    wavetable_select: StackProgram<'b>,
    wavetables: &'a [Wavetable<'a>],
    amp: f32,
}

impl<'a, 'b> WavetableOscillator<'a, 'b> {
    pub fn new_with_precompute(
        f0: f32,
        position_state: usize,
        signal_output: usize,
        pitch_control: StackProgram<'b>,
        linear_modulation: StackProgram<'b>,
        wavetable_select: StackProgram<'b>,
        wavetables: &'a [Wavetable<'a>],
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

impl<'a, 'b> Module for WavetableOscillator<'a, 'b> {
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
