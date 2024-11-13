use crate::modules::*;
use crate::*;
use ini::Properties;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;

const MODULE_TYPE: &str = "saw_oscillator";
const MODULE_NAME: &str = "name";
const FREQUENCY_CONTROL: &str = "frequency_control";
const FREQUENCY_ZERO: &str = "frequency_zero";
const LINEAR_CONTROL: &str = "linear_modulation";
const SIGNAL_OUTPUT: &str = "signal_output";
const INPUT_SIZE: usize = 2;
const STATE_SIZE: usize = 2;

pub struct SawOscillatorModuleSpec {
    name: String,
    inputs: [InputSpec; INPUT_SIZE],
    state: [usize; STATE_SIZE],
    f0: f32,
}

impl SawOscillatorModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: Option<String> = None;
        let mut f0: f32 = 1.;
        let mut fc: Option<InputSpec> = None;
        let mut lc: Option<InputSpec> = None;

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = Some(v.to_string()),
                FREQUENCY_CONTROL => fc = Some(InputSpec::parse(&v)?),
                LINEAR_CONTROL => lc = Some(InputSpec::parse(&v)?),
                FREQUENCY_ZERO => f0 = v.parse::<f32>()?,
                _ => {
                    return Err(ModuleError::InvalidField {
                        module_type: MODULE_TYPE.to_string(),
                        field_name: k,
                    })
                }
            }
        }

        Ok(Self {
            name: name.ok_or(ModuleError::MissingField {
                module_type: MODULE_TYPE.to_string(),
                field_name: MODULE_NAME.to_string(),
            })?,
            inputs: [
                fc.unwrap_or(InputSpec::zero()),
                lc.unwrap_or(InputSpec::zero()),
            ],
            state: [0; STATE_SIZE],
            f0,
        })
    }
}

impl ModuleSpec for SawOscillatorModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let osc = SawOscillator::new(
            self.f0,
            self.state[0],
            self.state[1],
            synth_spec.input_expr(&self.inputs[0])?,
            synth_spec.input_expr(&self.inputs[1])?,
        );

        Ok(Box::new(osc))
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            SIGNAL_OUTPUT => Ok(self.state[1]),
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