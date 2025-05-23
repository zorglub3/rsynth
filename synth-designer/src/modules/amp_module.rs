use crate::input_expr::*;
use crate::modules::*;
use crate::synth_spec::gen_stack_program;
use crate::synth_spec::SynthSpec;
use ini::Properties;
use proc_macro2::TokenStream;
use quote::quote;
use synth_engine::modules::*;
use synth_engine::simulator::state::StateInput;
use core::ops::Range;

const MODULE_TYPE: &str = "amplifier";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";
const LINEAR_CONTROL: &str = "linear_control";
const EXP_CONTROL: &str = "exp_control";
const SIGNAL_OUTPUT: &str = "signal_output";

pub struct AmpModuleSpec {
    name: String,
    inputs: Vec<Expr>,
    state_range: Range<usize>,
    input_range: Range<usize>,
}

impl AmpModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut inputs = vec![Expr::zero().clone(); Amplifier::INPUT_SIZE];

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                SIGNAL_INPUT => inputs[Amplifier::SIGNAL_INPUT] = Expr::parse(&v)?,
                LINEAR_CONTROL => inputs[Amplifier::LINEAR_CONTROL_INPUT] = Expr::parse(&v)?,
                EXP_CONTROL => inputs[Amplifier::EXP_CONTROL_INPUT] = Expr::parse(&v)?,
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        Ok(Self {
            name,
            inputs,
            state_range: 0..0,
            input_range: 0..0,
        })
    }
}

impl ModuleSpec for AmpModuleSpec {
    fn allocate_state_input(&mut self, alloc: &mut StateAllocator) {
        let state_input_range = alloc.allocate(Amplifier::STATE_SIZE, Amplifier::INPUT_SIZE);
        self.state_range = state_input_range.state_range;
        self.input_range = state_input_range.input_range;
    }

    fn compile_input_exprs(
        &self, 
        synth_spec: &SynthSpec, 
        state_input: &mut StateInput,
    ) -> Result<(), ModuleError> {
        for i in 0..Amplifier::INPUT_SIZE {
            let program = self.inputs[i].compile(synth_spec)?;
            let index = i + self.input_range.start;
            state_input.set_program(index, program);
        }

        Ok(())
    }

    fn make_module_entry(&self) -> ModuleEntry {
        let min = (-5.0_f32).exp2();
        let scale = 1. / (1. - min);

        let amplifier = Amplifier::new(min, scale);

        ModuleEntry {
            synth_module: SynthModule::Amp(amplifier),
            input: self.input_range.clone(),
            state: self.state_range.clone(),
        }
    }

    fn codegen(&self, synth_spec: &SynthSpec) -> TokenStream {
        let min = -5.0_f32.exp2();
        let scale = 1. / (1. - min);

        todo!("should include state- and input- ranges somehow")
        // quote! { SynthModule::Amp(Ampflifier::new(#min, #scale)) }

        /*
        let s0 = self.state[0];
        let i0 = gen_stack_program(&self.inputs[0].compile(&synth_spec).unwrap());
        let i1 = gen_stack_program(&self.inputs[1].compile(&synth_spec).unwrap());
        let i2 = gen_stack_program(&self.inputs[2].compile(&synth_spec).unwrap());

        quote! { SynthModule::Amp(Amplifier::new(#i0, #s0, #i1, #i2)) }
        */
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            SIGNAL_OUTPUT => Ok(self.state_range.start + Amplifier::SIGNAL_OUTPUT),
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
        Amplifier::STATE_SIZE
    }

    fn input_size(&self) -> usize {
        Amplifier::INPUT_SIZE
    }
}
