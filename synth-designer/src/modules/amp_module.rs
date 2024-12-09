use super::gen_stack_program;
use crate::input_expr::*;
use crate::modules::*;
use crate::*;
use ini::Properties;
use proc_macro2::TokenStream;
use quote::quote;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;

const MODULE_TYPE: &str = "amplifier";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";
const LINEAR_CONTROL: &str = "linear_control";
const EXP_CONTROL: &str = "exp_control";
const SIGNAL_OUTPUT: &str = "signal_output";
const STATE_SIZE: usize = 1;
const INPUT_SIZE: usize = 3;

pub struct AmpModuleSpec {
    name: String,
    inputs: [Expr; INPUT_SIZE],
    state: [usize; STATE_SIZE],
}

impl AmpModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut signal_in: Expr = Expr::zero();
        let mut lin_control: Expr = Expr::zero();
        let mut exp_control: Expr = Expr::zero();

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                SIGNAL_INPUT => signal_in = Expr::parse(&v)?,
                LINEAR_CONTROL => lin_control = Expr::parse(&v)?,
                EXP_CONTROL => exp_control = Expr::parse(&v)?,
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        Ok(Self {
            name,
            inputs: [signal_in, lin_control, exp_control],
            state: [0; STATE_SIZE],
        })
    }
}

impl ModuleSpec for AmpModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let amplifier = Amplifier::new(
            self.inputs[0].compile(&synth_spec)?,
            self.state[0],
            self.inputs[1].compile(&synth_spec)?,
            self.inputs[2].compile(&synth_spec)?,
        );

        Ok(Box::new(amplifier))
    }

    fn codegen(&self, synth_spec: &SynthSpec) -> TokenStream {
        let s0 = self.state[0];
        let i0 = gen_stack_program(&self.inputs[0].compile(&synth_spec).unwrap());
        let i1 = gen_stack_program(&self.inputs[1].compile(&synth_spec).unwrap());
        let i2 = gen_stack_program(&self.inputs[2].compile(&synth_spec).unwrap());

        quote! { Amplifier::new(#i0, #s0, #i1, #i2) }
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
