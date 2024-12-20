use crate::codegen::Codegen;
use crate::input_expr::*;
use crate::modules::*;
use crate::synth_resource::SynthResource;
use crate::synth_spec::SynthSpec;
use ini::Properties;
use proc_macro2::TokenStream;
use quote::quote;
use synth_engine::modules::*;
use synth_engine::stack_program::StackProgram;

const MODULE_TYPE: &str = "contour";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";
const SIGNAL_OUTPUT: &str = "signal_output";
const RISE_CONTROL: &str = "rise_control";
const DECAY_CONTROL: &str = "decay_control";
const SHAPE_CONTROL: &str = "shape_control";
const INPUT_SIZE: usize = 4;
const STATE_SIZE: usize = 2;

pub struct ContourModuleSpec {
    name: String,
    inputs: [Expr; INPUT_SIZE],
    state: [usize; STATE_SIZE],
}

impl ContourModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut signal_in: Expr = Expr::zero();
        let mut rise_control: Expr = Expr::zero();
        let mut decay_control: Expr = Expr::zero();
        let mut shape_control: Expr = Expr::zero();

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                SIGNAL_INPUT => signal_in = Expr::parse(&v)?,
                RISE_CONTROL => rise_control = Expr::parse(&v)?,
                DECAY_CONTROL => decay_control = Expr::parse(&v)?,
                SHAPE_CONTROL => shape_control = Expr::parse(&v)?,
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        Ok(Self {
            name,
            inputs: [signal_in, rise_control, decay_control, shape_control],
            state: [0; STATE_SIZE],
        })
    }
}

impl ModuleSpec for ContourModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module<'a>(
        &self,
        synth_resource: &'a SynthResource,
    ) -> Result<SynthModule<'a>, ModuleError> {
        let env = Envelope::new(
            StackProgram::new_compute_stack_size(
                &synth_resource.get_code_buffer(self.get_name(), 0)?,
            ),
            StackProgram::new_compute_stack_size(
                &synth_resource.get_code_buffer(self.get_name(), 1)?,
            ),
            StackProgram::new_compute_stack_size(
                &synth_resource.get_code_buffer(self.get_name(), 2)?,
            ),
            StackProgram::new_compute_stack_size(
                &synth_resource.get_code_buffer(self.get_name(), 3)?,
            ),
            self.state[0],
            self.state[1],
        );

        Ok(SynthModule::Contour(env))
    }

    fn create_resources(
        &self,
        synth_spec: &SynthSpec,
        synth_resources: &mut SynthResource,
    ) -> Result<(), ModuleError> {
        let mut code_buffers = Vec::new();

        for input in &self.inputs {
            code_buffers.push(input.compile_to_instructions(synth_spec)?);
        }

        synth_resources.add_code_buffers(self.get_name(), code_buffers);

        Ok(())
    }

    fn codegen(&self, synth_spec: &SynthSpec, codegen: &mut Codegen) -> TokenStream {
        let i0 = codegen.add_stack_program(&self.inputs[0], &synth_spec);
        let i1 = codegen.add_stack_program(&self.inputs[0], &synth_spec);
        let i2 = codegen.add_stack_program(&self.inputs[0], &synth_spec);
        let i3 = codegen.add_stack_program(&self.inputs[0], &synth_spec);
        let s0 = self.state[0];
        let s1 = self.state[1];

        quote! { SynthModule::Contour(Envelope::new(#i0, #i1, #i2, #i3, #s0, #s1)) }
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
