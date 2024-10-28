use crate::midi::message::MidiMessage;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate};

pub struct MonoOutput {
    output_index: usize, 
    state_index: usize,
}

impl MonoOutput {
    pub fn new(
        output_index: usize,
        state_index: usize,
    ) -> Self {
        Self {
            output_index,
            state_index,
        }
    }
}

impl Module for MonoOutput {
    fn simulate(&self, _state: &State, _update: &mut StateUpdate) {
        /* do nothing */
    }

    fn process_event(&mut self, _even: &MidiMessage, _channel: u8) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State) {
        let v = state.get(self.state_index);
        state.set_output(self.output_index, v);
    }
}
