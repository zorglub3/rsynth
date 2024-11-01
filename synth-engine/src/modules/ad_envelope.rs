use crate::midi::message::MidiMessage;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};

const MIN_TIME: f32 = 0.0001_f32;

enum EnvState {
    Finished,
    Attack,
    Hold,
    Decay,
}

#[derive(Eq, PartialEq)]
enum EnvType {
    AttackDecay,
    AttackRelease,
    Cyclic,
}

pub struct ADEnvelope {
    input_index: usize,
    output_index: usize,
    attack_index: usize,
    decay_index: usize,
    env_state: EnvState,
    env_type: EnvType,
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
            env_type: EnvType::AttackRelease,
        }
    }
}

fn rise_decay(t: f32) -> f32 {
    let t = t.max(MIN_TIME);

    1. / t
}

impl Module for ADEnvelope {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let attack = state.get(self.attack_index);
        let decay = state.get(self.decay_index);

        match self.env_state {
            EnvState::Attack => {
                update.set(
                    self.output_index,
                    rise_decay(attack),
                    UpdateType::Differentiable,
                );
            }
            EnvState::Decay => {
                update.set(
                    self.output_index,
                    -rise_decay(decay),
                    UpdateType::Differentiable,
                );
            }
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
                    if self.env_type == EnvType::AttackRelease {
                        self.env_state = EnvState::Hold;
                    } else {
                        self.env_state = EnvState::Decay;
                    }
                }
            }
            EnvState::Hold => {
                if input_state < 0.5 || self.env_type == EnvType::AttackDecay {
                    self.env_state = EnvState::Decay;
                }
            }
            EnvState::Decay => {
                if output_state <= 0. {
                    if self.env_type == EnvType::Cyclic {
                        self.env_state = EnvState::Attack;
                    } else {
                        self.env_state = EnvState::Finished;
                    }
                }

                if input_state > 0.5 {
                    self.env_state = EnvState::Attack;
                }
            }
            EnvState::Finished => {
                if input_state > 0.5 || self.env_type == EnvType::Cyclic {
                    self.env_state = EnvState::Attack;
                }
            }
        }

        state.set(self.output_index, output_state.max(0.).min(1.));
    }
}
