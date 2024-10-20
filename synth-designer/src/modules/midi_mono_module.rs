use synth_engine::modules::*;
use synth_engine::simulator::module::Module;
use crate::*;

pub struct MidiMonoModule {
    name: String,
    state: [usize; 2],
    channel: u8,
}

impl MidiMonoModule {
    pub fn new(channel: u8) -> Self {
        Self {
            name: "midi_mono".to_string(),
            state: [0; 2],
            channel,
        }
    }

    pub fn pitch_output(&self) -> ModuleOutput {
        ModuleOutput {
            module_output_index: 0,
            state_index: self.state[0],
        }
    }

    pub fn gate_output(&self) -> ModuleOutput {
        ModuleOutput {
            module_output_index: 0,
            state_index: self.state[1],
        }
    }

    pub fn velocity_output(&self) -> ModuleOutput {
        todo!("Velocity not implemented yet")
    }
}

impl SynthModule for MidiMonoModule {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn state_size(&self) -> usize {
        3
    }

    fn allocate_state(&mut self, state_allocator: &mut StateAllocator) {
        state_allocator.allocate(&mut self.state);
    }

    fn set_input(&mut self, input_index: usize, state_index: usize) {
        debug_assert!(false);
    }

    fn create(self) -> Box<dyn Module> {
        Box::new(
            MidiMono::new(
                self.state[0],
                self.state[1],
                self.channel,
            )
        )
    }
}
