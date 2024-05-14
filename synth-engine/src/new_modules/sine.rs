use crate::midi::message::MidiMessage;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use std::f32::consts::PI;

fn modulo_one(v: f32) -> f32 {
    ((v % 1.) + 1.) % 1.
}

fn freq_to_step(f0: f32, freq: f32, lin_freq: f32) -> f32 {
    (f0 * 2.0_f32.powf(freq)) + lin_freq * 100.0
}

pub struct SinOsc {
    f0: f32,
    freq_index: usize,
    state0_index: usize,
    output_index: usize,
}

impl SinOsc {
    pub fn new(
        f0: f32,
        freq_index: usize,
        state0_index: usize,
        output_index: usize,
    ) -> Self {
        Self {
            f0,
            freq_index,
            state0_index,
            output_index,
        }
    }
}

impl Module for SinOsc {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let f = state.get(self.freq_index);
        let step = freq_to_step(self.f0, f, 0.);
        update.set(self.state0_index, step, UpdateType::Differentiable);

        let v = (modulo_one(state.get(self.state0_index)) * 2.0 * PI).sin();

        update.set(self.output_index, v, UpdateType::Absolute);
    }

    fn process_event(&mut self, _event: &MidiMessage, _channel: u8) {
        /* do nothing */
    }

    fn finalize(&self, state: &mut State) {
        let v = modulo_one(state.get(self.state0_index));
        state.set(self.state0_index, v);
    }
}
