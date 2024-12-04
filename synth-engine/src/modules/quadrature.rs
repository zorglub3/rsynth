use super::control_to_frequency;
use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use std::f32::consts::PI;

pub struct QuadratureOscillator {
    f0: f32,
    state_x_index: usize,
    state_y_index: usize,
    control_index: usize,
}

impl QuadratureOscillator {
    pub fn new(f0: f32, state_x_index: usize, state_y_index: usize, control_index: usize) -> Self {
        Self {
            f0,
            state_x_index,
            state_y_index,
            control_index,
        }
    }
}

impl Module for QuadratureOscillator {
    fn simulate(&self, state: &State, update: &mut StateUpdate, _stack: &mut [f32]) {
        let omega = control_to_frequency(self.f0, state.get(self.control_index), 0.0) * (2. * PI);
        let x = state.get(self.state_x_index);
        let y = state.get(self.state_y_index);

        update.set(self.state_x_index, omega * y, UpdateType::Differentiable);
        update.set(self.state_y_index, -omega * x, UpdateType::Differentiable);
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State, _time_step: f32, _stack: &mut [f32]) {
        let x = state.get(self.state_x_index);
        let y = state.get(self.state_y_index);
        let s = (x * x + y * y).sqrt();

        if s < f32::EPSILON {
            state.set(self.state_x_index, 0.);
            state.set(self.state_y_index, 1.);
        } else {
            state.set(self.state_x_index, x / s);
            state.set(self.state_y_index, y / s);
        }
    }
}
