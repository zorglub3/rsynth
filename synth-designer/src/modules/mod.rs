mod amp_module;
mod osc_module;
mod filter_module;
mod midi_cc_module;
mod midi_mono_module;
mod contour_module;
mod mono_out_module;

pub use amp_module::AmpModule;
pub use osc_module::OscModule;
pub use filter_module::FilterModule;
pub use midi_cc_module::MidiCCModule;
pub use midi_mono_module::MidiMonoModule;
pub use contour_module::ContourModule;
pub use mono_out_module::MonoOutputModule;

use synth_engine::simulator::module::Module;
use crate::StateAllocator;
use std::collections::BTreeMap;

const ZERO_MODULE: &str = "zero";
const ZERO_FIELD: &str = "zero";

pub struct InputSpec {
    module_name: String,
    state_field: String,
}

pub fn zero_input() -> InputSpec {
    InputSpec { module_name: ZERO_MODULE.to_string(), state_field: ZERO_FIELD.to_string() }
}

pub enum ModuleError {
    MissingField { module_type: String, field_name: String },
    MalformedInputSpec { value: String },
    MissingStateName { module_type: String, module_name: String, field_name: String },
    MissingModule { module_name: String },
}

pub struct SynthSpec(BTreeMap<String, Box<dyn ModuleSpec>>);

impl SynthSpec {
    fn new() -> Self {
        // TODO add ZERO_MODULE
        Self(BTreeMap::new())
    }

    fn add_module(&mut self, module_spec: Box<dyn ModuleSpec>) {
        todo!()
    }

    fn input_state_index(&self, input_spec: &InputSpec) -> Result<usize, ModuleError> {
        let module_spec = 
            self.0.get(&input_spec.module_name)
                .ok_or(ModuleError::MissingModule { module_name: input_spec.module_name.clone() });

        module_spec?.state_index(&input_spec.state_field)
    }
}


pub trait ModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator);
    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError>;
    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError>;
    fn get_name(&self) -> &str;
}

pub fn parse_input_spec(s: &str) -> Result<InputSpec, ModuleError> {
    todo!()
}
