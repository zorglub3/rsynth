use crate::input_expr::*;
use crate::modules::*;
use crate::synth_spec::gen_stack_program;
use ini::Properties;
use proc_macro2::TokenStream;
use quote::quote;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;
use synth_engine::stack_program::StackProgram;
use crate::codegen::Codegen;
use crate::synth_resource::SynthResource;

const MODULE_TYPE: &str = "mono_output";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";
const OUTPUT_INDEX: &str = "output_index";
const INPUT_SIZE: usize = 1;

pub struct MonoOutputModuleSpec {
    name: String,
    output_index: usize,
    inputs: [Expr; INPUT_SIZE],
}

impl MonoOutputModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name = MODULE_TYPE.to_string();
        let mut output_index: usize = 0;
        let mut signal_in: Expr = Expr::zero();

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                SIGNAL_INPUT => signal_in = Expr::parse(&v)?,
                OUTPUT_INDEX => output_index = v.parse::<usize>()?,
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        Ok(Self {
            name,
            output_index,
            inputs: [signal_in],
        })
    }
}

impl ModuleSpec for MonoOutputModuleSpec {
    fn allocate_state(&mut self, _alloc: &mut StateAllocator) {
        /* do nothing */
    }

    fn create_module(&self, synth_spec: &SynthSpec, synth_resource: &SynthResource) -> Result<SynthModule, ModuleError> {
        let mono_output = MonoOutput::new(
            self.output_index, 
            StackProgram::new_compute_stack_size(&synth_resource.get_code_buffer(self.get_name(), 0)?));

        Ok(SynthModule::Output(mono_output))
    }

    fn create_resources(&self, synth_spec: &SynthSpec, synth_resources: &mut SynthResource) -> Result<(), ModuleError> {
        let code_buffers = vec![
            self.inputs[0].compile_to_instructions(synth_spec)?
        ];

        synth_resources.add_code_buffers(self.get_name(), code_buffers);

        Ok(())
    }

    fn codegen(&self, synth_spec: &SynthSpec, codegen: &mut Codegen) -> TokenStream {
        let oi = self.output_index;
        let i = codegen.add_stack_program(&self.inputs[0], &synth_spec);

        quote! { SynthModule::Output(MonoOutput::new(#oi, #i)) }
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
        0
    }
}
