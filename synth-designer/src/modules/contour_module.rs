use crate::modules::*;
use crate::*;
use ini::Properties;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;

const MODULE_TYPE: &str = "contour";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";
const SIGNAL_OUTPUT: &str = "signal_output";
const RISE_CONTROL: &str = "rise_control";
const DECAY_CONTROL: &str = "decay_control";
const INPUT_SIZE: usize = 3;
const STATE_SIZE: usize = 1;

pub struct ContourModuleSpec {
    name: String,
    inputs: [InputSpec; INPUT_SIZE],
    state: [usize; STATE_SIZE],
}

impl ContourModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let name = props.get(MODULE_NAME).ok_or(ModuleError::MissingField {
            module_type: MODULE_TYPE.to_string(),
            field_name: MODULE_NAME.to_string(),
        })?;

        Ok(Self {
            name: name.to_string(),
            inputs: [
                props
                    .get(SIGNAL_INPUT)
                    .map(InputSpec::parse)
                    .unwrap_or(Ok(InputSpec::zero()))?,
                props
                    .get(RISE_CONTROL)
                    .map(InputSpec::parse)
                    .unwrap_or(Ok(InputSpec::zero()))?,
                props
                    .get(DECAY_CONTROL)
                    .map(InputSpec::parse)
                    .unwrap_or(Ok(InputSpec::zero()))?,
            ],
            state: [0],
        })
    }
}

impl ModuleSpec for ContourModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let env = ADEnvelope::new(
            synth_spec.input_expr(&self.inputs[0])?,
            self.state[0],
            synth_spec.input_expr(&self.inputs[1])?,
            synth_spec.input_expr(&self.inputs[2])?,
        );

        Ok(Box::new(env))
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
