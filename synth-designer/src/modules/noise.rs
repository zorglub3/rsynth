use crate::modules::*;
use crate::synth_spec::SynthSpec;
use ini::Properties;
use proc_macro2::TokenStream;
use quote::quote;
use synth_engine::modules::noise::A_PARAMETER_DEFAULT;
use synth_engine::modules::noise::B_PARAMETER_DEFAULT;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;
use crate::codegen::Codegen;

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
    pub fn new(name: &str, out_index: usize) -> Self {
        Self {
            name: name.to_string(),
            state: [out_index],
            a: A_PARAMETER_DEFAULT,
            b: B_PARAMETER_DEFAULT,
            seed: 1,
        }
    }

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
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
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

    fn codegen(&self, _synth_spec: &SynthSpec, _codegen: &mut Codegen) -> TokenStream {
        let a = self.a;
        let b = self.b;
        let s = self.seed;
        let s0 = self.state[0];

        quote! { SynthModule::Noise(NoiseGenerator::new(#a, #b, #s, #s0)) }
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
