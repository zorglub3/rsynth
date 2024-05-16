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

fn saw_value(v: f32) -> f32 {
    let v = modulo_one(v);
    v * 2. - 1.0
}

fn saw_average_value(start: f32, step: f32) -> f32 {
    let end = start + step / 44100.0; // TODO - this constant doesn't belong here!

    if end > 1. {
        let part_one = (saw_value(start) + 1.) / 2.;
        let part_two = (saw_value(end) - 1.) / 2.;

        part_one * (1. - start) + part_two * (end - 1.)
    } else {
        (saw_value(end) + saw_value(start)) / 2.
    }
}

pub struct SawOsc {
    f0: f32,
    freq_index: usize,
    state0_index: usize,
    state1_index: usize,
    output_index: usize,
    rc: f32,
}

impl SawOsc {
    pub fn new(
        f0: f32,
        freq_index: usize,
        state0_index: usize,
        state1_index: usize,
        output_index: usize,
        fc: f32,
    ) -> Self {
        let rc = PI / fc;

        Self {
            f0,
            freq_index,
            state0_index,
            state1_index,
            output_index,
            rc,
        }
    }
}

impl Module for SawOsc {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let f = state.get(self.freq_index);
        let step = freq_to_step(self.f0, f, 0.);
        update.set(self.state0_index, step, UpdateType::Differentiable);

        let start = modulo_one(state.get(self.state0_index));
        let v = saw_average_value(start, step);
        let u = (v - state.get(self.state1_index)) / self.rc - state.get(self.state1_index) * 0.01;

        update.set(self.state1_index, u, UpdateType::Differentiable);

        let s = (state.get(self.state1_index) - state.get(self.output_index)) / self.rc;

        update.set(self.output_index, s, UpdateType::Differentiable);
    }

    fn process_event(&mut self, _event: &MidiMessage, _channel: u8) {
        /* do nothing */
    }

    fn finalize(&self, state: &mut State) {
        let v = modulo_one(state.get(self.state0_index));
        state.set(self.state0_index, v);
    }
}
