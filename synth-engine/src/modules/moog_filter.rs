use crate::event::ControllerEvent;
use crate::modules::input_expr::InputExpr;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use std::f32::consts::PI;

pub struct MoogFilter {
    f0: f32,
    state0_index: usize,
    state1_index: usize,
    state2_index: usize,
    state3_index: usize,
    freq_control_input: InputExpr,
    res_control_input: InputExpr,
    signal_input: InputExpr,
}

impl MoogFilter {
    pub fn new(
        f0: f32,
        state0_index: usize,
        state1_index: usize,
        state2_index: usize,
        state3_index: usize,
        freq_control_input: InputExpr,
        res_control_input: InputExpr,
        signal_input: InputExpr,
    ) -> Self {
        Self {
            f0,
            state0_index,
            state1_index,
            state2_index,
            state3_index,
            freq_control_input,
            res_control_input,
            signal_input,
        }
    }
}

fn control_to_frequency(f0: f32, exp_control: f32, lin_control: f32) -> f32 {
    f0 * 2.0_f32.powf(exp_control) + lin_control
}

impl Module for MoogFilter {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let f: f32 = control_to_frequency(self.f0, self.freq_control_input.from_state(state), 0.);
        let g: f32 = f * 2. * PI;
        let r: f32 = self.res_control_input.from_state(state).max(0.);

        let input = self.signal_input.from_state(state);

        update.set(
            self.state0_index,
            g * (input
                - r * state.get(self.state3_index).tanh()
                - state.get(self.state0_index).tanh()),
            UpdateType::Differentiable,
        );
        update.set(
            self.state1_index,
            g * (state.get(self.state0_index).tanh() - state.get(self.state1_index).tanh()),
            UpdateType::Differentiable,
        );
        update.set(
            self.state2_index,
            g * (state.get(self.state1_index).tanh() - state.get(self.state2_index).tanh()),
            UpdateType::Differentiable,
        );
        update.set(
            self.state3_index,
            g * (state.get(self.state2_index).tanh() - state.get(self.state3_index).tanh()),
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
