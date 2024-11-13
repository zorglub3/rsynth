use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::BTreeSet;

struct ActiveNote {
    pitch_code: u8,
    pitch_value: f32,
    velocity: f32,
}

impl PartialEq for ActiveNote {
    fn eq(&self, other: &Self) -> bool {
        self.pitch_code == other.pitch_code
    }

    fn ne(&self, other: &Self) -> bool {
        self.pitch_code != other.pitch_code
    }
}

impl Eq for ActiveNote {}

impl PartialOrd for ActiveNote {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ActiveNote {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pitch_code.cmp(&other.pitch_code)
    }
}

pub struct MidiMono {
    pitch_output_index: usize,
    gate_output_index: usize,
    pressure_output_index: usize,
    velocity_output_index: usize,
    active_notes: BTreeSet<ActiveNote>,
    current_pressure: f32,
    current_velocity: f32,
}

impl MidiMono {
    pub fn new(
        pitch_output_index: usize,
        gate_output_index: usize,
        pressure_output_index: usize,
        velocity_output_index: usize,
    ) -> Self {
        Self {
            pitch_output_index,
            gate_output_index,
            pressure_output_index,
            velocity_output_index,
            active_notes: BTreeSet::new(),
            current_pressure: 0.,
            current_velocity: 0.,
        }
    }
}

impl Module for MidiMono {
    fn simulate(&self, _state: &State, update: &mut StateUpdate) {
        match self.active_notes.first() {
            Some(ActiveNote { pitch_value, .. }) => {
                update.set(self.gate_output_index, 1., UpdateType::Absolute);
                update.set(self.pitch_output_index, *pitch_value, UpdateType::Absolute);
            }
            None => {
                update.set(self.gate_output_index, 0., UpdateType::Absolute);
            }
        }

        update.set(
            self.pressure_output_index,
            self.current_pressure,
            UpdateType::Absolute,
        );
        update.set(
            self.velocity_output_index,
            self.current_velocity,
            UpdateType::Absolute,
        );
    }

    fn process_event(&mut self, event: &ControllerEvent) {
        use ControllerEvent::*;

        match event {
            NoteOn {
                pitch,
                velocity,
                pitch_value,
            } => {
                self.current_velocity = *velocity;
                self.active_notes.insert(ActiveNote {
                    pitch_code: *pitch,
                    pitch_value: *pitch_value,
                    velocity: *velocity,
                });
            }
            NoteOff { pitch, velocity } => {
                self.active_notes.remove(&ActiveNote {
                    pitch_code: *pitch,
                    velocity: *velocity,
                    pitch_value: 0.,
                });
            }
            Aftertouch { amount } => self.current_pressure = *amount,
            PitchWheel { amount } => { /* do thing - TODO */ }
            _ => {} // do nothing
        }
        /*
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
        */
    }

    fn finalize(&mut self, _state: &mut State, _time_step: f32) {
        /* do nothing */
    }
}
