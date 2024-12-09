mod amp_module;
mod contour_module;
mod delay_line;
mod filter_12db_module;
mod filter_24db;
mod filter_6db;
mod folder_module;
mod midi_cc_module;
mod midi_mono_module;
mod mono_out_module;
mod noise;
mod osc_module;
mod saw_osc;
mod wavetable;
mod zero;

pub use amp_module::AmpModuleSpec;
pub use contour_module::ContourModuleSpec;
pub use delay_line::DelayLineModuleSpec;
pub use filter_12db_module::Filter12dbModuleSpec;
pub use filter_24db::Filter24dbModuleSpec;
pub use filter_6db::Filter6dbModuleSpec;
pub use folder_module::FolderModuleSpec;
pub use midi_cc_module::MidiCCModuleSpec;
pub use midi_mono_module::MidiMonoModuleSpec;
pub use mono_out_module::MonoOutputModuleSpec;
pub use noise::NoiseGeneratorModuleSpec;
pub use osc_module::OscillatorModuleSpec;
pub use saw_osc::SawOscillatorModuleSpec;
pub use wavetable::WavetableOscillatorModuleSpec;
pub use zero::ZeroModuleSpec;

use crate::input_expr::ExprError;
use crate::StateAllocator;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;
use std::num::ParseFloatError;
use std::num::ParseIntError;
use synth_engine::simulator::module::Module;
use synth_engine::stack_program::Function;
use synth_engine::stack_program::Instr;
use synth_engine::stack_program::StackProgram;
use thiserror::Error;

pub const ZERO_MODULE: &str = "zero";
pub const ZERO_FIELD: &str = "zero";

#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("No such field, {1}, on module with type {0}")]
    MissingField(String, String),
    #[error("Malformed input spec: {0}")]
    MalformedInputSpec(String),
    #[error("Invalid field, {1}, for module with type {0}")]
    InvalidField(String, String),
    #[error("No such field, {2}, on module {1} of type {0}")]
    MissingStateName(String, String, String),
    #[error("Missing module {0}")]
    MissingModule(String),
    #[error("Error parsing float: {0}")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("Error parsing integer: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("Error parsing INI file: {0}")]
    HoundError(#[from] hound::Error),
    #[error("Error in arithmetic expression: {0}")]
    ExprError(#[from] ExprError),
    #[error("Module with name {0} already in spec")]
    ModuleNameClash(String),
}

pub struct SynthSpec(BTreeMap<String, Box<dyn ModuleSpec>>);

impl SynthSpec {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn add_zero_module(&mut self) {
        self.0
            .insert(ZERO_MODULE.to_string(), Box::new(ZeroModuleSpec::new()));
    }

    pub fn add_module(&mut self, module_spec: Box<dyn ModuleSpec>) -> Result<(), ModuleError> {
        let key = module_spec.get_name().to_string();

        if self.0.contains_key(&key) {
            Err(ModuleError::ModuleNameClash(key))
        } else {
            self.0.insert(key, module_spec);
            Ok(())
        }
    }

    pub fn input_state_index(
        &self,
        module_name: &str,
        module_field: &str,
    ) -> Result<usize, ModuleError> {
        let module_spec = self
            .0
            .get(module_name)
            .ok_or(ModuleError::MissingModule(module_name.to_string()));

        module_spec?.state_index(module_field)
    }

    fn state_size(&self) -> usize {
        let mut state_size: usize = 0;

        for (_k, v) in self.0.iter() {
            state_size = state_size + v.state_size();
        }

        state_size
    }

    pub fn allocate_state(&mut self) -> usize {
        let size = self.state_size();

        let mut state_allocator = StateAllocator::new(size);

        for (_k, v) in self.0.iter_mut() {
            v.allocate_state(&mut state_allocator)
        }

        size
    }

    pub fn make_modules(&self, modules: &mut Vec<Box<dyn Module>>) -> Result<(), ModuleError> {
        for (_k, v) in self.0.iter() {
            modules.push(v.create_module(self)?);
        }

        Ok(())
    }

    pub fn codegen(&self) -> Result<TokenStream, ModuleError> {
        let mut module_code: Vec<TokenStream> = Vec::new();

        for (_k, v) in self.0.iter() {
            module_code.push(v.codegen(self));
        }

        Ok(module_code.into_iter().collect())
    }
}

pub fn gen_stack_program(stack_program: &StackProgram) -> TokenStream {
    let mut prg: Vec<TokenStream> = Vec::new();

    for instr in &stack_program.code {
        use Instr::*;

        match instr {
            Add => prg.push(quote! { Add }),
            Subtract => prg.push(quote! { Subtract }),
            Multiply => prg.push(quote! { Multiply }),
            Divide => prg.push(quote! { Divide }),
            Negate => prg.push(quote! { Negate }),
            Const(v) => prg.push(quote! { Const(#v) }),
            State(s) => prg.push(quote! { State(#s) }),
            Call(f) => {
                use Function::*;

                match f {
                    Sin => prg.push(quote! { Call(Sin) }),
                    Cos => prg.push(quote! { Call(Cos) }),
                    Tan => prg.push(quote! { Call(Tan) }),
                    Tanh => prg.push(quote! { Call(Tanh) }),
                    Ln => prg.push(quote! { Call(Ln) }),
                    Exp => prg.push(quote! { Call(Exp) }),
                    Logistic => prg.push(quote! { Call(Logistic) }),
                    Abs => prg.push(quote! { Call(Abs) }),
                    Min => prg.push(quote! { Call(Min) }),
                    Max => prg.push(quote! { Call(Max) }),
                    Lerp => prg.push(quote! { Call(Lerp) }),
                }
            }
        }
    }

    quote! {
        StackProgram {
            code: #(prg.into_iter().collect()),
            stack_size: #(stack_program.stack_size),
        }
    }
}

pub trait ModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator);
    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError>;
    fn codegen(&self, synth_spec: &SynthSpec) -> TokenStream;
    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError>;
    fn get_name(&self) -> &str;
    fn state_size(&self) -> usize;
}
