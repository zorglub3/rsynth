use crate::modules::*;
use crate::DEFAULT_FREQUENCY_ZERO;
use crate::*;
use ini::Properties;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;

const MODULE_TYPE: &str = "oscillator";
const MODULE_NAME: &str = "name";
const FREQUENCY_CONTROL: &str = "frequency_control";
const LINEAR_FREQUENCY_CONTROL: &str = "linear_frequency_control";
const PRESSURE_CONTROL: &str = "pressure_control";
const VELOCITY_CONTROL: &str = "velocity_control";
const SIGNAL_1_OUTPUT: &str = "signal1";
const SIGNAL_2_OUTPUT: &str = "signal2";
const FREQ0: &str = "frequency_zero";
const PARAM_A: &str = "param_a";
const PARAM_B: &str = "param_b";
const INPUT_SIZE: usize = 4;
const STATE_SIZE: usize = 2;

pub struct OscillatorModuleSpec {
    name: String,
    inputs: [InputSpec; INPUT_SIZE],
    state: [usize; STATE_SIZE],
    f0: f32,
    a: f32,
    b: f32,
}

impl OscillatorModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut fc: InputSpec = InputSpec::zero();
        let mut lc: InputSpec = InputSpec::zero();
        let mut pc: InputSpec = InputSpec::zero();
        let mut vc: InputSpec = InputSpec::zero();
        let mut f0: f32 = DEFAULT_FREQUENCY_ZERO;
        let mut a: f32 = 0.;
        let mut b: f32 = 0.;

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                FREQUENCY_CONTROL => fc = InputSpec::parse(&v)?,
                LINEAR_FREQUENCY_CONTROL => lc = InputSpec::parse(&v)?,
                PRESSURE_CONTROL => pc = InputSpec::parse(&v)?,
                VELOCITY_CONTROL => vc = InputSpec::parse(&v)?,
                FREQ0 => f0 = v.parse::<f32>()?,
                PARAM_A => a = v.parse::<f32>()?,
                PARAM_B => b = v.parse::<f32>()?,
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
            inputs: [fc, lc, pc, vc],
            state: [0; STATE_SIZE],
            f0,
            a,
            b,
        })
    }
}

impl ModuleSpec for OscillatorModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let osc = BowedOscillator::new(
            self.f0,
            self.a,
            self.b,
            self.state[0],
            self.state[1],
            synth_spec.input_expr(&self.inputs[0])?,
            synth_spec.input_expr(&self.inputs[1])?,
            synth_spec.input_expr(&self.inputs[2])?,
            synth_spec.input_expr(&self.inputs[3])?,
        );

        Ok(Box::new(osc))
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            SIGNAL_1_OUTPUT => Ok(self.state[0]),
            SIGNAL_2_OUTPUT => Ok(self.state[1]),
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
