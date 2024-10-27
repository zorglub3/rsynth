use synth_engine::modules::*;
use synth_engine::simulator::module::Module;
use crate::*;
use ini::Properties;
use crate::modules::*;

const MODULE_TYPE: &str = "midi_mono";
const MODULE_NAME: &str = "name";
const CHANNEL: &str = "channel";
const PITCH_OUTPUT: &str = "pitch";
const GATE_OUTPUT: &str = "gate";

pub struct MidiMonoModuleSpec {
    name: String,
    channel: u8,
    state: [usize; 2],
}

impl MidiMonoModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let name =
            props.get(MODULE_NAME)
                .ok_or(ModuleError::MissingField {
                    module_type: MODULE_TYPE.to_string(),
                    field_name: MODULE_NAME.to_string(),
                })?;

        Ok(Self {
            name: name.to_string(),
            channel: props.get(CHANNEL).map(|s| s.parse::<u8>()).unwrap_or(Ok(1_u8))?,
            state: [0; 2],
        })
    }
}

impl ModuleSpec for MidiMonoModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        todo!()
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            PITCH_OUTPUT => Ok(self.state[0]),
            GATE_OUTPUT => Ok(self.state[1]),
            _ => Err(ModuleError::MissingStateName { module_type: MODULE_TYPE.to_string(), module_name: self.name.clone(), field_name: state_field.to_string() }),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

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
