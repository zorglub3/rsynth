use crate::modules::*;
use crate::synth_spec::SynthSpec;
use ini::Properties;
use proc_macro2::TokenStream;
use quote::quote;
use synth_engine::modules::noise::A_PARAMETER_DEFAULT;
use synth_engine::modules::noise::B_PARAMETER_DEFAULT;
use synth_engine::modules::*;
use synth_engine::simulator::state::StateInput;
use core::ops::Range;

const MODULE_TYPE: &str = "noise";
const MODULE_NAME: &str = "name";
const SIGNAL_OUTPUT: &str = "signal_output";
const SEED: &str = "seed";
const PARAMETER_A: &str = "a";
const PARAMETER_B: &str = "b";

const STATE_SIZE: usize = 1;

pub struct NoiseGeneratorModuleSpec {
    name: String,
    a: u32,
    b: u32,
    seed: u32,
    state_range: Range<usize>,
    input_range: Range<usize>,
}

impl NoiseGeneratorModuleSpec {
    pub fn new(name: &str, out_index: usize) -> Self {
        Self {
            name: name.to_string(),
            a: A_PARAMETER_DEFAULT,
            b: B_PARAMETER_DEFAULT,
            seed: 1,
            state_range: 0..0,
            input_range: 0..0,
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
            a,
            b,
            seed,
            state_range: 0..0,
            input_range: 0..0,
        })
    }
}

impl ModuleSpec for NoiseGeneratorModuleSpec {
    fn allocate_state_input(&mut self, alloc: &mut StateAllocator) {
        let state_input_range = alloc.allocate(NoiseGenerator::STATE_SIZE, NoiseGenerator::INPUT_SIZE);
        self.state_range = state_input_range.state_range;
        self.input_range = state_input_range.input_range;
    }

    fn compile_input_exprs(
        &self,
        _synth_spec: &SynthSpec,
        _state_input: &mut StateInput,
    ) -> Result<(), ModuleError> {
        /* do nothing */
        Ok(())
    }

    fn make_module_entry(&self) -> ModuleEntry {
        let m = NoiseGenerator::new(self.a, self.b, self.seed);

        ModuleEntry {
            synth_module: SynthModule::Noise(m),
            state: self.state_range.clone(),
            input: self.input_range.clone(),
        }
    }
    /*
    fn create_module(&self, _synth_spec: &SynthSpec) -> Result<SynthModule, ModuleError> {
        let noise = NoiseGenerator::new(self.a, self.b, self.seed, self.state[0]);

        Ok(SynthModule::Noise(noise))
    }
    */

    fn codegen(&self, _synth_spec: &SynthSpec) -> TokenStream {
        /*
        let a = self.a;
        let b = self.b;
        let s = self.seed;
        let s0 = self.state[0];

        quote! { SynthModule::Noise(NoiseGenerator::new(#a, #b, #s, #s0)) }
        */
        todo!()
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            SIGNAL_OUTPUT => Ok(self.state_range.start + NoiseGenerator::SIGNAL_OUTPUT),
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
        NoiseGenerator::STATE_SIZE
    }

    fn input_size(&self) -> usize {
        NoiseGenerator::INPUT_SIZE
    }
}
