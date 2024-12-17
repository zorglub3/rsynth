use super::control_to_frequency;
use crate::distortion::*;
use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;
use core::f32::consts::PI;

const CLAMP_VALUE: f32 = 2.5;

pub struct Filter12db {
    f0: f32,
    state_hp: usize,
    state_bp: usize,
    state_lp: usize,
    freq_control_input: StackProgram,
    linear_control_input: StackProgram,
    res_control_input: StackProgram,
    signal_input: StackProgram,
    distortion: Option<DistortionType>,
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
            distortion: None, // Some(DistortionType::Smoothstep), //Some(make_diodelike(1000., 2.)),
        }
    }
}

impl Module for Filter12db {
    fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]) {
        let a = control_to_frequency(
            self.f0,
            self.freq_control_input.run(state, stack).unwrap_or(0.),
            self.linear_control_input.run(state, stack).unwrap_or(0.),
        ) * 2.
            * PI;
        let b = 1.
            / self
                .res_control_input
                .run(state, stack)
                .unwrap_or(0.)
                .max(0.4);

        let input = self.signal_input.run(state, stack).unwrap_or(0.);

        let input = input.distort(&self.distortion);
        let bp_value = state.get(self.state_bp);
        let lp_value = state.get(self.state_lp).distort(&self.distortion);

        let hp_value = input - (bp_value * b).distort(&self.distortion) - lp_value;

        let bp_value = bp_value.distort(&self.distortion);

        update.set(self.state_hp, hp_value, UpdateType::Absolute);
        update.set(self.state_bp, a * hp_value, UpdateType::Differentiable);
        update.set(self.state_lp, a * bp_value, UpdateType::Differentiable);
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State, _time_step: f32, _stack: &mut [f32]) {
        let bp = state.get(self.state_bp);
        let lp = state.get(self.state_lp);

        state.set(bp.clamp(-CLAMP_VALUE, CLAMP_VALUE));
        state.set(lp.clamp(-CLAMP_VALUE, CLAMP_VALUE));
    }
}
