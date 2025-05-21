use crate::control_interface::ControlInterface;
use crate::simulator::module::Module;
use crate::simulator::state::UpdateType;
// use alloc::collections::BTreeSet;
// use core::cmp::Ord;
// use core::cmp::Ordering;
use core::f32::consts::PI;

const PRESSURE_FILTER_CONSTANT: f32 = 2. * PI * 20.;
const PITCHWHEEL_FILTER_CONSTANT: f32 = 2. * PI * 20.;

/*
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
*/

pub struct MonoKeys {
    // pitch_output_index: usize,
    // gate_output_index: usize,
    // pressure_output_index: usize,
    // velocity_output_index: usize,
    // pitchwheel_output_index: usize,
    // active_notes: BTreeSet<ActiveNote>,
    // current_pressure: f32,
    // current_velocity: f32,
    // current_pitch_value: f32,
    // current_gate: f32,
    // pitch_wheel: f32,
}

impl MonoKeys {
    pub const STATE_SIZE: usize = 5;
    pub const INPUT_SIZE: usize = 0;

    // state/output
    pub const GATE_OUTPUT: usize = 0;
    pub const PITCH_OUTPUT: usize = 1;
    pub const PITCHWHEEL_OUTPUT: usize = 2;
    pub const VELOCITY_OUTPUT: usize = 3;
    pub const PRESSURE_OUTPUT: usize = 4;

    pub fn new(
        // pitch_output_index: usize,
        // gate_output_index: usize,
        // pressure_output_index: usize,
        // velocity_output_index: usize,
        // pitchwheel_output_index: usize,
    ) -> Self {
        Self {
            // pitch_output_index,
            // gate_output_index,
            // pressure_output_index,
            // velocity_output_index,
            // pitchwheel_output_index,
            // active_notes: BTreeSet::new(),
            // current_pressure: 0.,
            // current_velocity: 0.,
            // current_pitch_value: 0.,
            // current_gate: 0.,
            // pitch_wheel: 0.,
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
        update[MonoKeys::GATE_OUTPUT] = control_interface.get_gate();
        update[MonoKeys::PITCH_OUTPUT] = control_interface.get_pitch();
        update[MonoKeys::PITCHWHEEL_OUTPUT] = PITCHWHEEL_FILTER_CONSTANT * (control_interface.get_pitchwheel() - state[MonoKeys::PITCHWHEEL_OUTPUT]);
        update[MonoKeys::PRESSURE_OUTPUT] = PRESSURE_FILTER_CONSTANT * (control_interface.get_aftertouch() - state[MonoKeys::PRESSURE_OUTPUT]);
        update[MonoKeys::VELOCITY_OUTPUT] = control_interface.get_velocity();
    }

    fn finalize(&mut self, _inputs: &[f32], _state: &mut [f32], _outputs: &mut [f32], _dt: f32) {
        // println!("gate: {}", _state[MonoKeys::GATE_OUTPUT]);
        // println!("pitch: {}", _state[MonoKeys::PITCH_OUTPUT]);
        /* do nothing */
    }

    fn get_input_size(&self) -> usize {
        MonoKeys::INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        MonoKeys::STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        update_types[MonoKeys::GATE_OUTPUT] = UpdateType::Absolute;
        update_types[MonoKeys::PITCH_OUTPUT] = UpdateType::Absolute;
        update_types[MonoKeys::PITCHWHEEL_OUTPUT] = UpdateType::Differentiable;
        update_types[MonoKeys::VELOCITY_OUTPUT] = UpdateType::Absolute;
        update_types[MonoKeys::PRESSURE_OUTPUT] = UpdateType::Differentiable;
    }
}
