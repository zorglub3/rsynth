use crate::midi::message::MidiMessage;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};

pub struct Folder {
    input_index: usize,
    control_index: usize,
    output_index: usize,
}

impl Folder {
    pub fn new(input_index: usize, control_index: usize, output_index: usize) -> Self {
        Self {
            input_index,
            control_index,
            output_index,
        }
    }
}

impl Module for Folder {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let i = state.get(self.input_index);
        let c = state.get(self.control_index).max(0.).min(5.) + 1.;

        update.set(self.output_index, (i * c).sin(), UpdateType::Absolute);
    }

    fn process_event(&mut self, _event: &MidiMessage, _channel: u8) {
        /* do nothing */
    }

    fn finalize(&mut self, _state: &mut State) {
        /* do nothing */
    }
}
