use crate::control_interface::ControlInterface;
use crate::simulator::module::Module;
use crate::simulator::state::UpdateType;
// use crate::stack_program::*;
use crate::synth_math::SynthMath;

pub struct Folder {
    // signal_input: StackProgram,
    // control_input: StackProgram,
    // output_index: usize,
}

impl Folder {
    pub const STATE_SIZE: usize = 1;
    pub const INPUT_SIZE: usize = 2;

    // state/outputs
    pub const SIGNAL_OUTPUT: usize = 0;

    // inputs/control
    pub const SIGNAL_INPUT: usize = 0;
    pub const CONTROL_INPUT: usize = 1;

    pub fn new(
        // signal_input: StackProgram,
        // control_input: StackProgram,
        // output_index: usize,
    ) -> Self {
        Self {
            // signal_input,
            // control_input,
            // output_index,
        }
    }
}

impl Module for Folder {
    fn simulate(
        &self,
        _control_interface: &ControlInterface,
        inputs: &[f32],
        _state: &[f32],
        update: &mut [f32],
        _dt: f32,
    ) {
        let i = inputs[Folder::SIGNAL_INPUT];
        let c = inputs[Folder::CONTROL_INPUT].clamp(0., 5.);

        update[Folder::SIGNAL_OUTPUT] = (i * c).sin();
    }

    fn finalize(&mut self, _inputs: &[f32], _state: &mut [f32], _outputs: &mut [f32], _dt: f32) {
        /* do nothing */
    }

    fn get_input_size(&self) -> usize {
        Folder::INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        Folder::STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        update_types[Folder::SIGNAL_OUTPUT] = UpdateType::Absolute;
    }
}
