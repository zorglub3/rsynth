use synth_engine::modules::*;
use synth_engine::simulator::module::Module;
use crate::*;

pub struct FilterModule {
    name: String,
    inputs: [usize; 3],
    state: [usize; 4],
}

impl FilterModule {
    pub fn new(f0: f32) -> Self {
        Self {
            name: "Moog filter".to_string(),
            inputs: [0; 3],
            state: [0; 4],
        }
    }

    pub fn signal_input(&mut self) -> ModuleInput {
        todo!()
    }
    
    pub fn signal_output(&self) -> ModuleOutput {
        todo!()
    }

    pub fn frequency_control(&mut self) -> ModuleInput {
        todo!()
    } 

    pub fn resonance_control(&mut self) -> ModuleInput {
        todo!()
    }
}

impl SynthModule for FilterModule {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn state_size(&self) -> usize {
        4
    }

    fn allocate_state(&mut self, state_allocator: &mut StateAllocator) {
        state_allocator.allocate(&mut self.state);
    }

    fn set_input(&mut self, input_index: usize, state_index: usize) {
        self.inputs[input_index] = state_index;
    }

    fn create(self) -> Box<dyn Module> {
        Box::new(
            MoogFilter::new(
                self.f0,
                self.state[0],
                self.state[1],
                self.state[2],
                self.state[3],
                self.inputs[0],
                self.inputs[1],
                self.inputs[2],
            )
        )
    }
}
