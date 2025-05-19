use super::control_to_frequency;
use crate::control_interface::ControlInterface;
use crate::distortion::*;
use crate::simulator::module::Module;
use crate::simulator::state::UpdateType;
// use crate::stack_program::*;
use core::f32::consts::PI;

pub struct Filter24db {
    f0: f32,
    // state0_index: usize,
    // state1_index: usize,
    // state2_index: usize,
    // state3_index: usize,
    // freq_control_input: StackProgram,
    // linear_control_input: StackProgram,
    // res_control_input: StackProgram,
    // signal_input: StackProgram,
    distortion: Option<DistortionType>,
}

impl Filter24db {
    pub const STATE_SIZE: usize = 4;
    pub const INPUT_SIZE: usize = 4;

    // state/outputs
    pub const STATE0: usize = 0; // NB: this is highpass output
    pub const STATE1: usize = 1;
    pub const STATE2: usize = 2;
    pub const STATE3: usize = 3; // NB: this is lowpass output, also

    // inputs/controls
    pub const EXP_CONTROL_INPUT: usize = 0;
    pub const LINEAR_CONTROL_INPUT: usize = 1;
    pub const RESONANCE_CONTROL_INPUT: usize = 2;
    pub const SIGNAL_INPUT: usize = 3;

    pub fn new(
        f0: f32,
        // state0_index: usize,
        // state1_index: usize,
        // state2_index: usize,
        // state3_index: usize,
        // freq_control_input: StackProgram,
        // linear_control_input: StackProgram,
        // res_control_input: StackProgram,
        // signal_input: StackProgram,
        distortion: Option<DistortionType>,
    ) -> Self {
        Self {
            f0,
            // state0_index,
            // state1_index,
            // state2_index,
            // state3_index,
            // freq_control_input,
            // linear_control_input,
            // res_control_input,
            // signal_input,
            distortion,
            // distortion: Some(DistortionType::Smoothstep),
        }
    }
}

impl Module for Filter24db {
    fn simulate(
        &self,
        _control_interface: &ControlInterface,
        inputs: &[f32],
        state: &[f32],
        update: &mut [f32],
        _dt: f32,
    ) {
        let f = control_to_frequency(
            self.f0,
            inputs[Filter24db::EXP_CONTROL_INPUT],
            inputs[Filter24db::LINEAR_CONTROL_INPUT],
        );
        let g = f * 2. * PI;
        let r = inputs[Filter24db::RESONANCE_CONTROL_INPUT].max(0.);
        let input = inputs[Filter24db::SIGNAL_INPUT];
        let d = &self.distortion;

        update[Filter24db::STATE0] =
            g * ((input - r * state[Filter24db::STATE3]).distort(d)
                 - state[Filter24db::STATE0]).distort(d);
        update[Filter24db::STATE1] =
            g * (state[Filter24db::STATE0].distort(d) - state[Filter24db::STATE1].distort(d));
        update[Filter24db::STATE2] =
            g * (state[Filter24db::STATE1].distort(d) - state[Filter24db::STATE2].distort(d));
        update[Filter24db::STATE3] =
            g * (state[Filter24db::STATE2].distort(d) - state[Filter24db::STATE3].distort(d));
    }

    fn finalize(&mut self, _inputs: &[f32], _state: &mut [f32], _outputs: &mut [f32], _dt: f32) {
        /* do nothing */
    }

    fn get_input_size(&self) -> usize {
        Filter24db::INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        Filter24db::STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        update_types[Filter24db::STATE0] = UpdateType::Differentiable;
        update_types[Filter24db::STATE1] = UpdateType::Differentiable;
        update_types[Filter24db::STATE2] = UpdateType::Differentiable;
        update_types[Filter24db::STATE3] = UpdateType::Differentiable;
    }
}
