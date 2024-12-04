use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;

pub struct Modulator {
    signal_1_input: StackProgram,
    signal_2_input: StackProgram,
    output_index: usize,
}

impl Modulator {
    pub fn new(
        signal_1_input: StackProgram,
        signal_2_input: StackProgram,
        output_index: usize,
    ) -> Self {
        Self {
            signal_1_input,
            signal_2_input,
            output_index,
        }
    }
}

impl Module for Modulator {
    fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]) {
        let u = self.signal_1_input.run(state, stack).unwrap_or(0.);
        let v = self.signal_2_input.run(state, stack).unwrap_or(0.);

        update.set(self.output_index, u * v, UpdateType::Absolute);
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, _state: &mut State, _time_step: f32, _stack: &mut [f32]) {
        /* do nothing */
    }
}
