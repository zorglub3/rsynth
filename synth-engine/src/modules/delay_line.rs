use crate::midi::message::MidiMessage;
use crate::modules::input_expr::InputExpr;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};

pub struct DelayLine {
    data: Vec<f32>,
    current_index: usize,
    output_index: usize,
    signal_input: InputExpr,
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

    fn read_index(&mut self) -> usize {
        self.index_modulo(self.current_index + 1)
    }

    fn write_index(&mut self) -> usize {
        self.current_index
    }
}

impl Module for DelayLine {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        todo!()
    }

    fn process_event(&mut self, _event: &MidiMessage, _channel: u8) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State) {
        todo!()
    }
}
