pub mod modules;

use core::ops::Range;
use synth_engine::event::Event;
use synth_engine::simulator::module::Module;

pub struct StateAllocator(Range<usize>);

impl StateAllocator {
    pub fn new(size: usize) -> Self {
        let size = size.min(16);
        Self(5..size)
    }

    fn count_free_slots(&self) -> usize {
        self.0.end - self.0.start
    }

    fn allocate_state_slot(&mut self) -> usize {
        if self.0.is_empty() {
            self.0 = Range { start: self.0.start, end: self.0.end * 2 };
        }

        let slot = self.0.start;

        self.0 = Range { start: self.0.start + 1, end: self.0.end };

        slot
    }

    pub fn allocate(&mut self, state: &mut [usize]) {
        for i in 0 .. state.len() {
            state[i] = self.allocate_state_slot();            
        }
    }
} 

pub trait SynthModule {
    fn name(&self) -> String;
    fn state_size(&self) -> usize;
    fn allocate_state(&mut self, state_allocator: &mut StateAllocator);
    fn set_input(&mut self, input_index: usize, state_index: usize);
    fn create(self) -> Box<dyn Module>;
}

pub struct ModuleOutput {
    module_output_index: usize,
    state_index: usize,
}

pub struct ModuleInput<'a> {
    module_input_index: usize,    
    module: &'a mut dyn SynthModule,
}

impl ModuleInput<'_> {
    fn connect_to(&mut self, output: ModuleOutput) {
        self.module.set_input(self.module_input_index, output.state_index);
    }    
}

use std::collections::HashMap;

fn demo1() -> HashMap<String, Box<dyn Module>> {
    let mut state_allocator = StateAllocator::new(32);

    let mut osc = modules::OscModule::new(10.0, 1., 1.);
    let mut amp = modules::AmpModule::new();

    osc.allocate_state(&mut state_allocator);
    amp.allocate_state(&mut state_allocator);

    amp.signal_input().connect_to(osc.signal_output());

    let mut modules = HashMap::new();

    modules.insert("osc".to_string(), osc.create());
    modules.insert("vca".to_string(), amp.create());

    modules
}
