use synth_engine::modules::*;
use synth_engine::simulator::module::Module;
use crate::*;

pub struct ContourModule {
    name: String,
    inputs: [usize; 3],
    state: [usize; 1],
}

impl ContourModule {
    pub fn new() -> Self {
        Self {
            name: "contour".to_string(),
            inputs: [0; 3],
            state: [0; 1],
        }
    }

    pub fn signal_output(&self) -> ModuleOutput {
        ModuleOutput {
            module_output_index: 0,
            state_index: self.state[0],
        }
    }

    pub fn rise_control_input(&mut self) -> ModuleInput {
        ModuleInput {
            module_input_index: 1,
            module: self,
        }
    }

    pub fn fall_control_input(&mut self) -> ModuleInput {
        ModuleInput {
            module_input_index: 2,
            module: self,
        }
    }

    pub fn signal_input(&mut self) -> ModuleInput {
        ModuleInput {
            module_input_index: 0,
            module: self,
        }
    }
}

impl SynthModule for ContourModule {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn state_size(&self) -> usize {
        1
    }

    fn allocate_state(&mut self, state_allocator: &mut StateAllocator) {
        state_allocator.allocate(&mut self.state);
    }

    fn set_input(&mut self, input_index: usize, state_index: usize) {
        self.inputs[input_index] = state_index;
    }

    fn create(self) -> Box<dyn Module> {
        Box::new(
            ADEnvelope::new(
                self.inputs[0],
                self.state[0],
                self.inputs[1],
                self.inputs[2],
            )
        )
    }
}
