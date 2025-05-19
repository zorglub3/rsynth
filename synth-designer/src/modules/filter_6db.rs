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

const MODULE_TYPE: &str = "lowpass_filter_6db";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";
const FREQ0: &str = "frequency_zero";
const CUTOFF_CONTROL: &str = "cutoff_frequency";
const LINEAR_CONTROL: &str = "linear_control";
const LOWPASS_OUTPUT: &str = "lowpass_output";
const HIGHPASS_OUTPUT: &str = "highpass_output";

pub struct Filter6dbModuleSpec {
    name: String,
    inputs: [Expr; Filter6db::INPUT_SIZE],
    f0: f32,
    state_range: Range<usize>,
    input_range: Range<usize>,
}

impl Filter6dbModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut inputs = vec![Expr::zero(); Filter6db::INPUT_SIZE];
        // let mut signal_in: Expr = Expr::zero();
        // let mut fc: Expr = Expr::zero();
        // let mut lc: Expr = Expr::zero();
        let mut f0: f32 = 1.0;

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                SIGNAL_INPUT => inputs[Filter6db::SIGNAL_INPUT] = Expr::parse(&v)?,
                CUTOFF_CONTROL => inputs[Filter6db::EXP_CONTROL_INPUT]  = Expr::parse(&v)?,
                LINEAR_CONTROL => inputs[Filter6db::LINEAR_CONTROL_INPUT] = Expr::parse(&v)?,
                FREQ0 => f0 = v.parse::<f32>()?,
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        Ok(Self {
            name,
            inputs,
            f0,
            state_range: 0..0,
            input_range: 0..0,
        })
    }
}

impl ModuleSpec for Filter6dbModuleSpec {
    fn allocate_state_input(&mut self, alloc: &mut StateAllocator) {
        let state_input_range = alloc.allocate(Filter6db::STATE_SIZE, Filter6db::INPUT_SIZE);
        self.state_range = state_input_range.state_range;
        self.input_range = state_input_range.input_range;
        // alloc.allocate(&mut self.state);
    }

    fn compile_input_exprs(
        &self,
        synth_spec: &SynthSpec,
        state_input: &mut StateInput,
    ) -> Result<(), ModuleError> {
        for i in 0..Filter6db::INPUT_SIZE {
            let program = self.inputs[i].compile(synth_spec)?;
            let index = i + self.input_range.start;
            state_input.set_program(index, program);
        }

        Ok(())
    }

    fn make_module_entry(&self) -> ModuleEntry {
        let m = Filter6db::new(self.f0);

        ModuleEntry {
            synth_module: SynthModule::Filter1Pole(m),
            state: self.state_range.clone(),
            input: self.input_range.clone(),
        }
    }
    /*
    fn create_module(&self, synth_spec: &SynthSpec) -> Result<SynthModule, ModuleError> {
        let filter = Filter6db::new(
            self.f0,
            self.state[2],
            self.state[0],
            self.state[1],
            self.inputs[0].compile(&synth_spec)?,
            self.inputs[1].compile(&synth_spec)?,
            self.inputs[2].compile(&synth_spec)?,
        );

        Ok(SynthModule::Filter1Pole(filter))
    }
    */

    fn codegen(&self, synth_spec: &SynthSpec) -> TokenStream {
        /*
        let f0 = self.f0;
        let s0 = self.state[0];
        let s1 = self.state[1];
        let s2 = self.state[2];
        let i0 = gen_stack_program(&self.inputs[0].compile(&synth_spec).unwrap());
        let i1 = gen_stack_program(&self.inputs[1].compile(&synth_spec).unwrap());
        let i2 = gen_stack_program(&self.inputs[2].compile(&synth_spec).unwrap());

        quote! { SynthModule::Filter1Pole(Filter6db::new(#f0, #s2, #s0, #s1, #i0, #i1, #i2)) }
        */
        todo!()
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            LOWPASS_OUTPUT => Ok(self.state_range.start + Filter6db::LOWPASS_OUTPUT),
            HIGHPASS_OUTPUT => Ok(self.state_range.start + Filter6db::HIGHPASS_OUTPUT),
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
        Filter6db::STATE_SIZE
    }

    fn input_size(&self) -> usize {
        Filter6db::INPUT_SIZE
    }
}
