use super::control_to_frequency;
use crate::event::ControllerEvent;
use crate::interpolation::Interpolation;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;

pub struct DelayLine {
    data: Vec<f32>,
    current_index: usize,
    signal_output: usize,
    f0: f32,
    signal_input: StackProgram,
    pitch_control: StackProgram,
    linear_modulation: StackProgram,
}

impl DelayLine {
    pub fn new(
        f0: f32,
        signal_output: usize,
        signal_input: StackProgram,
        pitch_control: StackProgram,
        linear_modulation: StackProgram,
        data_size: usize,
    ) -> Self {
        Self {
            f0,
            signal_output,
            signal_input,
            pitch_control,
            linear_modulation,
            current_index: 0,
            data: vec![0.; data_size],
        }
    }

    fn index_modulo(&self, index: usize) -> usize {
        let size = self.data.len();

        ((index % size) + size) % size
    }

    fn increment_index(&mut self) {
        self.current_index = self.index_modulo(self.current_index + 1);
    }

    fn write_index(&self) -> usize {
        self.current_index
    }
}

impl Module for DelayLine {
    fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]) {
        let wi = self.write_index() as f32;
        let l = self.data.len() as f32;
        let d = update.get_time_step();
        let s = update.get_delta_time();

        let f = control_to_frequency(
            self.f0,
            self.pitch_control.run(state, stack).unwrap_or(0.),
            self.linear_modulation.run(state, stack).unwrap_or(0.),
        );

        let index = (1. / (d * f) - s / d).min(l - 5.).max(5.);
        let index = (((wi - index) % l) + l) % l;

        update.set(
            self.signal_output,
            self.data.lagrange_interpolate(index),
            UpdateType::Absolute,
        );
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State, _time_step: f32, stack: &mut [f32]) {
        let write_index = self.write_index();

        self.data[write_index] = self.signal_input.run(state, stack).unwrap_or(0.);
        self.increment_index();
    }
}
