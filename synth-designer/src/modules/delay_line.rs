use crate::modules::*;
use crate::*;
use ini::Properties;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;

const MODULE_TYPE: &str = "delay_line";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";
const FREQUENCY_ZERO: &str = "frequency_zero";
const FREQUENCY_CONTROL: &str = "pitch_control";
const LINEAR_CONTROL: &str = "linear_control";
const SIGNAL_OUTPUT: &str = "signal_output";
const DATA_SIZE_FIELD: &str = "data_size";

const INPUT_SIZE: usize = 3;
const STATE_SIZE: usize = 1;
const DATA_SIZE_VALUE: usize = 1024;

pub struct DelayLineModuleSpec {
    name: String,
    inputs: [InputSpec; INPUT_SIZE],
    f0: f32,
    state: [usize; STATE_SIZE],
    data_size: usize,
}

impl DelayLineModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut f0: f32 = 1.;
        let mut fc: InputSpec = InputSpec::zero();
        let mut lc: InputSpec = InputSpec::zero();
        let mut input: InputSpec = InputSpec::zero();
        let mut data_size: usize = DATA_SIZE_VALUE;

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                SIGNAL_INPUT => input = InputSpec::parse(&v)?,
                FREQUENCY_ZERO => f0 = v.parse::<f32>()?,
                FREQUENCY_CONTROL => fc = InputSpec::parse(&v)?,
                LINEAR_CONTROL => lc = InputSpec::parse(&v)?,
                DATA_SIZE_FIELD => data_size = v.parse::<usize>()?,
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
            inputs: [input, fc, lc],
            f0,
            state: [0; STATE_SIZE],
            data_size,
        })
    }
}

impl ModuleSpec for DelayLineModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let delay_line = DelayLine::new(
            self.f0,
            self.state[0],
            synth_spec.input_expr(&self.inputs[0])?,
            synth_spec.input_expr(&self.inputs[1])?,
            synth_spec.input_expr(&self.inputs[2])?,
            self.data_size,
        );

        Ok(Box::new(delay_line))
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
