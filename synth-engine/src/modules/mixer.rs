use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};

// TODO delete this file and module
pub struct Mixer {
    input_indices: Vec<usize>,
    attenuation_indices: Vec<usize>,
    output_index: usize,
}

impl Mixer {
    pub fn new(
        input_indices: Vec<usize>,
        attenuation_indices: Vec<usize>,
        output_index: usize,
    ) -> Self {
        debug_assert!(input_indices.len() == attenuation_indices.len());

        Self {
            input_indices,
            attenuation_indices,
            output_index,
        }
    }
}

impl Module for Mixer {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let mut output_value = 0.0;

        for i in 0..self.input_indices.len() {
            output_value +=
                state.get(self.input_indices[i]) * state.get(self.attenuation_indices[i]).max(0.0);
        }

        update.set(self.output_index, output_value, UpdateType::Absolute);
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, _state: &mut State, _time_step: f32) {
        /* do nothing */
    }
}
