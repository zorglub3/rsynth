use super::control_to_frequency;
use crate::event::ControllerEvent;
use crate::modules::input_expr::InputExpr;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use std::f32::consts::PI;

pub struct Filter6db {
    f0: f32,
    lowpass_output: usize,
    highpass_output: usize,
    freq_control_input: InputExpr,
    linear_control: InputExpr,
    signal_input: InputExpr,
}

impl Filter6db {
    pub fn new(
        f0: f32,
        lowpass_output: usize,
        highpass_output: usize,
        freq_control_input: InputExpr,
        linear_control: InputExpr,
        signal_input: InputExpr,
    ) -> Self {
        Self {
            f0,
            lowpass_output,
            highpass_output,
            freq_control_input,
            linear_control,
            signal_input,
        }
    }
}

impl Module for Filter6db {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let a = control_to_frequency(
            self.f0,
            self.freq_control_input.from_state(state),
            self.linear_control.from_state(state),
        );
        let b = 1. - (-2. * a * PI).exp();
        let c = (1. - b).max(0.0001);

        update.set(
            self.lowpass_output,
            self.signal_input.from_state(state) * b - state.get(self.lowpass_output) * c,
            UpdateType::Differentiable,
        );
        update.set(
            self.highpass_output,
            self.signal_input.from_state(state) - state.get(self.lowpass_output),
            UpdateType::Absolute,
        );
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, _state: &mut State, _time_step: f32) {
        /* do nothing */
    }
}
