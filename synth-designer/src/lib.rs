pub mod input_expr;
pub mod modules;
pub mod state_allocator;
pub mod synth_spec;
pub mod codegen;

use crate::modules::ModuleError;

/// This matches the frequency of note zero for the default MIDI spec.
pub const DEFAULT_FREQUENCY_ZERO: f32 = 8.18;

// TODO use thiserror::Error here
#[derive(Debug)]
pub enum SynthError {
    FileError(ini::Error),
    ModuleError(ModuleError),
    UnknownModule(String),
}

impl From<ini::Error> for SynthError {
    fn from(err: ini::Error) -> Self {
        Self::FileError(err)
    }
}

impl From<ModuleError> for SynthError {
    fn from(err: ModuleError) -> Self {
        Self::ModuleError(err)
    }
}
