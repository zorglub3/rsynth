use crate::midi::message::MidiMessage;
use crate::modules::input_expr::InputExpr;
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
    signal_input: InputExpr,
    output_index: usize,
    attack_input: InputExpr,
    decay_input: InputExpr,
    env_state: EnvState,
    env_type: EnvType,
}

impl ADEnvelope {
    pub fn new(
        signal_input: InputExpr,
        output_index: usize,
        attack_input: InputExpr,
        decay_input: InputExpr,
    ) -> Self {
        Self {
            signal_input,
            output_index,
            attack_input,
            decay_input,
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
        let attack = self.attack_input.from_state(state);
        let decay = self.decay_input.from_state(state);

        match self.env_state {
            EnvState::Attack => {
                update.set(
                    self.output_index,
                    rise_decay(attack),
                    UpdateType::ClampedDifferentiable(0., 1.),
                );
            }
            EnvState::Decay => {
                update.set(
                    self.output_index,
                    -rise_decay(decay),
                    UpdateType::ClampedDifferentiable(0., 1.),
                );
            }
            _ => { /* do nothing */ }
        }
    }

    fn process_event(&mut self, _event: &MidiMessage, _channel: u8) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State) {
        let input_state = self.signal_input.from_state(state);
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
