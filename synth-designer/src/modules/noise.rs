use crate::modules::*;
use crate::*;
use ini::Properties;
use synth_engine::modules::noise::A_PARAMETER_DEFAULT;
use synth_engine::modules::noise::B_PARAMETER_DEFAULT;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;

const MODULE_TYPE: &str = "noise";
const MODULE_NAME: &str = "name";
const SIGNAL_OUTPUT: &str = "signal_output";
const SEED: &str = "seed";
const PARAMETER_A: &str = "a";
const PARAMETER_B: &str = "b";

const STATE_SIZE: usize = 1;

pub struct NoiseGeneratorModuleSpec {
    name: String,
    state: [usize; STATE_SIZE],
    a: u32,
    b: u32,
    seed: u32,
}

impl NoiseGeneratorModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut a: u32 = A_PARAMETER_DEFAULT;
        let mut b: u32 = B_PARAMETER_DEFAULT;
        let mut seed: u32 = 1;

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                PARAMETER_A => a = v.parse::<u32>()?,
                PARAMETER_B => b = v.parse::<u32>()?,
                SEED => seed = v.parse::<u32>()?,
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
            state: [0; STATE_SIZE],
            a,
            b,
            seed,
        })
    }
}

impl ModuleSpec for NoiseGeneratorModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, _synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let noise = NoiseGenerator::new(self.a, self.b, self.seed, self.state[0]);

        Ok(Box::new(noise))
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
