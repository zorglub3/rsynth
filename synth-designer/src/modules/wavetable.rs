use crate::modules::*;
use crate::DEFAULT_FREQUENCY_ZERO;
use crate::*;
use hound;
use ini::Properties;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;

const MODULE_TYPE: &str = "wavetable_oscillator";
const MODULE_NAME: &str = "name";
const FREQUENCY_CONTROL: &str = "frequency_control";
const FREQUENCY_ZERO: &str = "frequency_zero";
const LINEAR_CONTROL: &str = "linear_modulation";
const SCAN_CONTROL: &str = "scan_control";
const WAVETABLE_FIELD: &str = "wavetable";
const SIGNAL_OUTPUT: &str = "signal_output";
const INPUT_SIZE: usize = 3;
const STATE_SIZE: usize = 2;

pub struct WavetableOscillatorModuleSpec {
    name: String,
    inputs: [InputSpec; INPUT_SIZE],
    state: [usize; STATE_SIZE],
    f0: f32,
    wavetables: Vec<Vec<f32>>,
}

fn load_wavetable(filename: &str) -> Result<Vec<f32>, ModuleError> {
    let mut reader = hound::WavReader::open(filename)?;
    let mut result: Vec<f32> = Vec::new();

    for sample in reader.samples::<i16>() {
        let sample = ((sample?) as f32) / (i16::MAX as f32);
        result.push(sample);
    }

    Ok(result)
}

impl WavetableOscillatorModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut f0: f32 = DEFAULT_FREQUENCY_ZERO;
        let mut fc: InputSpec = InputSpec::zero();
        let mut lc: InputSpec = InputSpec::zero();
        let mut sc: InputSpec = InputSpec::zero();
        let mut wavetables: Vec<Vec<f32>> = Vec::new();

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                FREQUENCY_CONTROL => fc = InputSpec::parse(&v)?,
                LINEAR_CONTROL => lc = InputSpec::parse(&v)?,
                SCAN_CONTROL => sc = InputSpec::parse(&v)?,
                FREQUENCY_ZERO => f0 = v.parse::<f32>()?,
                WAVETABLE_FIELD => wavetables.push(load_wavetable(&v)?),
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        if wavetables.len() == 0 {
            Err(ModuleError::MissingField(
                MODULE_TYPE.to_string(),
                WAVETABLE_FIELD.to_string(),
            ))
        } else {
            Ok(Self {
                name,
                inputs: [fc, lc, sc],
                state: [0; STATE_SIZE],
                f0,
                wavetables,
            })
        }
    }
}

impl ModuleSpec for WavetableOscillatorModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let module = Wavetable::new(
            self.f0,
            self.state[0],
            self.state[1],
            synth_spec.input_expr(&self.inputs[0])?,
            synth_spec.input_expr(&self.inputs[1])?,
            synth_spec.input_expr(&self.inputs[2])?,
            self.wavetables.clone(),
        );

        Ok(Box::new(module))
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            SIGNAL_OUTPUT => Ok(self.state[1]),
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
