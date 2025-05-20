use crate::input_expr::*;
use crate::modules::*;
use crate::synth_spec::gen_stack_program;
use ini::Properties;
use proc_macro2::TokenStream;
use quote::quote;
use synth_engine::modules::*;
use synth_engine::simulator::state::StateInput;
use core::ops::Range;

const MODULE_TYPE: &str = "mono_output";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";
const OUTPUT_INDEX: &str = "output_index";

pub struct MonoOutputModuleSpec {
    name: String,
    output_index: usize,
    inputs: Vec<Expr>,
    state_range: Range<usize>,
    input_range: Range<usize>,
}

impl MonoOutputModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name = MODULE_TYPE.to_string();
        let mut output_index: usize = 0;
        // let mut signal_in: Expr = Expr::zero();
        let mut inputs = vec![Expr::zero(); MonoOutput::INPUT_SIZE];

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                SIGNAL_INPUT => inputs[MonoOutput::SIGNAL_INPUT] = Expr::parse(&v)?,
                OUTPUT_INDEX => output_index = v.parse::<usize>()?,
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        Ok(Self {
            name,
            output_index,
            inputs,
            state_range: 0..0,
            input_range: 0..0,
        })
    }
}

impl ModuleSpec for MonoOutputModuleSpec {
    fn allocate_state_input(&mut self, alloc: &mut StateAllocator) {
        let state_input_range = alloc.allocate(MonoOutput::STATE_SIZE, MonoOutput::INPUT_SIZE);
        self.state_range = state_input_range.state_range;
        self.input_range = state_input_range.input_range;
    }

    fn compile_input_exprs(
        &self,
        synth_spec: &SynthSpec,
        state_input: &mut StateInput,
    ) -> Result<(), ModuleError> {
        for i in 0..MonoOutput::INPUT_SIZE {
            let program = self.inputs[i].compile(synth_spec)?;
            let index = self.input_range.start + i;
            state_input.set_program(index, program);
        }

        Ok(())
    }

    fn make_module_entry(&self) -> ModuleEntry {
        let m = MonoOutput::new(self.output_index);

        ModuleEntry {
            synth_module: SynthModule::Output(m),
            state: self.state_range.clone(),
            input: self.input_range.clone(),
        }
    }

    /*
    fn create_module(&self, synth_spec: &SynthSpec) -> Result<SynthModule, ModuleError> {
        let mono_output = MonoOutput::new(self.output_index, self.inputs[0].compile(&synth_spec)?);

        Ok(SynthModule::Output(mono_output))
    }
    */

    fn codegen(&self, synth_spec: &SynthSpec) -> TokenStream {
        /*
        let oi = self.output_index;
        let i = gen_stack_program(&self.inputs[0].compile(&synth_spec).unwrap());

        quote! { SynthModule::Output(MonoOutput::new(#oi, #i)) }
        */
        todo!()
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        // One happy day even this module might have outputs :-P
        match state_field {
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
        MonoOutput::STATE_SIZE
    }

    fn input_size(&self) -> usize {
        MonoOutput::INPUT_SIZE
    }
}
