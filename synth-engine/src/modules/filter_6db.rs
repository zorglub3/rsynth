use super::control_to_frequency;
use crate::control_interface::ControlInterface;
use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;
use core::f32::consts::PI;

pub const STATE_SIZE: usize = 3;
pub const INPUT_SIZE: usize = 3;

// state/outputs
const INTERNAL_STATE: usize = 0;
const LOWPASS_OUTPUT: usize = 1;
const HIGHPASS_OUTPUT: usize  = 2;

// inputs/control
const EXP_CONTROL_INPUT: usize = 0;
const LINEAR_CONTROL_INPUT: usize = 1;
const SIGNAL_INPUT: usize = 2;

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
    fn simulate(
        &self,
        _control_interface: &ControlInterface,
        inputs: &[f32],
        state: &[f32],
        update: &mut [f32],
        _dt: f32,
    ) {
        let input = inputs[SIGNAL_INPUT];
        let f = control_to_frequency(
            self.f0,
            inputs[EXP_CONTROL_INPUT],
            inputs[LINEAR_CONTROL_INPUT],
        );
        let a = 2. * PI * f;

        update[LOWPASS_OUTPUT] = a * (input - state[LOWPASS_OUTPUT]);
        update[HIGHPASS_OUTPUT] = input - a * state[INTERNAL_STATE];
        update[INTERNAL_STATE] = state[HIGHPASS_OUTPUT];
    }

    fn finalize(&mut self, _inputs: &[f32], _state: &mut [f32], _outputs: &mut [f32], _dt: f32) {
        /* do nothing */
    }

    fn get_input_size(&self) -> usize {
        INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        update_types[LOWPASS_OUTPUT] = UpdateType::Differentiable;
        update_types[HIGHPASS_OUTPUT] = UpdateType::Absolute;
        update_types[INTERNAL_STATE] = UpdateType::Differentiable;
    }

    /*
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
    */
}
