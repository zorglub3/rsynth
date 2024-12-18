use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate};
use crate::stack_program::*;

pub struct MonoOutput<'a> {
    output_index: usize,
    signal_input: StackProgram<'a>,
}

impl<'a> MonoOutput<'a> {
    pub fn new(output_index: usize, signal_input: StackProgram<'a>) -> Self {
        Self {
            output_index,
            signal_input,
        }
    }
}

impl<'a> Module for MonoOutput<'a> {
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
