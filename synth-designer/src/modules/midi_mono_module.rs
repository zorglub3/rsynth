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
const STATE_SIZE: usize = 2;

pub struct MidiMonoModuleSpec {
    name: String,
    channel: u8,
    state: [usize; STATE_SIZE],
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

    fn create_module(&self, _synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let midi_mono = MidiMono::new(
            self.state[0],
            self.state[1],
            self.channel,
        );

        Ok(Box::new(midi_mono))
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

    fn state_size(&self) -> usize {
        self.state.len()
    }
}
