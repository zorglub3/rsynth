use crate::midi::message::MidiMessage;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use std::collections::BTreeSet;

pub struct MidiMono {
    pitch_output_index: usize,
    gate_output_index: usize,
    pressure_output_index: usize,
    velocity_output_index: usize,
    channel: u8,
    keys_pressed: BTreeSet<u8>,
    current_pressure: u8,
    current_velocity: u8,
}

impl MidiMono {
    pub fn new(
        pitch_output_index: usize,
        gate_output_index: usize,
        pressure_output_index: usize,
        velocity_output_index: usize,
        channel: u8,
    ) -> Self {
        Self {
            pitch_output_index,
            gate_output_index,
            pressure_output_index,
            velocity_output_index,
            channel,
            keys_pressed: BTreeSet::new(),
            current_pressure: 0,
            current_velocity: 0,
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
                let pv = pitch_value(*pitch);

                update.set(self.gate_output_index, 1., UpdateType::Absolute);
                update.set(self.pitch_output_index, pv, UpdateType::Absolute);
            }
            None => {
                update.set(self.gate_output_index, 0., UpdateType::Absolute);
            }
        }

        update.set(
            self.pressure_output_index,
            (self.current_pressure as f32) / 127.,
            UpdateType::Absolute,
        );
        update.set(
            self.velocity_output_index,
            (self.current_velocity as f32) / 127.,
            UpdateType::Absolute,
        );
    }

    fn process_event(&mut self, event: &MidiMessage, channel: u8) {
        if channel == self.channel {
            match event {
                MidiMessage::NoteOn { pitch, velocity } => {
                    self.keys_pressed.insert(*pitch);
                    self.current_velocity = *velocity;
                }
                MidiMessage::NoteOff { pitch, .. } => {
                    self.keys_pressed.remove(pitch);
                }
                MidiMessage::ChannelAftertouch { amount } => {
                    self.current_pressure = *amount;
                }
                _ => { /* do nothing */ }
            }
        }
    }

    fn finalize(&mut self, _state: &mut State) {
        /* do nothing */
    }
}
