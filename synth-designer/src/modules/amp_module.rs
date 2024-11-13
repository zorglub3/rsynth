use crate::modules::*;
use crate::*;
use ini::Properties;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;

const MODULE_TYPE: &str = "amplifier";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";
const LINEAR_CONTROL: &str = "linear_control";
const EXP_CONTROL: &str = "exp_control";
const SIGNAL_OUTPUT: &str = "signal_output";
const STATE_SIZE: usize = 1;
const INPUT_SIZE: usize = 3;

pub struct AmpModuleSpec {
    name: String,
    inputs: [InputSpec; INPUT_SIZE],
    state: [usize; STATE_SIZE],
}

impl AmpModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut signal_in: InputSpec = InputSpec::zero();
        let mut lin_control: InputSpec = InputSpec::zero();
        let mut exp_control: InputSpec = InputSpec::zero();

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                SIGNAL_INPUT => signal_in = InputSpec::parse(&v)?,
                LINEAR_CONTROL => lin_control = InputSpec::parse(&v)?,
                EXP_CONTROL => exp_control = InputSpec::parse(&v)?,
                _ => {
                    return Err(ModuleError::InvalidField {
                        module_type: MODULE_TYPE.to_string(),
                        field_name: k,
                    })
                }
            }
        }

        Ok(Self {
            name,
            inputs: [signal_in, lin_control, exp_control],
            state: [0; STATE_SIZE],
        })
    }
}

impl ModuleSpec for AmpModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let amplifier = Amplifier::new(
            synth_spec.input_expr(&self.inputs[0])?,
            self.state[0],
            synth_spec.input_expr(&self.inputs[1])?,
            synth_spec.input_expr(&self.inputs[2])?,
        );

        Ok(Box::new(amplifier))
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
