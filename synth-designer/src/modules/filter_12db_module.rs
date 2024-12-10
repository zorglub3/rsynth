use crate::input_expr::*;
use crate::modules::*;
use crate::synth_spec::gen_stack_program;
use crate::synth_spec::SynthSpec;
use ini::Properties;
use proc_macro2::TokenStream;
use quote::quote;
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
    inputs: [Expr; INPUT_SIZE],
    state: [usize; STATE_SIZE],
    f0: f32,
}

impl Filter12dbModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut f0: f32 = 1.;
        let mut fc: Option<Expr> = None;
        let mut lc: Option<Expr> = None;
        let mut rc: Option<Expr> = None;
        let mut input: Option<Expr> = None;

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                SIGNAL_INPUT => input = Some(Expr::parse(&v)?),
                CUTOFF_CONTROL => fc = Some(Expr::parse(&v)?),
                RESONANCE_CONTROL => rc = Some(Expr::parse(&v)?),
                LINEAR_CONTROL => lc = Some(Expr::parse(&v)?),
                FREQUENCY_ZERO => f0 = v.parse::<f32>()?,
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        Ok(Self {
            name,
            f0,
            inputs: [
                input.unwrap_or(Expr::zero()),
                fc.unwrap_or(Expr::zero()),
                lc.unwrap_or(Expr::zero()),
                rc.unwrap_or(Expr::zero()),
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
            self.inputs[1].compile(&synth_spec)?,
            self.inputs[2].compile(&synth_spec)?,
            self.inputs[3].compile(&synth_spec)?,
            self.inputs[0].compile(&synth_spec)?,
        );

        Ok(Box::new(filter))
    }

    fn codegen(&self, synth_spec: &SynthSpec) -> TokenStream {
        let f0 = self.f0;
        let s0 = self.state[0];
        let s1 = self.state[1];
        let s2 = self.state[2];
        let i0 = gen_stack_program(&self.inputs[0].compile(&synth_spec).unwrap());
        let i1 = gen_stack_program(&self.inputs[1].compile(&synth_spec).unwrap());
        let i2 = gen_stack_program(&self.inputs[2].compile(&synth_spec).unwrap());
        let i3 = gen_stack_program(&self.inputs[3].compile(&synth_spec).unwrap());

        quote! { SynthModule::Filter2Pole(Filter12db::new(#f0, #s0, #s1, #s2, #i1, #i2, #i3, #i0)) }
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
