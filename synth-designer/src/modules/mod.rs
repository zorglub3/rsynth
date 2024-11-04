mod amp_module;
mod contour_module;
mod filter_module;
mod input_spec;
mod midi_cc_module;
mod midi_mono_module;
mod mono_out_module;
mod osc_module;
mod zero;

pub use amp_module::AmpModuleSpec;
pub use contour_module::ContourModuleSpec;
pub use filter_module::FilterModuleSpec;
pub use input_spec::InputSpec;
pub use input_spec::InputSpecTerm;
pub use midi_cc_module::MidiCCModuleSpec;
pub use midi_mono_module::MidiMonoModuleSpec;
pub use mono_out_module::MonoOutputModuleSpec;
pub use osc_module::OscillatorModuleSpec;
pub use zero::ZeroModuleSpec;

use crate::StateAllocator;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::num::ParseFloatError;
use std::num::ParseIntError;
use synth_engine::modules::{InputExpr, InputTerm};
use synth_engine::simulator::module::Module;

pub const ZERO_MODULE: &str = "zero";
pub const ZERO_FIELD: &str = "zero";

/*
pub struct InputSpec {
    module_name: String,
    state_field: String,
}

pub fn zero_input() -> InputSpec {
    InputSpec {
        module_name: ZERO_MODULE.to_string(),
        state_field: ZERO_FIELD.to_string(),
    }
}
*/

#[derive(Debug)]
pub enum ModuleError {
    MissingField {
        module_type: String,
        field_name: String,
    },
    MalformedInputSpec {
        value: String,
    },
    MissingStateName {
        module_type: String,
        module_name: String,
        field_name: String,
    },
    MissingModule {
        module_name: String,
    },
    ParseFloatError {
        parse_error: ParseFloatError,
    },
    ParseIntError {
        parse_error: ParseIntError,
    },
}

impl From<ParseFloatError> for ModuleError {
    fn from(err: ParseFloatError) -> Self {
        Self::ParseFloatError { parse_error: err }
    }
}

impl From<ParseIntError> for ModuleError {
    fn from(err: ParseIntError) -> Self {
        Self::ParseIntError { parse_error: err }
    }
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

    pub fn add_module(&mut self, module_spec: Box<dyn ModuleSpec>) {
        let key = module_spec.get_name().to_string();

        // TODO check that the key isn't already taken

        self.0.insert(key, module_spec);
    }

    pub fn input_state_index(
        &self,
        module_name: &str,
        module_field: &str,
    ) -> Result<usize, ModuleError> {
        let module_spec = self.0.get(module_name).ok_or(ModuleError::MissingModule {
            module_name: module_name.to_string(),
        });

        module_spec?.state_index(module_field)
    }

    pub fn input_expr(&self, input_spec: &InputSpec) -> Result<InputExpr, ModuleError> {
        let mut result = vec![];

        for term in input_spec.0.iter() {
            match term {
                InputSpecTerm::Constant(v) => result.push(InputTerm::constant(*v)),
                InputSpecTerm::Term {
                    module_name,
                    module_field,
                    scale,
                } => {
                    let state_index = self.input_state_index(module_name, module_field)?;
                    result.push(InputTerm::term(state_index, *scale));
                }
            }
        }

        Ok(InputExpr::new(result))
    }
    /*
    pub fn input_expr(&self, input_spec: &InputSpec) -> Result<InputExpr, ModuleError> {
        let state_index = self.input_state_index(input_spec)?;

        Ok(InputExpr::new(vec![InputTerm::term(state_index, 1.)]))
    }
    */

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

    pub fn make_modules(
        &self,
        modules: &mut HashMap<String, Box<dyn Module>>,
    ) -> Result<(), ModuleError> {
        for (k, v) in self.0.iter() {
            modules.insert(k.clone(), v.create_module(self)?);
        }

        Ok(())
    }
}

pub trait ModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator);
    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError>;
    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError>;
    fn get_name(&self) -> &str;
    fn state_size(&self) -> usize;
}

/*
pub fn parse_input_spec(s: &str) -> Result<InputSpec, ModuleError> {
    let mut split = s.split(':');

    let Some(module_name) = split.next().map(|s| s.to_string()) else {
        return Err(ModuleError::MalformedInputSpec {
            value: s.to_string(),
        });
    };

    let Some(state_field) = split.next().map(|s| s.to_string()) else {
        return Err(ModuleError::MalformedInputSpec {
            value: s.to_string(),
        });
    };

    if split.next().is_none() {
        Ok(InputSpec {
            module_name,
            state_field,
        })
    } else {
        Err(ModuleError::MalformedInputSpec {
            value: s.to_string(),
        })
    }
}
*/
