use crate::modules::*;
use crate::*;
use ini::Properties;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;

const MODULE_TYPE: &str = "filter_12db";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";
const FREQUENCY_ZERO: &str = "frequency_zero";
const CUTOFF_CONTROL: &str = "cutoff_frequency";
const RESONANCE_CONTROL: &str = "resonance";
const LINEAR_CONTROL: &str = "linear_control";
const LP_OUTPUT: &str = "lowpass_output";
const BP_OUTPUT: &str = "bandpass_output";
const HP_OUTPUT: &str = "highpass_output";

const INPUT_SIZE: usize = 4;
const STATE_SIZE: usize = 3;

pub struct Filter12dbModuleSpec {
    name: String,
    inputs: [InputSpec; INPUT_SIZE],
    state: [usize; STATE_SIZE],
    f0: f32,
}

impl Filter12dbModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut f0: f32 = 1.;
        let mut fc: Option<InputSpec> = None;
        let mut lc: Option<InputSpec> = None;
        let mut rc: Option<InputSpec> = None;
        let mut input: Option<InputSpec> = None;

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                SIGNAL_INPUT => input = Some(InputSpec::parse(&v)?),
                CUTOFF_CONTROL => fc = Some(InputSpec::parse(&v)?),
                RESONANCE_CONTROL => rc = Some(InputSpec::parse(&v)?),
                LINEAR_CONTROL => lc = Some(InputSpec::parse(&v)?),
                FREQUENCY_ZERO => f0 = v.parse::<f32>()?,
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        Ok(Self {
            name,
            f0,
            inputs: [
                input.unwrap_or(InputSpec::zero()),
                fc.unwrap_or(InputSpec::zero()),
                lc.unwrap_or(InputSpec::zero()),
                rc.unwrap_or(InputSpec::zero()),
            ],
            state: [0; STATE_SIZE],
        })
    }
}

impl ModuleSpec for Filter12dbModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let filter = Filter12db::new(
            self.f0,
            self.state[0],
            self.state[1],
            self.state[2],
            synth_spec.input_expr(&self.inputs[1])?,
            synth_spec.input_expr(&self.inputs[2])?,
            synth_spec.input_expr(&self.inputs[3])?,
            synth_spec.input_expr(&self.inputs[0])?,
        );

        Ok(Box::new(filter))
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            HP_OUTPUT => Ok(self.state[0]),
            BP_OUTPUT => Ok(self.state[1]),
            LP_OUTPUT => Ok(self.state[2]),
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
