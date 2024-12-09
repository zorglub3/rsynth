use super::control_to_frequency;
use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;
use core::f32::consts::PI;

const FILTER_FREQUENCY: f32 = 15_000.0;

// TODO - try out polyBLEP

pub struct SawOscillator {
    f0: f32,
    filter_state: usize,
    signal_output: usize,
    pitch_control: StackProgram,
    linear_modulation: StackProgram,
    current_position: f32,
    amp: f32,
}

impl SawOscillator {
    pub fn new(
        f0: f32,
        filter_state: usize,
        signal_output: usize,
        pitch_control: StackProgram,
        linear_modulation: StackProgram,
    ) -> Self {
        Self {
            f0,
            filter_state,
            signal_output,
            pitch_control,
            linear_modulation,
            current_position: 1.,
            amp: 2. * PI * FILTER_FREQUENCY,
        }
    }

    fn saw_eval(&self, x: f32) -> f32 {
        let x = ((x % 1.) + 1.) % 1.;

        1. - 2. * x
    }

    fn integral(&self, start: f32, end: f32) -> f32 {
        let start = ((start % 1.) + 1.) % 1.;
        let end = ((end % 1.) + 1.) % 1.;

        let i_start = start - start * start;
        let i_end = end - end * end;

        i_end - i_start
    }
}

impl Module for SawOscillator {
    fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]) {
        let velocity = control_to_frequency(
            self.f0,
            self.pitch_control.run(state, stack).unwrap_or(0.),
            self.linear_modulation.run(state, stack).unwrap_or(0.),
        );

        let distance = update.get_time_step() * velocity;
        let start = self.current_position + update.get_delta_time() * velocity;
        let end = start + distance;

        let i = if distance.abs() < f32::EPSILON {
            self.saw_eval(start)
        } else {
            self.integral(start, end) / distance
        };

        let a = self.amp;
        let b = 2.;

        update.set(
            self.filter_state,
            a * (i - b * state.get(self.filter_state) - state.get(self.signal_output)),
            UpdateType::Differentiable,
        );
        update.set(
            self.signal_output,
            a * state.get(self.filter_state),
            UpdateType::Differentiable,
        );
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State, time_step: f32, stack: &mut [f32]) {
        let velocity = control_to_frequency(
            self.f0,
            self.pitch_control.run(state, stack).unwrap_or(0.),
            self.linear_modulation.run(state, stack).unwrap_or(0.),
        );

        let p = self.current_position + velocity * time_step;

        self.current_position = ((p % 1.) + 1.) % 1.;
    }
}
