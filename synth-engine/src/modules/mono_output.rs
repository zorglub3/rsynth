use crate::midi::message::MidiMessage;
use crate::modules::input_expr::InputExpr;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate};

pub struct MonoOutput {
    output_index: usize,
    signal_input: InputExpr,
}

impl MonoOutput {
    pub fn new(output_index: usize, signal_input: InputExpr) -> Self {
        Self {
            output_index,
            signal_input,
        }
    }
}

impl Module for MonoOutput {
    fn simulate(&self, _state: &State, _update: &mut StateUpdate) {
        /* do nothing */
    }

    fn process_event(&mut self, _even: &MidiMessage, _channel: u8) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State) {
        let v = self.signal_input.from_state(state);
        state.set_output(self.output_index, v);
    }
}
