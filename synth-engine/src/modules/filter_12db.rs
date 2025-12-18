use super::control_to_frequency;
use crate::control_interface::ControlInterface;
use crate::distortion::*;
use crate::simulator::module::Module;
use crate::simulator::state::UpdateType;
// use crate::stack_program::*;

const CLAMP_VALUE: f32 = 2.5;

pub struct Filter12db {
    f0: f32,
    // state_hp: usize,
    // state_bp: usize,
    // state_lp: usize,
    // freq_control_input: StackProgram,
    // linear_control_input: StackProgram,
    // res_control_input: StackProgram,
    // signal_input: StackProgram,
    distortion: Option<DistortionType>,
}

impl Filter12db {
    pub const STATE_SIZE: usize = 3;
    pub const INPUT_SIZE: usize = 4;

    // state/outputs
    pub const STATE_HP: usize = 0;
    pub const STATE_BP: usize = 1;
    pub const STATE_LP: usize = 2;

    // inputs/controls
    pub const EXP_CONTROL_INPUT: usize = 0;
    pub const LINEAR_CONTROL_INPUT: usize = 1;
    pub const RESONANCE_CONTROL_INPUT: usize = 2;
    pub const SIGNAL_INPUT: usize = 3;

    pub fn new(
        f0: f32,
        // state_hp: usize,
        // state_bp: usize,
        // state_lp: usize,
        // freq_control_input: StackProgram,
        // linear_control_input: StackProgram,
        // res_control_input: StackProgram,
        // signal_input: StackProgram,
        distortion: Option<DistortionType>,
    ) -> Self {
        Self {
            f0,
            // state_hp,
            // state_bp,
            // state_lp,
            // freq_control_input,
            // linear_control_input,
            // res_control_input,
            // signal_input,
            distortion,
            // distortion: None, // Some(DistortionType::Smoothstep), //Some(make_diodelike(1000., 2.)),
        }
    }
}

impl Module for Filter12db {
    fn simulate(
        &self,
        _control_interface: &ControlInterface,
        inputs: &[f32],
        state: &[f32],
        update: &mut [f32],
        _dt: f32,
    ) {
        let a = control_to_frequency(
            self.f0,
            inputs[Filter12db::EXP_CONTROL_INPUT],
            inputs[Filter12db::LINEAR_CONTROL_INPUT],
        );
        let b = 1. / inputs[Filter12db::RESONANCE_CONTROL_INPUT].max(0.01);

        let input = inputs[Filter12db::SIGNAL_INPUT];
        let bp_value = state[Filter12db::STATE_BP];
        let lp_value = state[Filter12db::STATE_LP];
        let hp_value = input - (bp_value * b) - lp_value;

        // let bp_value = bp_value;

        update[Filter12db::STATE_HP] = hp_value;
        update[Filter12db::STATE_BP] = a * hp_value;
        update[Filter12db::STATE_LP] = a * bp_value;
    }

    fn finalize(&mut self, _inputs: &[f32], state: &mut [f32], _outputs: &mut [f32], _dt: f32) {
        let bp = state[Filter12db::STATE_BP].clamp(-CLAMP_VALUE, CLAMP_VALUE);
        let lp = state[Filter12db::STATE_LP].clamp(-CLAMP_VALUE, CLAMP_VALUE);

        state[Filter12db::STATE_BP] = bp;
        state[Filter12db::STATE_LP] = lp;
    }

    fn get_input_size(&self) -> usize {
        Filter12db::INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        Filter12db::STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        update_types[Filter12db::STATE_HP] = UpdateType::Absolute;
        update_types[Filter12db::STATE_BP] = UpdateType::Differentiable;
        update_types[Filter12db::STATE_LP] = UpdateType::Differentiable;
    }
}
