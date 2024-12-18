use super::control_to_frequency;
use super::wavetable::*;
use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;
use core::f32::consts::PI;
use libm::{floorf, fabsf};

// TODO cleanup
// #[cfg(any(feature = "allocator", test))]
// use alloc::vec::Vec;

pub struct Vosim<'a, 'b> {
    f0: f32,
    position_state: usize,
    signal_output: usize,
    pitch_control: StackProgram<'b>,
    linear_modulation: StackProgram<'b>,
    grain_pitch_control: StackProgram<'b>,
    grain_linear_modulation: StackProgram<'b>,
    wavetable_select: StackProgram<'b>,
    wavetables: &'a [WavetableEntry<'a>],
    amp: f32,
}

impl<'a, 'b> Vosim<'a, 'b> {
    /*
    #[cfg(any(feature = "allocator", test))]
    pub fn new(
        f0: f32,
        position_state: usize,
        signal_output: usize,
        pitch_control: StackProgram<'b>,
        linear_modulation: StackProgram<'b>,
        grain_pitch_control: StackProgram<'b>,
        grain_linear_modulation: StackProgram<'b>,
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
            grain_pitch_control,
            grain_linear_modulation,
            wavetable_select,
            wavetables: &wavetable_entries,
            amp: 2. * PI * FREQUENCY_LIMIT,
        }
    }
    */

    pub fn new_with_precompute(
        f0: f32,
        position_state: usize,
        signal_output: usize,
        pitch_control: StackProgram<'b>,
        linear_modulation: StackProgram<'b>,
        grain_pitch_control: StackProgram<'b>,
        grain_linear_modulation: StackProgram<'b>,
        wavetable_select: StackProgram<'b>,
        wavetables: &'a [WavetableEntry<'a>],
    ) -> Self {
        Self {
            f0,
            position_state,
            signal_output,
            pitch_control,
            linear_modulation,
            grain_pitch_control,
            grain_linear_modulation,
            wavetable_select,
            wavetables,
            amp: 2. * PI * FREQUENCY_LIMIT,
        }
    }
}

impl<'a, 'b> Module for Vosim<'a, 'b> {
    fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]) {
        let velocity = control_to_frequency(
            self.f0,
            self.pitch_control.run(state, stack).unwrap_or(0.),
            self.linear_modulation.run(state, stack).unwrap_or(0.),
        );
        let grain_velocity = control_to_frequency(
            self.f0,
            self.grain_pitch_control.run(state, stack).unwrap_or(0.),
            self.grain_linear_modulation.run(state, stack).unwrap_or(0.),
        )
        .max(velocity);

        let position = state.get(self.position_state);
        let grain_distance = update.get_time_step() * velocity;

        let position = ((position % 1.) + 1.) % 1.;
        let grain_ratio = if grain_velocity > f32::EPSILON {
            fabsf(velocity) / fabsf(grain_velocity)
        } else {
            0.
        };

        let wavetable_sample = if position >= grain_ratio {
            0.
        } else {
            let position = position / grain_ratio;

            if self.wavetables.len() == 1 {
                self.wavetables[0].eval(grain_distance, position)
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

                let v1 = self.wavetables[index0].eval(grain_distance, position);
                let v2 = self.wavetables[index1].eval(grain_distance, position);

                v1 + (v2 - v1) * x
            } else {
                0.
            }
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
