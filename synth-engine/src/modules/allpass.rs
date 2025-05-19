use super::control_to_frequency;
use crate::control_interface::ControlInterface;
use crate::simulator::module::Module;
use crate::simulator::state::UpdateType;
// use crate::stack_program::*;
use core::f32::consts::PI;

pub struct AllpassFilter {
    f0: f32,
    // cap_state: usize,
    // signal_output: usize,
    // freq_control_input: StackProgram,
    // linear_control: StackProgram,
    // signal_input: StackProgram,
}

impl AllpassFilter {
    pub const STATE_SIZE: usize = 2;
    pub const INPUT_SIZE: usize = 3;

    // state/outputs
    pub const CAP_STATE: usize = 0;
    pub const SIGNAL_OUTPUT: usize = 1;

    // inputs
    pub const FREQ_CONTROL_INPUT: usize = 0;
    pub const LINEAR_CONTROL_INPUT: usize = 1;
    pub const SIGNAL_INPUT: usize = 2;

    pub fn new(
        f0: f32,
        // cap_state: usize,
        // signal_output: usize,
        // freq_control_input: StackProgram,
        // linear_control: StackProgram,
        // signal_input: StackProgram,
    ) -> Self {
        Self {
            f0,
            // cap_state,
            // signal_output,
            // freq_control_input,
            // linear_control,
            // signal_input,
        }
    }
}

impl Module for AllpassFilter {
    fn simulate(
        &self,
        _control_interface: &ControlInterface,
        inputs: &[f32],
        state: &[f32],
        update: &mut [f32],
        _dt: f32,
    ) {
        let input = inputs[AllpassFilter::SIGNAL_INPUT];
        let f = control_to_frequency(
            self.f0,
            inputs[AllpassFilter::FREQ_CONTROL_INPUT],
            inputs[AllpassFilter::LINEAR_CONTROL_INPUT],
        );

        update[AllpassFilter::CAP_STATE] = 2. * PI * f * (input - state[AllpassFilter::CAP_STATE]);
        update[AllpassFilter::SIGNAL_OUTPUT] = 2. * state[AllpassFilter::CAP_STATE] - input;
    }

    fn finalize(&mut self, _inputs: &[f32], _state: &mut [f32], _outputs: &mut [f32], _dt: f32) {
        /* do nothing */
    }

    fn get_input_size(&self) -> usize {
        AllpassFilter::INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        AllpassFilter::STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        update_types[AllpassFilter::CAP_STATE] = UpdateType::Differentiable;
        update_types[AllpassFilter::SIGNAL_OUTPUT] = UpdateType::Absolute;
    }
}
