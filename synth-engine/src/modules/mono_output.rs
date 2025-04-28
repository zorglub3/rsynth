use crate::control_interface::ControlInterface;
use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;

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
        control_interface: &ControlInterface,
        inputs: &[f32],
        state: &mut [f32],
        dt: f32,
    ) {
        todo!()
    }

    fn finalize(&mut self, state: &mut [f32], outputs: &mut [f32], dt: f32) {
        todo!()
    }

    fn get_input_size(&self) -> usize {
        todo!()
    }

    fn get_state_size(&self) -> usize {
        todo!()
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        todo!()
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
