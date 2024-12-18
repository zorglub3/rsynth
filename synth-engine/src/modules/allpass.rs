use super::control_to_frequency;
use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;
use core::f32::consts::PI;

pub struct AllpassFilter<'a> {
    f0: f32,
    cap_state: usize,
    signal_output: usize,
    freq_control_input: StackProgram<'a>,
    linear_control: StackProgram<'a>,
    signal_input: StackProgram<'a>,
}

impl<'a> AllpassFilter<'a> {
    pub fn new(
        f0: f32,
        cap_state: usize,
        signal_output: usize,
        freq_control_input: StackProgram<'a>,
        linear_control: StackProgram<'a>,
        signal_input: StackProgram<'a>,
    ) -> Self {
        Self {
            f0,
            cap_state,
            signal_output,
            freq_control_input,
            linear_control,
            signal_input,
        }
    }
}

impl<'a> Module for AllpassFilter<'a> {
    fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]) {
        let input = self.signal_input.run(state, stack).unwrap_or(0.);
        let f = control_to_frequency(
            self.f0,
            self.freq_control_input.run(state, stack).unwrap_or(0.),
            self.linear_control.run(state, stack).unwrap_or(0.),
        );

        update.set(
            self.cap_state,
            2.0 * PI * f * (input - state.get(self.cap_state)),
            UpdateType::Differentiable,
        );

        update.set(
            self.signal_output,
            2. * state.get(self.cap_state) - input,
            UpdateType::Absolute,
        );
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, _state: &mut State, _time_step: f32, _stack: &mut [f32]) {
        /* do nothing */
    }
}
