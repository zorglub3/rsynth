use crate::control_interface::ControlInterface;
use crate::simulator::module::Module;
use crate::simulator::state::UpdateType;
// use crate::stack_program::*;

pub struct MonoOutput {
    output_index: usize,
    // signal_input: StackProgram,
}

impl MonoOutput {
    pub const STATE_SIZE: usize = 0;
    pub const INPUT_SIZE: usize = 1;

    // inputs/controls
    pub const SIGNAL_INPUT: usize = 0;

    pub fn new(
        output_index: usize, 
        // signal_input: StackProgram,
    ) -> Self {
        Self {
            output_index,
            // signal_input,
        }
    }
}

impl Module for MonoOutput {
    fn simulate(
        &self,
        _control_interface: &ControlInterface,
        _inputs: &[f32],
        _state: &[f32],
        _update: &mut [f32],
        _dt: f32,
    ) {
        /* do nothing */
    }

    fn finalize(&mut self, inputs: &[f32], _state: &mut [f32], outputs: &mut [f32], _dt: f32) {
        outputs[self.output_index] = inputs[MonoOutput::SIGNAL_INPUT];
    }

    fn get_input_size(&self) -> usize {
        MonoOutput::INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        MonoOutput::STATE_SIZE
    }

    fn set_update_type(&self, _update_types: &mut [UpdateType]) {
        /* do nothing */ 
    }
}
