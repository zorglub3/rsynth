use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;
use core::f32::consts::PI;
use libm::{fabsf, cosf};

#[allow(dead_code)]
fn hamming(x: f32) -> f32 {
    let x = x.clamp(0., 1.);

    0.54 - 0.46 * cosf(2. * PI * x)
}

#[allow(dead_code)]
fn blackman(x: f32) -> f32 {
    let x = x.clamp(0., 1.);
    0.42 - 0.5 * cosf(2. * PI * x) + 0.08 * cosf(4. * PI * x)
}

#[allow(dead_code)]
fn triangle(x: f32) -> f32 {
    let x = x.clamp(0., 1.);

    1. - fabsf(2. * x - 1.)
}

const MIN_TIME: f32 = 0.01_f32; // 10 ms

enum EnvState {
    Finished,
    Attack,
    Hold,
    Decay,
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
enum EnvType {
    AttackDecay,
    AttackRelease,
    Cyclic,
}

pub struct Envelope<'a> {
    signal_input: StackProgram<'a>,
    attack_input: StackProgram<'a>,
    decay_input: StackProgram<'a>,
    shape_select: StackProgram<'a>,
    output_index: usize,
    cycle_state: usize,
    env_state: EnvState,
    env_type: EnvType,
}

impl<'a> Envelope<'a> {
    pub fn new(
        signal_input: StackProgram<'a>,
        attack_input: StackProgram<'a>,
        decay_input: StackProgram<'a>,
        shape_select: StackProgram<'a>,
        output_index: usize,
        cycle_state: usize,
    ) -> Self {
        Self {
            signal_input,
            attack_input,
            decay_input,
            shape_select,
            output_index,
            cycle_state,
            env_state: EnvState::Finished,
            env_type: EnvType::AttackRelease,
        }
    }
}

fn rise_decay(t: f32) -> f32 {
    let t = t.max(MIN_TIME);

    1. / t
}

fn output_value(cycle_index: f32, shape: f32) -> f32 {
    let cycle_index = (cycle_index * 0.5).clamp(0., 0.5);
    let shape = shape.clamp(0., 1.);
    blackman(cycle_index) * shape + triangle(cycle_index) * (1. - shape)
}

impl<'a> Module for Envelope<'a> {
    fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]) {
        let attack = self.attack_input.run(state, stack).unwrap_or(0.);
        let decay = self.decay_input.run(state, stack).unwrap_or(0.);

        match self.env_state {
            EnvState::Attack => {
                let delta = rise_decay(attack);
                update.set(self.cycle_state, delta, UpdateType::Differentiable);
                update.set(
                    self.output_index,
                    output_value(
                        state.get(self.cycle_state),
                        self.shape_select.run(state, stack).unwrap_or(0.),
                    ),
                    UpdateType::Absolute,
                );
            }
            EnvState::Decay => {
                let delta = -rise_decay(decay);
                update.set(self.cycle_state, delta, UpdateType::Differentiable);
                update.set(
                    self.output_index,
                    output_value(
                        state.get(self.cycle_state),
                        self.shape_select.run(state, stack).unwrap_or(0.),
                    ),
                    UpdateType::Absolute,
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
        let cycle = state.get(self.cycle_state);

        use EnvState::*;
        use EnvType::*;

        match (&self.env_state, &self.env_type) {
            (Attack, AttackRelease) if cycle >= 1. => self.env_state = Hold,
            (Attack, AttackDecay | Cyclic) if cycle >= 1. => self.env_state = Decay,
            (Hold, AttackRelease) if input_state < 0.5 => self.env_state = Decay,
            (Hold, AttackDecay | Cyclic) => self.env_state = Decay,
            (Decay, Cyclic) if cycle <= 0. => self.env_state = Attack,
            (Decay, AttackRelease | AttackDecay) if input_state >= 0.5 => self.env_state = Attack,
            (Decay, AttackRelease | AttackDecay) if cycle <= 0. => self.env_state = Finished,
            (Finished, Cyclic) => self.env_state = Attack,
            (Finished, AttackRelease | AttackDecay) if input_state > 0.5 => self.env_state = Attack,
            _ => { /* do nothing */ }
        }

        state.set(self.output_index, output_state.clamp(0., 1.));
        state.set(self.cycle_state, cycle.clamp(0., 1.));
    }
}
