use crate::event::ControllerEvent;
use crate::modules::input_expr::InputExpr;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};

pub struct Modulator {
    signal_1_input: InputExpr,
    signal_2_input: InputExpr,
    output_index: usize,
}

impl Modulator {
    pub fn new(signal_1_input: InputExpr, signal_2_input: InputExpr, output_index: usize) -> Self {
        Self {
            signal_1_input,
            signal_2_input,
            output_index,
        }
    }
}

impl Module for Modulator {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let u = self.signal_1_input.from_state(state);
        let v = self.signal_2_input.from_state(state);

        update.set(self.output_index, u * v, UpdateType::Absolute);
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, _state: &mut State, _time_step: f32) {
        /* do nothing */
    }
}
