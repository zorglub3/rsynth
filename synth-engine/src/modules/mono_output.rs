use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate};
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
}
