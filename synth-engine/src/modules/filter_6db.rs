use super::control_to_frequency;
use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;
use std::f32::consts::PI;

pub struct Filter6db {
    f0: f32,
    internal_state: usize,
    lowpass_output: usize,
    highpass_output: usize,
    freq_control_input: StackProgram,
    linear_control: StackProgram,
    signal_input: StackProgram,
}

impl Filter6db {
    pub fn new(
        f0: f32,
        internal_state: usize,
        lowpass_output: usize,
        highpass_output: usize,
        freq_control_input: StackProgram,
        linear_control: StackProgram,
        signal_input: StackProgram,
    ) -> Self {
        Self {
            f0,
            internal_state,
            lowpass_output,
            highpass_output,
            freq_control_input,
            linear_control,
            signal_input,
        }
    }
}

impl Module for Filter6db {
    fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]) {
        let input = self.signal_input.run(state, stack).unwrap_or(0.);
        let f = control_to_frequency(
            self.f0,
            self.freq_control_input.run(state, stack).unwrap_or(0.),
            self.linear_control.run(state, stack).unwrap_or(0.),
        );

        let a = 2. * PI * f;

        update.set(
            self.lowpass_output,
            a * (input - state.get(self.lowpass_output)),
            UpdateType::Differentiable,
        );

        update.set(
            self.highpass_output,
            input - a * state.get(self.internal_state),
            UpdateType::Absolute,
        );

        update.set(
            self.internal_state,
            state.get(self.highpass_output),
            UpdateType::Differentiable,
        );
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, _state: &mut State, _time_step: f32, _stack: &mut [f32]) {
        /* do nothing */
    }
}
