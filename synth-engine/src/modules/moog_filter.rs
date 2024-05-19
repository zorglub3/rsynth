use crate::midi::message::MidiMessage;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use std::f32::consts::PI;

pub struct MoogFilter {
    f0: f32,
    state0_index: usize,
    state1_index: usize,
    state2_index: usize,
    state3_index: usize,
    freq_control_index: usize,
    res_control_index: usize,
    input_index: usize,
}

impl MoogFilter {
    pub fn new(
        f0: f32,
        state0_index: usize,
        state1_index: usize,
        state2_index: usize,
        state3_index: usize,
        freq_control_index: usize,
        res_control_index: usize,
        input_index: usize,
    ) -> Self {
        Self {
            f0,
            state0_index,
            state1_index,
            state2_index,
            state3_index,
            freq_control_index,
            res_control_index,
            input_index,
        }
    }
}

fn control_to_frequency(f0: f32, exp_control: f32, lin_control: f32) -> f32 {
    f0 * 2.0_f32.powf(exp_control) + lin_control
}

impl Module for MoogFilter {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let f: f32 = control_to_frequency(self.f0, state.get(self.freq_control_index), 0.);
        let g: f32 = f * 2. * PI;
        let r: f32 = state.get(self.res_control_index);

        update.set(
            self.state0_index,
            g * (state.get(self.input_index)
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

    fn process_event(&mut self, _event: &MidiMessage, _channel: u8) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State) {
        /* do nothing */
    }
}
