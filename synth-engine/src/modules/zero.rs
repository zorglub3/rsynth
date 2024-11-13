use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};

pub struct ZeroModule {
    state_index: usize,
}

impl ZeroModule {
    pub fn new(state_index: usize) -> Self {
        Self { state_index }
    }
}

impl Module for ZeroModule {
    fn simulate(&self, _state: &State, update: &mut StateUpdate) {
        update.set(self.state_index, 0., UpdateType::Absolute);
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State, _time_step: f32) {
        state.set(self.state_index, 0.);
    }
}
