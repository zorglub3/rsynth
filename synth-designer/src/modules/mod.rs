mod amp_module;
mod bowed_osc;
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
mod quad;
mod saw_osc;
mod wavetable;

pub use amp_module::AmpModuleSpec;
pub use bowed_osc::BowedOscillatorModuleSpec;
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
pub use quad::QuadOscillatorModuleSpec;
pub use saw_osc::SawOscillatorModuleSpec;
pub use wavetable::WavetableOscillatorModuleSpec;

use crate::input_expr::ExprError;
use crate::state_allocator::StateAllocator;
use crate::synth_spec::SynthSpec;
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

pub trait ModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator);
    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError>;
    fn codegen(&self, synth_spec: &SynthSpec) -> TokenStream;
    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError>;
    fn get_name(&self) -> &str;
    fn state_size(&self) -> usize;
}
