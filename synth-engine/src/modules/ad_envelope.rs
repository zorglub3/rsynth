use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;

// TODO env type selectable
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
    signal_input: StackProgram,
    output_index: usize,
    attack_input: StackProgram,
    decay_input: StackProgram,
    env_state: EnvState,
    env_type: EnvType,
}

impl ADEnvelope {
    pub fn new(
        signal_input: StackProgram,
        output_index: usize,
        attack_input: StackProgram,
        decay_input: StackProgram,
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
    fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]) {
        let attack = self.attack_input.run(state, stack).unwrap_or(0.);
        let decay = self.decay_input.run(state, stack).unwrap_or(0.);

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

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State, _time_step: f32, stack: &mut [f32]) {
        let input_state = self.signal_input.run(state, stack).unwrap_or(0.0);
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
