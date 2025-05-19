use super::control_to_frequency;
use crate::control_interface::ControlInterface;
use crate::simulator::module::Module;
use crate::simulator::state::UpdateType;
// use crate::stack_program::*;
use crate::synth_math::SynthMath;

// NOTE the amplitude of this oscillator needs to be scaled up. It usually goes
// in the range of 0.008 to 0.016 peak-to-peak.

// The second output (state_v_index) has the sawtooth sound-a-like.

// This oscillator is different to the quadrature in that it _requires_ pressure
// and velocity to work.

// abs(velocity) should be about 0.1 to 0.2 (either positive or negative), but 0.0
// will produce no output.

// Force should be about 500 - 5000

// parameter `a` should be 100 to 1000 or thereabouts

pub struct BowedOscillator {
    f0: f32,
    a: f32,
    // state_u_index: usize,
    // state_v_index: usize,
    // control_input: StackProgram,
    // linear_control: StackProgram,
    // pressure_input: StackProgram,
    // velocity_input: StackProgram,
}

impl BowedOscillator {
    pub const STATE_SIZE: usize = 2;
    pub const INPUT_SIZE: usize = 4;

    // state/outputs
    pub const STATE_U: usize = 0;
    pub const STATE_V: usize = 1;

    // inputs/control
    pub const EXP_CONTROL_INPUT: usize = 0;
    pub const LINEAR_CONTROL_INPUT: usize = 1;
    pub const PRESSURE_CONTROL_INPUT: usize = 2;
    pub const VELOCITY_CONTROL_INPUT: usize = 3;

    pub fn new(
        f0: f32,
        a: f32,
        // state_u_index: usize,
        // state_v_index: usize,
        // control_input: StackProgram,
        // linear_control: StackProgram,
        // pressure_input: StackProgram,
        // velocity_input: StackProgram,
    ) -> Self {
        Self {
            f0,
            a,
            // state_u_index,
            // state_v_index,
            // control_input,
            // linear_control,
            // pressure_input,
            // velocity_input,
        }
    }
}

fn friction(a: f32, x: f32) -> f32 {
    (2. * a).sqrt() * x * (-2. * a * x * x + 0.5).exp()
}

#[allow(dead_code)]
fn discontinuous_friction(a: f32, x: f32) -> f32 {
    x.signum() * (-a * x.abs()).exp()
}

impl Module for BowedOscillator {
    fn simulate(
        &self,
        _control_interface: &ControlInterface,
        inputs: &[f32],
        state: &[f32],
        update: &mut [f32],
        _dt: f32,
    ) {
        let omega = control_to_frequency(
            self.f0,
            inputs[BowedOscillator::EXP_CONTROL_INPUT],
            inputs[BowedOscillator::LINEAR_CONTROL_INPUT],
        );

        let u = state[BowedOscillator::STATE_U];
        let v = state[BowedOscillator::STATE_V];

        let vb = inputs[BowedOscillator::VELOCITY_CONTROL_INPUT].max(-1.).min(1.);
        let force = inputs[BowedOscillator::PRESSURE_CONTROL_INPUT];

        let f = force * friction(self.a, u - vb);

        update[BowedOscillator::STATE_U] = -(omega * omega) * v - f;
        update[BowedOscillator::STATE_V] = u;
    }

    fn finalize(&mut self, _inputs: &[f32], _state: &mut [f32], _outputs: &mut [f32], _dt: f32) {
        /* do nothing */
    }

    fn get_input_size(&self) -> usize {
        BowedOscillator::INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        BowedOscillator::STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        update_types[BowedOscillator::STATE_U] = UpdateType::Differentiable;
        update_types[BowedOscillator::STATE_V] = UpdateType::Differentiable;
    }
}
