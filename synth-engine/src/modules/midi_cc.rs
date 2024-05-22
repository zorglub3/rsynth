use crate::midi::message::MidiMessage;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};

pub struct MidiCC {
    output_index: usize,
    control: u8,
    value: u8,
    channel: u8,
}

impl MidiCC {
    pub fn new(output_index: usize, control: u8, channel: u8) -> Self {
        Self {
            output_index,
            control,
            value: 0,
            channel,
        }
    }
}

impl Module for MidiCC {
    fn simulate(&self, _state: &State, update: &mut StateUpdate) {
        let v = (self.value as f32) / 127.0;
        update.set(self.output_index, v, UpdateType::Absolute);
    }

    fn process_event(&mut self, event: &MidiMessage, channel: u8) {
        if channel == self.channel {
            match event {
                MidiMessage::ContinuousControl { control, value } if *control == self.control => {
                    self.value = *value;
                }
                _ => { /* do nothing */ }
            }
        }
    }

    fn finalize(&mut self, _state: &mut State) {
        /* do nothing */
    }
}
