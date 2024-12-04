use super::control_to_frequency;
use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;
use std::f32::consts::PI;

pub struct BowedOscillator {
    f0: f32,
    a: f32,
    b: f32,
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
        b: f32,
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
            b,
            state_u_index,
            state_v_index,
            control_input,
            linear_control,
            pressure_input,
            velocity_input,
        }
    }
}

fn friction(a: f32, b: f32, x: f32) -> f32 {
    a * x * (-b * x * x + 0.5).exp()
}

impl Module for BowedOscillator {
    fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]) {
        let linear_control = self.linear_control.run(state, stack).unwrap_or(0.);
        // let linear_control = self.linear_control.from_state(state);

        let omega = control_to_frequency(
            self.f0,
            self.control_input.run(state, stack).unwrap_or(0.),
            // self.control_input.from_state(state),
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
        let force = self
            .pressure_input
            .run(state, stack)
            .unwrap_or(0.)
            .min(omega / 2.);

        let f = force * friction(self.a, self.b, v - vb);

        update.set(self.state_u_index, -omega * v, UpdateType::Differentiable);
        update.set(
            self.state_v_index,
            omega * u - f,
            UpdateType::Differentiable,
        );
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State, _time_step: f32, _stack: &mut [f32]) {
        let u = state.get(self.state_u_index);
        let v = state.get(self.state_v_index);
        let s = (u * u + v * v).sqrt();

        if s < f32::EPSILON {
            state.set(self.state_u_index, 1.);
            state.set(self.state_v_index, 0.);
        } else {
            state.set(self.state_u_index, u / s);
            state.set(self.state_v_index, v / s);
        }
    }
}
