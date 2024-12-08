use super::control_to_frequency;
use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;
use core::f32::consts::PI;

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
    state_u_index: usize,
    state_v_index: usize,
    control_input: StackProgram,
    linear_control: StackProgram,
    pressure_input: StackProgram,
    velocity_input: StackProgram,
}

impl BowedOscillator {
    pub fn new(
        f0: f32,
        a: f32,
        state_u_index: usize,
        state_v_index: usize,
        control_input: StackProgram,
        linear_control: StackProgram,
        pressure_input: StackProgram,
        velocity_input: StackProgram,
    ) -> Self {
        Self {
            f0,
            a,
            state_u_index,
            state_v_index,
            control_input,
            linear_control,
            pressure_input,
            velocity_input,
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
    fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]) {
        let linear_control = self.linear_control.run(state, stack).unwrap_or(0.);

        let omega = control_to_frequency(
            self.f0,
            self.control_input.run(state, stack).unwrap_or(0.),
            linear_control,
        ) * 2.0
            * PI;

        let u = state.get(self.state_u_index);
        let v = state.get(self.state_v_index);

        let vb = self
            .velocity_input
            .run(state, stack)
            .unwrap_or(0.)
            .max(-1.)
            .min(1.);
        let force = self.pressure_input.run(state, stack).unwrap_or(0.);

        let f = force * friction(self.a, u - vb);

        update.set(
            self.state_u_index,
            -(omega * omega) * v - f,
            UpdateType::Differentiable,
        );
        update.set(self.state_v_index, u, UpdateType::Differentiable);
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, _state: &mut State, _time_step: f32, _stack: &mut [f32]) {
        /* do nothing */
    }
}
