use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use core::f32::consts::PI;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::BTreeSet;

const PRESSURE_FILTER_CONSTANT: f32 = 2. * PI * 20.;

// TODO
// - rename file and struct (not MIDI)
// - make highest/lowest priority selectable at creation
struct ActiveNote {
    pitch_code: u8,
    pitch_value: f32,
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
    current_pitch_value: f32,
    current_gate: f32,
    pitch_wheel: f32,
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
            current_pitch_value: 0.,
            current_gate: 0.,
            pitch_wheel: 0.,
        }
    }
}

impl Module for MidiMono {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        update.set(
            self.gate_output_index,
            self.current_gate,
            UpdateType::Absolute,
        );
        update.set(
            self.pitch_output_index,
            self.current_pitch_value + self.pitch_wheel,
            UpdateType::Absolute,
        );
        update.set(
            self.pressure_output_index,
            PRESSURE_FILTER_CONSTANT
                * (self.current_pressure - state.get(self.pressure_output_index)),
            UpdateType::Differentiable,
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
                });
            }
            NoteOff { pitch, .. } => {
                self.active_notes.remove(&ActiveNote {
                    pitch_code: *pitch,
                    pitch_value: 0.,
                });
            }
            Aftertouch { amount } => self.current_pressure = *amount,
            PitchWheel { amount } => self.pitch_wheel = *amount,
            _ => {} // do nothing
        }

        match self.active_notes.first() {
            Some(ActiveNote { pitch_value, .. }) => {
                self.current_gate = 1.;
                self.current_pitch_value = *pitch_value;
            }
            None => {
                self.current_gate = 0.;
            }
        }
    }

    fn finalize(&mut self, _state: &mut State, _time_step: f32) {
        /* do nothing */
    }
}
