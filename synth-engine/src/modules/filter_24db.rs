use super::control_to_frequency;
use crate::distortion::*;
use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;
use core::f32::consts::PI;

pub struct Filter24db<'a> {
    f0: f32,
    state0_index: usize,
    state1_index: usize,
    state2_index: usize,
    state3_index: usize,
    freq_control_input: StackProgram<'a>,
    linear_control_input: StackProgram<'a>,
    res_control_input: StackProgram<'a>,
    signal_input: StackProgram<'a>,
    distortion: Option<DistortionType>,
}

impl<'a> Filter24db<'a> {
    pub fn new(
        f0: f32,
        state0_index: usize,
        state1_index: usize,
        state2_index: usize,
        state3_index: usize,
        freq_control_input: StackProgram<'a>,
        linear_control_input: StackProgram<'a>,
        res_control_input: StackProgram<'a>,
        signal_input: StackProgram<'a>,
    ) -> Self {
        Self {
            f0,
            state0_index,
            state1_index,
            state2_index,
            state3_index,
            freq_control_input,
            linear_control_input,
            res_control_input,
            signal_input,
            distortion: Some(DistortionType::Smoothstep),
        }
    }
}

impl<'a> Module for Filter24db<'a> {
    fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]) {
        let f: f32 = control_to_frequency(
            self.f0,
            self.freq_control_input.run(state, stack).unwrap_or(0.),
            self.linear_control_input.run(state, stack).unwrap_or(0.),
        );
        let g: f32 = f * 2. * PI;
        let r: f32 = self
            .res_control_input
            .run(state, stack)
            .unwrap_or(0.)
            .max(0.);

        let input = self.signal_input.run(state, stack).unwrap_or(0.);

        update.set(
            self.state0_index,
            g * ((input - r * state.get(self.state3_index)).distort(&self.distortion)
                - state.get(self.state0_index).distort(&self.distortion)),
            UpdateType::Differentiable,
        );
        update.set(
            self.state1_index,
            g * (state.get(self.state0_index).distort(&self.distortion)
                - state.get(self.state1_index).distort(&self.distortion)),
            UpdateType::Differentiable,
        );
        update.set(
            self.state2_index,
            g * (state.get(self.state1_index).distort(&self.distortion)
                - state.get(self.state2_index).distort(&self.distortion)),
            UpdateType::Differentiable,
        );
        update.set(
            self.state3_index,
            g * (state.get(self.state2_index).distort(&self.distortion)
                - state.get(self.state3_index).distort(&self.distortion)),
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
