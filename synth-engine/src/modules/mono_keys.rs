use crate::control_interface::ControlInterface;
use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use alloc::collections::BTreeSet;
use core::cmp::Ord;
use core::cmp::Ordering;
use core::f32::consts::PI;

pub const STATE_SIZE: usize = 5;
pub const INPUT_SIZE: usize = 0;

// state/output
const GATE_OUTPUT: usize = 0;
const PITCH_OUTPUT: usize = 1;
const PITCHWHEEL_OUTPUT: usize = 2;
const VELOCITY_OUTPUT: usize = 3;
const PRESSURE_OUTPUT: usize = 4;

const PRESSURE_FILTER_CONSTANT: f32 = 2. * PI * 20.;
const PITCHWHEEL_FILTER_CONSTANT: f32 = 2. * PI * 20.;

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

pub struct MonoKeys {
    pitch_output_index: usize,
    gate_output_index: usize,
    pressure_output_index: usize,
    velocity_output_index: usize,
    pitchwheel_output_index: usize,
    active_notes: BTreeSet<ActiveNote>,
    current_pressure: f32,
    current_velocity: f32,
    current_pitch_value: f32,
    current_gate: f32,
    pitch_wheel: f32,
}

impl MonoKeys {
    pub fn new(
        pitch_output_index: usize,
        gate_output_index: usize,
        pressure_output_index: usize,
        velocity_output_index: usize,
        pitchwheel_output_index: usize,
    ) -> Self {
        Self {
            pitch_output_index,
            gate_output_index,
            pressure_output_index,
            velocity_output_index,
            pitchwheel_output_index,
            active_notes: BTreeSet::new(),
            current_pressure: 0.,
            current_velocity: 0.,
            current_pitch_value: 0.,
            current_gate: 0.,
            pitch_wheel: 0.,
        }
    }
}

impl Module for MonoKeys {
    fn simulate(
        &self,
        control_interface: &ControlInterface,
        _inputs: &[f32],
        state: &[f32],
        update: &mut [f32],
        _dt: f32,
    ) {
        update[GATE_OUTPUT] = control_interface.get_gate();
        update[PITCH_OUTPUT] = control_interface.get_pitch();
        update[PITCHWHEEL_OUTPUT] = PITCHWHEEL_FILTER_CONSTANT * (control_interface.get_pitchwheel() - state[PITCHWHEEL_OUTPUT]);
        update[PRESSURE_OUTPUT] = PRESSURE_FILTER_CONSTANT * (control_interface.get_aftertouch() - state[PRESSURE_OUTPUT]);
        update[VELOCITY_OUTPUT] = control_interface.get_velocity();
    }

    fn finalize(&mut self, _inputs: &[f32], state: &mut [f32], outputs: &mut [f32], dt: f32) {
        /* do nothing */
    }

    fn get_input_size(&self) -> usize {
        INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        update_types[GATE_OUTPUT] = UpdateType::Absolute;
        update_types[PITCH_OUTPUT] = UpdateType::Absolute;
        update_types[PITCHWHEEL_OUTPUT] = UpdateType::Differentiable;
        update_types[VELOCITY_OUTPUT] = UpdateType::Absolute;
        update_types[PRESSURE_OUTPUT] = UpdateType::Differentiable;
    }

    /*
        fn simulate(&self, state: &State, update: &mut StateUpdate, _stack: &mut [f32]) {
            update.set(
                self.gate_output_index,
                self.current_gate,
                UpdateType::Absolute,
            );
            update.set(
                self.pitchwheel_output_index,
                PITCHWHEEL_FILTER_CONSTANT
                    * (self.pitch_wheel - state.get(self.pitchwheel_output_index)),
                UpdateType::Differentiable,
            );
            update.set(
                self.pitch_output_index,
                self.current_pitch_value + state.get(self.pitchwheel_output_index),
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

        fn finalize(&mut self, _state: &mut State, _time_step: f32, _stack: &mut [f32]) {
            /* do nothing */
        }
    */
}
