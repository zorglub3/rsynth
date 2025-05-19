mod amp_module;
mod bowed_osc;
mod contour_module;
mod control_module;
mod delay_line;
mod filter_12db_module;
mod filter_24db;
mod filter_6db;
mod folder_module;
mod mono_keys_module;
mod mono_out_module;
mod noise;
mod quad;
mod vosim;
mod wavetable;

pub use amp_module::AmpModuleSpec;
pub use bowed_osc::BowedOscillatorModuleSpec;
pub use contour_module::ContourModuleSpec;
pub use control_module::ControlModuleSpec;
pub use delay_line::DelayLineModuleSpec;
pub use filter_12db_module::Filter12dbModuleSpec;
pub use filter_24db::Filter24dbModuleSpec;
pub use filter_6db::Filter6dbModuleSpec;
pub use folder_module::FolderModuleSpec;
pub use mono_keys_module::MonoKeysModuleSpec;
pub use mono_out_module::MonoOutputModuleSpec;
pub use noise::NoiseGeneratorModuleSpec;
pub use quad::QuadOscillatorModuleSpec;
pub use vosim::VosimOscillatorModuleSpec;
pub use wavetable::WavetableOscillatorModuleSpec;

use crate::input_expr::ExprError;
use crate::state_allocator::StateAllocator;
use crate::synth_spec::SynthSpec;
use proc_macro2::TokenStream;
use std::num::ParseFloatError;
use std::num::ParseIntError;
use synth_engine::modules::SynthModule;
use synth_engine::simulator::ModuleEntry;
use thiserror::Error;
use synth_engine::simulator::state::StateInput;

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

// to build a synth from module specs
// - gather all module specs in a synth spec
// - allocate state/input for all module specs
// - compile input exprs (and put stack programs in StateInput struct)
// - make ModuleEntry from all module specs (put in a vec)
// - give StateInput struct and vec of ModuleEntry to simulator
// - ... good to go!

pub trait ModuleSpec {
    // fn allocate_state(&mut self, alloc: &mut StateAllocator);
    fn allocate_state_input(&mut self, alloc: &mut StateAllocator);

    // fn create_module(&self, synth_spec: &SynthSpec) -> Result<SynthModule, ModuleError>;
    fn compile_input_exprs(&self, synth_spec: &SynthSpec, state_input: &mut StateInput) -> Result<(), ModuleError>;
    fn make_module_entry(&self) -> ModuleEntry;

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError>;
    fn get_name(&self) -> &str;

    fn state_size(&self) -> usize;
    fn input_size(&self) -> usize;

    fn codegen(&self, synth_spec: &SynthSpec) -> TokenStream;
}
