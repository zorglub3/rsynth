use super::control_to_frequency;
use crate::control_interface::ControlInterface;
use crate::simulator::module::Module;
use crate::simulator::state::UpdateType;
// use crate::stack_program::*;
use core::f32::consts::PI;

pub struct Filter6db {
    f0: f32,
    // internal_state: usize,
    // lowpass_output: usize,
    // highpass_output: usize,
    // freq_control_input: StackProgram,
    // linear_control: StackProgram,
    // signal_input: StackProgram,
}

impl Filter6db {
    pub const STATE_SIZE: usize = 3;
    pub const INPUT_SIZE: usize = 3;

    // state/outputs
    pub const INTERNAL_STATE: usize = 0;
    pub const LOWPASS_OUTPUT: usize = 1;
    pub const HIGHPASS_OUTPUT: usize  = 2;

    // inputs/control
    pub const EXP_CONTROL_INPUT: usize = 0;
    pub const LINEAR_CONTROL_INPUT: usize = 1;
    pub const SIGNAL_INPUT: usize = 2;

    pub fn new(
        f0: f32,
        // internal_state: usize,
        // lowpass_output: usize,
        // highpass_output: usize,
        // freq_control_input: StackProgram,
        // linear_control: StackProgram,
        // signal_input: StackProgram,
    ) -> Self {
        Self {
            f0,
            // internal_state,
            // lowpass_output,
            // highpass_output,
            // freq_control_input,
            // linear_control,
            // signal_input,
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
        let input = inputs[Filter6db::SIGNAL_INPUT];
        let f = control_to_frequency(
            self.f0,
            inputs[Filter6db::EXP_CONTROL_INPUT],
            inputs[Filter6db::LINEAR_CONTROL_INPUT],
        );
        let a = 2. * PI * f;

        update[Filter6db::LOWPASS_OUTPUT] = a * (input - state[Filter6db::LOWPASS_OUTPUT]);
        update[Filter6db::HIGHPASS_OUTPUT] = input - a * state[Filter6db::INTERNAL_STATE];
        update[Filter6db::INTERNAL_STATE] = state[Filter6db::HIGHPASS_OUTPUT];
    }

    fn finalize(&mut self, _inputs: &[f32], _state: &mut [f32], _outputs: &mut [f32], _dt: f32) {
        /* do nothing */
    }

    fn get_input_size(&self) -> usize {
        Filter6db::INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        Filter6db::STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        update_types[Filter6db::LOWPASS_OUTPUT] = UpdateType::Differentiable;
        update_types[Filter6db::HIGHPASS_OUTPUT] = UpdateType::Absolute;
        update_types[Filter6db::INTERNAL_STATE] = UpdateType::Differentiable;
    }
}
