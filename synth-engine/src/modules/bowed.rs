use crate::event::ControllerEvent;
use crate::modules::input_expr::InputExpr;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use std::f32::consts::PI;

pub struct BowedOscillator {
    f0: f32,
    a: f32,
    b: f32,
    state_u_index: usize,
    state_v_index: usize,
    control_input: InputExpr,
    linear_control: InputExpr,
    pressure_input: InputExpr,
    velocity_input: InputExpr,
}

impl BowedOscillator {
    pub fn new(
        f0: f32,
        a: f32,
        b: f32,
        state_u_index: usize,
        state_v_index: usize,
        control_input: InputExpr,
        linear_control: InputExpr,
        pressure_input: InputExpr,
        velocity_input: InputExpr,
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

fn control_to_frequency(f0: f32, exp_control: f32, lin_control: f32) -> f32 {
    f0 * 2.0_f32.powf(exp_control) + lin_control
}

fn friction(a: f32, b: f32, x: f32) -> f32 {
    a * x * (-b * x * x + 0.5).exp()
}

fn sign(v: f32) -> f32 {
    if v < 0. {
        -1.
    } else {
        1.
    }
}

impl Module for BowedOscillator {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let linear_control = self.linear_control.from_state(state);

        let omega = control_to_frequency(
            self.f0,
            self.control_input.from_state(state),
            linear_control,
        ) * 2.0
            * PI;

        let u = state.get(self.state_u_index);
        let v = state.get(self.state_v_index);

        let vb = self.velocity_input.from_state(state).max(-1.).min(1.);
        let force = self.pressure_input.from_state(state).min(omega / 2.);

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

    fn finalize(&mut self, state: &mut State, _time_step: f32) {
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
