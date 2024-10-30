use crate::midi::message::MidiMessage;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};

enum EnvState {
    Finished,
    Attack,
    Hold,
    Decay,
}

pub struct ADEnvelope {
    input_index: usize,
    output_index: usize,
    attack_index: usize,
    decay_index: usize,
    env_state: EnvState,
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
            env_state: EnvState::Finished,
        }
    }
}

impl Module for ADEnvelope {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let attack = state.get(self.attack_index);
        let decay = state.get(self.decay_index);

        match self.env_state {
            EnvState::Attack => update.set(self.output_index, attack, UpdateType::Differentiable),
            EnvState::Decay => update.set(self.output_index, -decay, UpdateType::Differentiable),
            _ => { /* do nothing */ }
        }
    }

    fn process_event(&mut self, _event: &MidiMessage, _channel: u8) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State) {
        let input_state = state.get(self.input_index);
        let output_state = state.get(self.output_index);

        match self.env_state {
            EnvState::Attack => {
                if output_state >= 1. {
                    self.env_state = EnvState::Hold;
                }
            }
            EnvState::Hold => {
                if input_state < 0.5 {
                    self.env_state = EnvState::Decay;
                }
            }
            EnvState::Decay => {
                if output_state <= 0. {
                    self.env_state = EnvState::Finished;
                }
            }
            EnvState::Finished => {
                if input_state > 0.5 {
                    self.env_state = EnvState::Attack;
                }
            }
        }

        state.set(self.output_index, output_state.max(0.).min(1.));
    }
}
