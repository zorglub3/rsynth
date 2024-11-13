use crate::event::ControllerEvent;
use crate::modules::input_expr::InputExpr;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use std::f32::consts::PI;

pub struct Filter12db {
    f0: f32,
    state_hp: usize,
    state_bp: usize,
    state_lp: usize,
    freq_control_input: InputExpr,
    linear_control_input: InputExpr,
    res_control_input: InputExpr,
    signal_input: InputExpr,
}

impl Filter12db {
    pub fn new(
        f0: f32,
        state_hp: usize,
        state_bp: usize,
        state_lp: usize,
        freq_control_input: InputExpr,
        linear_control_input: InputExpr,
        res_control_input: InputExpr,
        signal_input: InputExpr,
    ) -> Self {
        Self {
            f0,
            state_hp,
            state_bp,
            state_lp,
            freq_control_input,
            linear_control_input,
            res_control_input,
            signal_input,
        }
    }
}

fn amp(f0: f32, f: f32, l: f32) -> f32 {
    let freq = f0 * 2.0_f32.powf(f) + l;
    2. * PI * freq
}

fn feedback(q: f32) -> f32 {
    1. / q.clamp(0.0001, 1.)
}

impl Module for Filter12db {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let a = amp(
            self.f0,
            self.freq_control_input.from_state(state),
            self.linear_control_input.from_state(state),
        );
        let b = feedback(self.res_control_input.from_state(state));

        update.set(
            self.state_hp,
            self.signal_input.from_state(state)
                - state.get(self.state_lp)
                - state.get(self.state_bp) * b,
            UpdateType::Absolute,
        );
        update.set(
            self.state_bp,
            a * state.get(self.state_hp),
            UpdateType::Differentiable,
        );
        update.set(
            self.state_lp,
            a * state.get(self.state_bp),
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
