use super::control_to_frequency;
use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;
use std::f32::consts::PI;

pub struct Filter12db {
    f0: f32,
    state_hp: usize,
    state_bp: usize,
    state_lp: usize,
    freq_control_input: StackProgram,
    linear_control_input: StackProgram,
    res_control_input: StackProgram,
    signal_input: StackProgram,
}

impl Filter12db {
    pub fn new(
        f0: f32,
        state_hp: usize,
        state_bp: usize,
        state_lp: usize,
        freq_control_input: StackProgram,
        linear_control_input: StackProgram,
        res_control_input: StackProgram,
        signal_input: StackProgram,
    ) -> Self {
        Self {
            f0,
            state_hp,
            state_bp,
            state_lp,
            freq_control_input,
            linear_control_input,
            res_control_input,
            signal_input,
        }
    }
}

fn amp(f0: f32, f: f32, l: f32) -> f32 {
    let freq = control_to_frequency(f0, f, l);
    2. * PI * freq
}

fn feedback(q: f32) -> f32 {
    1. / q.clamp(0.0001, 1.)
}

impl Module for Filter12db {
    fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]) {
        let a = amp(
            self.f0,
            self.freq_control_input.run(state, stack).unwrap_or(0.),
            self.linear_control_input.run(state, stack).unwrap_or(0.),
        );
        let b = feedback(self.res_control_input.run(state, stack).unwrap_or(0.));

        update.set(
            self.state_hp,
            self.signal_input.run(state, stack).unwrap_or(0.)
                - state.get(self.state_lp)
                - state.get(self.state_bp) * b,
            UpdateType::Absolute,
        );
        update.set(
            self.state_bp,
            a * state.get(self.state_hp),
            UpdateType::Differentiable,
        );
        update.set(
            self.state_lp,
            a * state.get(self.state_bp),
            UpdateType::Differentiable,
        );
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, _state: &mut State, _time_step: f32, _stack: &mut [f32]) {
        /* do nothing */
    }
}
