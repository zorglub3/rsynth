use crate::event::ControllerEvent;
use crate::modules::input_expr::InputExpr;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};

pub struct DelayLine {
    data: Vec<f32>,
    current_index: usize,
    output_index: usize,
    signal_input: InputExpr,
}

// see https://www.paulinternet.nl/?page=bicubic
fn cubic_interpolation(p0: f32, p1: f32, p2: f32, p3: f32, x: f32) -> f32 {
    let x2 = x * x;
    let x3 = x2 * x;

    (-0.5 * p0 + 1.5 * p1 - 1.5 * p2 + 0.5 * p3) * x3
        + (p0 - 2.5 * p1 + 2. * p2 - 0.5 * p3) * x2
        + (-0.5 * p0 + 0.5 * p2) * x
        + p1
}

impl DelayLine {
    pub fn new(size: usize, signal_input: InputExpr, output_index: usize) -> Self {
        Self {
            data: vec![0.0_f32; size],
            current_index: 0,
            output_index,
            signal_input,
        }
    }

    fn index_modulo(&self, index: usize) -> usize {
        let size = self.data.len();

        ((index % size) + size) % size
    }

    fn increment_index(&mut self) {
        self.current_index = self.index_modulo(self.current_index + 1);
    }

    fn read_index(&self) -> usize {
        self.index_modulo(self.current_index + 1)
    }

    fn write_index(&self) -> usize {
        self.current_index
    }

    fn data_point(&self, delta: usize) -> f32 {
        todo!()
    }
}

impl Module for DelayLine {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let y0 = self.data[self.read_index()];
        let y1 = self.data[self.index_modulo(self.current_index + 2)];
        let y2 = self.data[self.index_modulo(self.current_index + 3)];

        let y = todo!();
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
