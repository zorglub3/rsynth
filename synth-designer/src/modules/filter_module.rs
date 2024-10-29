use synth_engine::modules::*;
use synth_engine::simulator::module::Module;
use crate::*;
use ini::Properties;
use crate::modules::*;

const MODULE_TYPE: &str = "lowpass_filter";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";
const FREQ0: &str = "frequency_zero";
const CUTOFF_CONTROL: &str = "cutoff_frequency";
const RESONANCE_CONTROL: &str = "resonance";
const SIGNAL_OUTPUT: &str = "signal_output";
const INPUT_SIZE: usize = 3;
const STATE_SIZE: usize = 4;

pub struct FilterModuleSpec {
    name: String,
    inputs: [InputSpec; INPUT_SIZE],
    state: [usize; STATE_SIZE],
    f0: f32,
}

impl FilterModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let name =
            props.get(MODULE_NAME)
                .ok_or(ModuleError::MissingField {
                    module_type: MODULE_TYPE.to_string(),
                    field_name: MODULE_NAME.to_string(),
                })?;

        Ok(Self {
            name: name.to_string(),
            inputs: [
                props.get(SIGNAL_INPUT).map(parse_input_spec).unwrap_or(Ok(zero_input()))?,
                props.get(CUTOFF_CONTROL).map(parse_input_spec).unwrap_or(Ok(zero_input()))?,
                props.get(RESONANCE_CONTROL).map(parse_input_spec).unwrap_or(Ok(zero_input()))?,
            ],
            state: [0; 4],
            f0: props.get(FREQ0).map(|s| s.parse::<f32>()).unwrap_or(Ok(1.0_f32))?,
        })
    }
}

impl ModuleSpec for FilterModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let filter = MoogFilter::new(
            self.f0,
            self.state[0],
            self.state[1],
            self.state[2],
            self.state[3],
            synth_spec.input_state_index(&self.inputs[1])?,
            synth_spec.input_state_index(&self.inputs[2])?,
            synth_spec.input_state_index(&self.inputs[0])?,
        );

        Ok(Box::new(filter))
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            SIGNAL_OUTPUT => Ok(self.state[3]),
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
