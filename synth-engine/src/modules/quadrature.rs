use super::control_to_frequency;
use crate::control_interface::ControlInterface;
use crate::simulator::module::Module;
use crate::simulator::state::UpdateType;
// use crate::stack_program::*;
use crate::synth_math::SynthMath;
use core::f32::consts::PI;

// TODO add to synth designer
pub struct QuadratureOscillator {
    f0: f32,
    // state_x_index: usize,
    // state_y_index: usize,
    // control_input: StackProgram,
    // linear_control: StackProgram,
}

impl QuadratureOscillator {
    pub const STATE_SIZE: usize = 2;
    pub const INPUT_SIZE: usize = 2;

    // state/outputs
    pub const STATE_X: usize = 0;
    pub const STATE_Y: usize = 1;

    // inputs/controls
    pub const EXP_CONTROL_INPUT: usize = 0;
    pub const LINEAR_CONTROL_INPUT: usize = 1;

    pub fn new(
        f0: f32,
        // state_x_index: usize,
        // state_y_index: usize,
        // control_input: StackProgram,
        // linear_control: StackProgram,
    ) -> Self {
        Self {
            f0,
            // state_x_index,
            // state_y_index,
            // control_input,
            // linear_control,
        }
    }
}

impl Module for QuadratureOscillator {
    fn simulate(
        &self,
        _control_interface: &ControlInterface,
        inputs: &[f32],
        state: &[f32],
        update: &mut [f32],
        _dt: f32,
    ) {
        let omega = 2. * PI * control_to_frequency(
            self.f0,
            inputs[QuadratureOscillator::EXP_CONTROL_INPUT],
            inputs[QuadratureOscillator::LINEAR_CONTROL_INPUT],
        );

        let x = state[QuadratureOscillator::STATE_X];
        let y = state[QuadratureOscillator::STATE_Y];

        update[QuadratureOscillator::STATE_X] = omega * y;
        update[QuadratureOscillator::STATE_Y] = -omega * x;
    }

    fn finalize(&mut self, _inputs: &[f32], state: &mut [f32], _outputs: &mut [f32], _dt: f32) {
        let x = state[QuadratureOscillator::STATE_X];
        let y = state[QuadratureOscillator::STATE_Y];
        let s = (x * x + y * y).sqrt();

        if s < f32::EPSILON {
            state[QuadratureOscillator::STATE_X] = 0.;
            state[QuadratureOscillator::STATE_Y] = 1.;
        } else {
            state[QuadratureOscillator::STATE_X] /= s;
            state[QuadratureOscillator::STATE_Y] /= s;
        }
    }

    fn get_input_size(&self) -> usize {
        QuadratureOscillator::INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        QuadratureOscillator::STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        update_types[QuadratureOscillator::STATE_X] = UpdateType::Differentiable;
        update_types[QuadratureOscillator::STATE_Y] = UpdateType::Differentiable;
    }
}
