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
        let mut name: String = MODULE_TYPE.to_string();
        let mut signal_in: InputSpec = InputSpec::zero();
        let mut rise_control: InputSpec = InputSpec::zero();
        let mut decay_control: InputSpec = InputSpec::zero();

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                SIGNAL_INPUT => signal_in = InputSpec::parse(&v)?,
                RISE_CONTROL => rise_control = InputSpec::parse(&v)?,
                DECAY_CONTROL => decay_control = InputSpec::parse(&v)?,
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        Ok(Self {
            name,
            inputs: [signal_in, rise_control, decay_control],
            state: [0; STATE_SIZE],
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
            _ => Err(ModuleError::MissingStateName(
                MODULE_TYPE.to_string(),
                self.name.clone(),
                state_field.to_string(),
            )),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn state_size(&self) -> usize {
        self.state.len()
    }
}
