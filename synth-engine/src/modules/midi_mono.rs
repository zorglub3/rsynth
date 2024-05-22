use crate::midi::message::MidiMessage;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use std::collections::BTreeSet;

pub struct MidiMono {
    pitch_output_index: usize,
    gate_output_index: usize,
    channel: u8,
    keys_pressed: BTreeSet<u8>,
}

impl MidiMono {
    pub fn new(pitch_output_index: usize, gate_output_index: usize, channel: u8) -> Self {
        Self {
            pitch_output_index,
            gate_output_index,
            channel,
            keys_pressed: BTreeSet::new(),
        }
    }
}

fn pitch_value(pitch: u8) -> f32 {
    (pitch as f32) / 12.
}

impl Module for MidiMono {
    fn simulate(&self, _state: &State, update: &mut StateUpdate) {
        match self.keys_pressed.first() {
            Some(pitch) => {
                update.set(self.gate_output_index, 1., UpdateType::Absolute);
                update.set(
                    self.pitch_output_index,
                    pitch_value(*pitch),
                    UpdateType::Absolute,
                );
            }
            None => {
                update.set(self.gate_output_index, 0., UpdateType::Absolute);
            }
        }
    }

    fn process_event(&mut self, event: &MidiMessage, channel: u8) {
        if channel == self.channel {
            match event {
                MidiMessage::NoteOn { pitch, .. } => {
                    self.keys_pressed.insert(*pitch);
                }
                MidiMessage::NoteOff { pitch, .. } => {
                    self.keys_pressed.remove(pitch);
                }
                _ => { /* do nothing */ }
            }
        }
    }

    fn finalize(&mut self, _state: &mut State) {
        /* do nothing */
    }
}
