use crate::modules::*;
use crate::*;
use ini::Properties;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;

const MODULE_TYPE: &str = "lowpass_filter_24db";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";
const FREQ0: &str = "frequency_zero";
const CUTOFF_CONTROL: &str = "cutoff_frequency";
const LINEAR_CONTROL: &str = "linear_frequency";
const RESONANCE_CONTROL: &str = "resonance";
const LOWPASS_OUTPUT: &str = "lowpass_output";
const HIGHPASS_OUTPUT: &str = "highpass_output";
const INPUT_SIZE: usize = 4;
const STATE_SIZE: usize = 4;

pub struct Filter24dbModuleSpec {
    name: String,
    inputs: [InputSpec; INPUT_SIZE],
    state: [usize; STATE_SIZE],
    f0: f32,
}

impl Filter24dbModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut signal_in: InputSpec = InputSpec::zero();
        let mut fc: InputSpec = InputSpec::zero();
        let mut lc: InputSpec = InputSpec::zero();
        let mut rc: InputSpec = InputSpec::zero();
        let mut f0: f32 = 1.0_f32;

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                SIGNAL_INPUT => signal_in = InputSpec::parse(&v)?,
                CUTOFF_CONTROL => fc = InputSpec::parse(&v)?,
                LINEAR_CONTROL => lc = InputSpec::parse(&v)?,
                RESONANCE_CONTROL => rc = InputSpec::parse(&v)?,
                FREQ0 => f0 = v.parse::<f32>()?,
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
            inputs: [signal_in, fc, rc, lc],
            state: [0; 4],
            f0,
        })
    }
}

impl ModuleSpec for Filter24dbModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let filter = Filter24db::new(
            self.f0,
            self.state[0],
            self.state[1],
            self.state[2],
            self.state[3],
            synth_spec.input_expr(&self.inputs[1])?,
            synth_spec.input_expr(&self.inputs[3])?,
            synth_spec.input_expr(&self.inputs[2])?,
            synth_spec.input_expr(&self.inputs[0])?,
        );

        Ok(Box::new(filter))
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            LOWPASS_OUTPUT => Ok(self.state[3]),
            HIGHPASS_OUTPUT => Ok(self.state[0]),
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
