use crate::modules::*;
use crate::*;
use ini::Properties;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;

const MODULE_TYPE: &str = "oscillator";
const MODULE_NAME: &str = "name";
const FREQUENCY_CONTROL: &str = "frequency_control";
const PRESSURE_CONTROL: &str = "pressure_control";
const VELOCITY_CONTROL: &str = "velocity_control";
const SIGNAL_1_OUTPUT: &str = "signal1";
const SIGNAL_2_OUTPUT: &str = "signal2";
const FREQ0: &str = "frequenzy_zero";
const PARAM_A: &str = "param_a";
const PARAM_B: &str = "param_b";
const INPUT_SIZE: usize = 3;
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
        let name = props.get(MODULE_NAME).ok_or(ModuleError::MissingField {
            module_type: MODULE_TYPE.to_string(),
            field_name: MODULE_NAME.to_string(),
        })?;

        Ok(Self {
            name: name.to_string(),
            inputs: [
                props
                    .get(FREQUENCY_CONTROL)
                    .map(parse_input_spec)
                    .unwrap_or(Ok(zero_input()))?,
                props
                    .get(PRESSURE_CONTROL)
                    .map(parse_input_spec)
                    .unwrap_or(Ok(zero_input()))?,
                props
                    .get(VELOCITY_CONTROL)
                    .map(parse_input_spec)
                    .unwrap_or(Ok(zero_input()))?,
            ],
            state: [0; 2],
            f0: props
                .get(FREQ0)
                .map(|s| s.parse::<f32>())
                .unwrap_or(Ok(1.0_f32))?,
            a: props
                .get(PARAM_A)
                .map(|s| s.parse::<f32>())
                .unwrap_or(Ok(1.0_f32))?,
            b: props
                .get(PARAM_B)
                .map(|s| s.parse::<f32>())
                .unwrap_or(Ok(1.0_f32))?,
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
            synth_spec.input_state_index(&self.inputs[0])?,
            synth_spec.input_state_index(&self.inputs[1])?,
            synth_spec.input_state_index(&self.inputs[2])?,
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
