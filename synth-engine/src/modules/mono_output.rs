use crate::control_interface::ControlInterface;
use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;

pub const STATE_SIZE: usize = 0;
pub const INPUT_SIZE: usize = 1;

// inputs/controls
const SIGNAL_INPUT: usize = 0;

pub struct MonoOutput {
    output_index: usize,
    signal_input: StackProgram,
}

impl MonoOutput {
    pub fn new(output_index: usize, signal_input: StackProgram) -> Self {
        Self {
            output_index,
            signal_input,
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
        outputs[self.output_index] = inputs[SIGNAL_INPUT];
    }

    fn get_input_size(&self) -> usize {
        INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        /* do nothing */ 
    }

    /*
    fn simulate(&self, _state: &State, _update: &mut StateUpdate, _stack: &mut [f32]) {
        /* do nothing */
    }

    fn process_event(&mut self, _even: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State, _time_step: f32, stack: &mut [f32]) {
        let v = self.signal_input.run(state, stack).unwrap_or(0.);
        state.set_output(self.output_index, v);
    }
    */
}
