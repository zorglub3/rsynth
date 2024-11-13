use crate::event::ControllerEvent;
use crate::modules::input_expr::InputExpr;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use std::f32::consts::PI;

pub struct Filter6db {
    f0: f32,
    signal_output: usize,
    freq_control_input: InputExpr,
    signal_input: InputExpr,
}

impl Filter6db {
    pub fn new(
        f0: f32,
        signal_output: usize,
        freq_control_input: InputExpr,
        signal_input: InputExpr,
    ) -> Self {
        Self {
            f0,
            signal_output,
            freq_control_input,
            signal_input,
        }
    }
}

impl Module for Filter6db {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let a = self.f0 * 2.0_f32.powf(self.freq_control_input.from_state(state));
        let b = 1. - (-2. * a * PI).exp();
        let c = (1. - b).max(0.0001);

        update.set(
            self.signal_output,
            self.signal_input.from_state(state) * b - state.get(self.signal_output) * c,
            UpdateType::Differentiable,
        );
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, _state: &mut State, _time_step: f32) {
        /* do nothing */
    }
}
