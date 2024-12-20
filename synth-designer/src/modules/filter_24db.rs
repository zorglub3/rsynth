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

const MODULE_TYPE: &str = "lowpass_filter_24db";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";
const FREQ0: &str = "frequency_zero";
const CUTOFF_CONTROL: &str = "cutoff_frequency";
const LINEAR_CONTROL: &str = "linear_frequency";
const RESONANCE_CONTROL: &str = "resonance";
const LOWPASS_OUTPUT: &str = "lowpass_output";
const HIGHPASS_OUTPUT: &str = "highpass_output";
const INPUT_SIZE: usize = 4;
const STATE_SIZE: usize = 4;

pub struct Filter24dbModuleSpec {
    name: String,
    inputs: [Expr; INPUT_SIZE],
    state: [usize; STATE_SIZE],
    f0: f32,
}

impl Filter24dbModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut signal_in: Expr = Expr::zero();
        let mut fc: Expr = Expr::zero();
        let mut lc: Expr = Expr::zero();
        let mut rc: Expr = Expr::zero();
        let mut f0: f32 = 1.0_f32;

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                SIGNAL_INPUT => signal_in = Expr::parse(&v)?,
                CUTOFF_CONTROL => fc = Expr::parse(&v)?,
                LINEAR_CONTROL => lc = Expr::parse(&v)?,
                RESONANCE_CONTROL => rc = Expr::parse(&v)?,
                FREQ0 => f0 = v.parse::<f32>()?,
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        Ok(Self {
            name,
            inputs: [signal_in, fc, rc, lc],
            state: [0; 4],
            f0,
        })
    }
}

impl ModuleSpec for Filter24dbModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module<'a>(
        &self,
        synth_resource: &'a SynthResource,
    ) -> Result<SynthModule<'a>, ModuleError> {
        let filter = Filter24db::new(
            self.f0,
            self.state[0],
            self.state[1],
            self.state[2],
            self.state[3],
            StackProgram::new_compute_stack_size(
                &synth_resource.get_code_buffer(self.get_name(), 1)?,
            ),
            StackProgram::new_compute_stack_size(
                &synth_resource.get_code_buffer(self.get_name(), 3)?,
            ),
            StackProgram::new_compute_stack_size(
                &synth_resource.get_code_buffer(self.get_name(), 2)?,
            ),
            StackProgram::new_compute_stack_size(
                &synth_resource.get_code_buffer(self.get_name(), 0)?,
            ),
        );

        Ok(SynthModule::Filter4Pole(filter))
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
        let f0 = self.f0;
        let s0 = self.state[0];
        let s1 = self.state[1];
        let s2 = self.state[2];
        let s3 = self.state[3];
        let i0 = codegen.add_stack_program(&self.inputs[0], &synth_spec);
        let i1 = codegen.add_stack_program(&self.inputs[1], &synth_spec);
        let i2 = codegen.add_stack_program(&self.inputs[2], &synth_spec);
        let i3 = codegen.add_stack_program(&self.inputs[3], &synth_spec);

        quote! { SynthModule::Filter4Pole(Filter24db::new( #f0, #s0, #s1, #s2, #s3, #i1, #i3, #i2, #i0 )) }
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            LOWPASS_OUTPUT => Ok(self.state[3]),
            HIGHPASS_OUTPUT => Ok(self.state[0]),
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
