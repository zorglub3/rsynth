use super::control_to_frequency;
use crate::event::ControllerEvent;
use crate::interpolation::Interpolation;
use crate::modules::input_expr::InputExpr;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};

pub struct DelayLine {
    data: Vec<f32>,
    current_index: usize,
    signal_output: usize,
    f0: f32,
    signal_input: InputExpr,
    pitch_control: InputExpr,
    linear_modulation: InputExpr,
}

impl DelayLine {
    pub fn new(
        f0: f32,
        signal_output: usize,
        signal_input: InputExpr,
        pitch_control: InputExpr,
        linear_modulation: InputExpr,
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
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let wi = self.write_index() as f32;
        let l = self.data.len() as f32;
        let d = update.get_time_step();

        let f = control_to_frequency(
            self.f0,
            self.pitch_control.from_state(state),
            self.linear_modulation.from_state(state),
        );

        let index = (f / d).min(l - 1.).max(3.);
        let index = (((index + wi) % l) + l) % l;

        update.set(
            self.signal_output,
            self.data.cubic_interpolate(index),
            UpdateType::Absolute,
        );
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State, _time_step: f32) {
        let write_index = self.write_index();

        self.data[write_index] = self.signal_input.from_state(state);
        self.increment_index();
    }
}
