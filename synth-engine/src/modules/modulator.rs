use crate::midi::message::MidiMessage;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};

pub struct Modulator {
    input1_index: usize,
    input2_index: usize,
    output_index: usize,
}

impl Modulator {
    pub fn new(input1_index: usize, input2_index: usize, output_index: usize) -> Self {
        Self {
            input1_index,
            input2_index,
            output_index,
        }
    }
}

impl Module for Modulator {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        update.set(
            self.output_index,
            state.get(self.input1_index) * state.get(self.input2_index),
            UpdateType::Absolute,
        );
    }

    fn process_event(&mut self, _event: &MidiMessage, _channel: u8) {
        /* do nothing */
    }

    fn finalize(&mut self, _state: &mut State) {
        /* do nothing */
    }
}
