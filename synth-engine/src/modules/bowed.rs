use crate::midi::message::MidiMessage;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use std::f32::consts::PI;

pub struct BowedOscillator {
    f0: f32,
    a: f32,
    state_u_index: usize,
    state_v_index: usize,
    control_index: usize,
    pressure_index: usize,
    velocity_index: usize,
}

impl BowedOscillator {
    pub fn new(
        f0: f32,
        a: f32,
        state_u_index: usize,
        state_v_index: usize,
        control_index: usize,
        pressure_index: usize,
        velocity_index: usize,
    ) -> Self {
        Self {
            f0,
            a,
            state_u_index,
            state_v_index,
            control_index,
            pressure_index,
            velocity_index,
        }
    }
}

fn control_to_frequency(f0: f32, exp_control: f32, lin_control: f32) -> f32 {
    f0 * 2.0_f32.powf(exp_control) + lin_control
}

fn friction(a: f32, x: f32) -> f32 {
    (2.0_f32 * a).sqrt() * x * (-a * x * x + 0.5).exp()
}

impl Module for BowedOscillator {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let omega = control_to_frequency(self.f0, state.get(self.control_index), 0.0) * (2. * PI);
        let u = state.get(self.state_u_index);
        let v = state.get(self.state_v_index);

        // let vb = state.get(self.velocity_index);
        // let force = state.get(self.pressure_index);
        let vb = 0.3;
        let force: f32 = 10.;

        let force: f32 = force.min(omega / 2.);

        let f = force * friction(self.a, v - vb);

        update.set(self.state_u_index, -omega * v, UpdateType::Differentiable);
        update.set(
            self.state_v_index,
            omega * u - f,
            UpdateType::Differentiable,
        );
    }

    fn process_event(&mut self, _event: &MidiMessage, _channel: u8) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State) {
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
