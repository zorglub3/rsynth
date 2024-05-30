use crate::midi::message::MidiMessage;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};

pub struct ADEnvelope {
    input_index: usize,
    output_index: usize,
    attack_index: usize,
    decay_index: usize,
    attack_state: bool,
}

impl ADEnvelope {
    pub fn new(
        input_index: usize,
        output_index: usize,
        attack_index: usize,
        decay_index: usize,
    ) -> Self {
        Self {
            input_index,
            output_index,
            attack_index,
            decay_index,
            attack_state: false,
        }
    }
}

impl Module for ADEnvelope {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        if self.attack_state {
            let attack = state.get(self.attack_index);
            update.set(self.output_index, attack, UpdateType::Differentiable);
        } else {
            let v = state.get(self.output_index);

            if v > 0. {
                let decay = state.get(self.decay_index);
                update.set(self.output_index, decay, UpdateType::Differentiable);
            }
        }
    }

    fn process_event(&mut self, _event: &MidiMessage, _channel: u8) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State) {
        let v = state.get(self.output_index);

        if self.attack_state {
            if v > 1. {
                state.set(self.output_index, 1.);
                self.attack_state = false;
            }
        } else {
            if state.get(self.input_index) > 0.1 {
                self.attack_state = true;
            }
        }

        if v < 0. {
            state.set(self.output_index, 0.);
        }
    }
}
