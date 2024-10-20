use synth_engine::modules::*;
use synth_engine::simulator::module::Module;
use crate::*;

pub struct MidiCCModule {
    name: String,
    state: [usize; 1],
    channel: u8,
    control: u8,
}

impl MidiCCModule {
    pub fn new(control: u8, channel: u8) -> Self {
        Self {
            name: "midi CC".to_string(),
            state: [0; 1],
            channel,
            control,
        }
    } 

    pub fn value_output(&self) -> ModuleOutput {
        ModuleOutput {
            module_output_index: 0,
            state_index: self.state[0],
        }
    }
}

impl SynthModule for MidiCCModule {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn state_size(&self) -> usize {
        1
    }

    fn allocate_state(&mut self, state_allocator: &mut StateAllocator) {
        state_allocator.allocate(&mut self.state);
    }

    fn set_input(&mut self, _input_index: usize, _state_index: usize) {
        debug_assert!(false);
    }

    fn create(self) -> Box<dyn Module> {
        Box::new(
            MidiCC::new(
                self.state[0],
                self.control,
                self.channel,
            )
        )
    }
}
