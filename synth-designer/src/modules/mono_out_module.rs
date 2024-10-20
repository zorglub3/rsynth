use synth_engine::modules::*;
use synth_engine::simulator::module::Module;
use crate::*;

pub struct MonoOutputModule {
    name: String,
    inputs: [usize; 1],
}

impl MonoOutputModule {
    pub fn new() -> Self {
        Self {
            name: "mono output".to_string(),
            inputs: [0; 1],
        }
    }

    pub fn signal_input(&mut self) -> ModuleInput {
        ModuleInput {
            module_input_index: 0,
            module: self,
        }
    }
}

impl SynthModule for MonoOutputModule {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn state_size(&self) -> usize {
        0
    }

    fn allocate_state(&mut self, _state_allocator: &mut StateAllocator) {
        /* do nothing - this module has no internal state */
    }

    fn set_input(&mut self, input_index: usize, state_index: usize) {
        self.inputs[input_index] = state_index;
    }

    fn create(self) -> Box<dyn Module> {
        todo!()
    }
}
