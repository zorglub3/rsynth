use super::control_to_frequency;
use crate::control_interface::ControlInterface;
use crate::simulator::module::Module;
use crate::simulator::state::UpdateType;
// use crate::stack_program::*;
use crate::synth_math::SynthMath;
use crate::wavetable_entry::WavetableEntry;
use alloc::vec::Vec;
use core::f32::consts::PI;

pub const FREQUENCY_LIMIT: f32 = 18_000.0;

pub struct Wavetable {
    f0: f32,
    // position_state: usize,
    // signal_output: usize,
    // pitch_control: StackProgram,
    // linear_modulation: StackProgram,
    // wavetable_select: StackProgram,
    wavetables: Vec<WavetableEntry>,
    amp: f32,
}

impl Wavetable {
    pub const STATE_SIZE: usize = 2;
    pub const INPUT_SIZE: usize = 3;

    // state/outputs
    pub const POSITION_STATE: usize = 0;
    pub const SIGNAL_OUTPUT: usize = 1;

    // inputs/control
    pub const EXP_CONTROL_INPUT: usize = 0;
    pub const LINEAR_CONTROL_INPUT: usize = 1;
    pub const WAVETABLE_SELECT_INPUT: usize = 2;

    pub fn new(
        f0: f32,
        // position_state: usize,
        // signal_output: usize,
        // pitch_control: StackProgram,
        // linear_modulation: StackProgram,
        // wavetable_select: StackProgram,
        wavetables: Vec<Vec<f32>>,
    ) -> Self {
        Self {
            f0,
            // position_state,
            // signal_output,
            // pitch_control,
            // linear_modulation,
            // wavetable_select,
            wavetables: wavetables
                .into_iter()
                .map(|samples| WavetableEntry::from_slice(&samples))
                .collect(),
            amp: 2. * PI * FREQUENCY_LIMIT,
        }
    }

    pub fn precompute_wavetables(table_data: &Vec<Vec<f32>>) -> Vec<WavetableEntry> {
        table_data
            .into_iter()
            .map(|samples| WavetableEntry::from_slice(&samples))
            .collect()
    }

    pub fn new_with_precompute(
        f0: f32,
        // position_state: usize,
        // signal_output: usize,
        // pitch_control: StackProgram,
        // linear_modulation: StackProgram,
        // wavetable_select: StackProgram,
        wavetables: Vec<WavetableEntry>,
    ) -> Self {
        Self {
            f0,
            // position_state,
            // signal_output,
            // pitch_control,
            // linear_modulation,
            // wavetable_select,
            wavetables,
            amp: 2. * PI * FREQUENCY_LIMIT,
        }
    }
}

impl Module for Wavetable {
    fn simulate(
        &self,
        _control_interface: &ControlInterface,
        inputs: &[f32],
        state: &[f32],
        update: &mut [f32],
        dt: f32,
    ) {
        let velocity = control_to_frequency(
            self.f0,
            inputs[Wavetable::EXP_CONTROL_INPUT],
            inputs[Wavetable::LINEAR_CONTROL_INPUT],
        );
        let position = state[Wavetable::POSITION_STATE];
        let distance = dt * velocity;

        let sample = if self.wavetables.len() == 1 {
            self.wavetables[0].eval(distance, position)
        } else if self.wavetables.len() > 1 {
            let scan = inputs[Wavetable::WAVETABLE_SELECT_INPUT].clamp(0., 1.);
            let scan_select = scan * ((self.wavetables.len() - 1) as f32);
            let index = scan_select.floor() as usize;
            let x = scan_select.fract();
            let index0 = index.min(self.wavetables.len() - 1);
            let index1 = (index + 1).min(self.wavetables.len() - 1);

            let v1 = self.wavetables[index0].eval(distance, position);
            let v2 = self.wavetables[index1].eval(distance, position);

            v1 + (v2 - v1) * x
        } else {
            0.
        };

        update[Wavetable::SIGNAL_OUTPUT] = self.amp * (sample - state[Wavetable::SIGNAL_OUTPUT]);
        update[Wavetable::POSITION_STATE] = velocity;
    }

    fn finalize(&mut self, _inputs: &[f32], state: &mut [f32], _outputs: &mut [f32], _dt: f32) {
        let p = ((state[Wavetable::POSITION_STATE] % 1.) + 1.) % 1.;

        state[Wavetable::POSITION_STATE] = p;
    }

    fn get_input_size(&self) -> usize {
        Wavetable::INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        Wavetable::STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        update_types[Wavetable::POSITION_STATE] = UpdateType::Differentiable;
        update_types[Wavetable::SIGNAL_OUTPUT] = UpdateType::Differentiable;
    }
}
