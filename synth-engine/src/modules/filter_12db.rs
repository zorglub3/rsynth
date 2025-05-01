use super::control_to_frequency;
use crate::control_interface::ControlInterface;
use crate::distortion::*;
use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;
use core::f32::consts::PI;

pub const STATE_SIZE: usize = 3;
pub const INPUT_SIZE: usize = 4;

// state/outputs
const STATE_HP: usize = 0;
const STATE_BP: usize = 1;
const STATE_LP: usize = 2;

// inputs/controls
const EXP_CONTROL_INPUT: usize = 0;
const LINEAR_CONTROL_INPUT: usize = 1;
const RESONANCE_CONTROL_INPUT: usize = 2;
const SIGNAL_INPUT: usize = 3;

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
    fn simulate(
        &self,
        _control_interface: &ControlInterface,
        inputs: &[f32],
        state: &[f32],
        update: &mut [f32],
        _dt: f32,
    ) {
        let a = control_to_frequency(
            self.f0,
            inputs[EXP_CONTROL_INPUT],
            inputs[LINEAR_CONTROL_INPUT],
        );
        let b = 1. / inputs[RESONANCE_CONTROL_INPUT].max(0.01);

        let input = inputs[SIGNAL_INPUT];
        let bp_value = state[STATE_BP];
        let lp_value = state[STATE_LP];
        let hp_value = input - (bp_value * b).distort(&self.distortion) - lp_value;

        let bp_value = bp_value.distort(&self.distortion);

        update[STATE_HP] = hp_value;
        update[STATE_BP] = a * hp_value;
        update[STATE_LP] = a * bp_value;
    }

    fn finalize(&mut self, _inputs: &[f32], state: &mut [f32], _outputs: &mut [f32], _dt: f32) {
        let bp = state[STATE_BP].clamp(-CLAMP_VALUE, CLAMP_VALUE);
        let lp = state[STATE_LP].clamp(-CLAMP_VALUE, CLAMP_VALUE);

        state[STATE_BP] = bp;
        state[STATE_LP] = lp;
    }

    fn get_input_size(&self) -> usize {
        INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        update_types[STATE_HP] = UpdateType::Absolute;
        update_types[STATE_BP] = UpdateType::Differentiable;
        update_types[STATE_LP] = UpdateType::Differentiable;
    }

    /*
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

        state.set(self.state_bp, bp.clamp(-CLAMP_VALUE, CLAMP_VALUE));
        state.set(self.state_lp, lp.clamp(-CLAMP_VALUE, CLAMP_VALUE));
    }
    */
}
