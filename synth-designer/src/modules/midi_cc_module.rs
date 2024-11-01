use crate::modules::*;
use crate::*;
use ini::Properties;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;

const MODULE_TYPE: &str = "midi_cc";
const MODULE_NAME: &str = "name";
const CHANNEL: &str = "channel";
const CONTROL: &str = "control";
const SIGNAL_OUTPUT: &str = "signal_output";
const MIN_VALUE: &str = "min_value";
const MAX_VALUE: &str = "max_value";
const STATE_SIZE: usize = 1;

pub struct MidiCCModuleSpec {
    name: String,
    channel: u8,
    control: u8,
    state: [usize; STATE_SIZE],
    min_value: f32,
    max_value: f32,
}

impl MidiCCModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let name = props.get(MODULE_NAME).ok_or(ModuleError::MissingField {
            module_type: MODULE_TYPE.to_string(),
            field_name: MODULE_NAME.to_string(),
        })?;

        Ok(Self {
            name: name.to_string(),
            channel: props
                .get(CHANNEL)
                .map(|s| s.parse::<u8>())
                .unwrap_or(Ok(1_u8))?,
            control: props
                .get(CONTROL)
                .map(|s| s.parse::<u8>())
                .unwrap_or(Ok(1_u8))?,
            state: [0; 1],
            min_value: props
                .get(MIN_VALUE)
                .map(|s| s.parse::<f32>())
                .unwrap_or(Ok(0.))?,
            max_value: props
                .get(MAX_VALUE)
                .map(|s| s.parse::<f32>())
                .unwrap_or(Ok(1.))?,
        })
    }
}

impl ModuleSpec for MidiCCModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, _synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let midi_cc = MidiCC::new(
            self.state[0],
            self.control,
            self.channel,
            self.min_value,
            self.max_value,
        );

        Ok(Box::new(midi_cc))
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            SIGNAL_OUTPUT => Ok(self.state[0]),
            _ => Err(ModuleError::MissingStateName {
                module_type: MODULE_TYPE.to_string(),
                module_name: self.name.clone(),
                field_name: state_field.to_string(),
            }),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn state_size(&self) -> usize {
        self.state.len()
    }
}
