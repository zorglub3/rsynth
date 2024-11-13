use crate::event::ControllerEvent;
use crate::modules::input_expr::InputExpr;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};

pub struct Folder {
    signal_input: InputExpr,
    control_input: InputExpr,
    output_index: usize,
}

impl Folder {
    pub fn new(signal_input: InputExpr, control_input: InputExpr, output_index: usize) -> Self {
        Self {
            signal_input,
            control_input,
            output_index,
        }
    }
}

impl Module for Folder {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let i = self.signal_input.from_state(state);
        let c = self.control_input.from_state(state).max(0.).min(5.) + 1.;

        update.set(self.output_index, (i * c).sin(), UpdateType::Absolute);
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, _state: &mut State, _time_step: f32) {
        /* do nothing */
    }
}
