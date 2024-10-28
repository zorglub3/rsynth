use synth_engine::modules::*;
use synth_engine::simulator::module::Module;
use crate::*;
use crate::modules::*;

pub struct ZeroModuleSpec {
    state: [usize; 1],
}

impl ZeroModuleSpec {
    pub fn new() -> Self {
        Self { state: [0] }
    }
}

impl ModuleSpec for ZeroModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, _synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        Ok(Box::new(ZeroModule::new(self.state[0])))
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            ZERO_FIELD => Ok(self.state[0]),
            _ => Err(ModuleError::MissingStateName { module_type: ZERO_MODULE.to_string(), module_name: ZERO_MODULE.to_string(), field_name: state_field.to_string() }),
        }
    }

    fn get_name(&self) -> &str {
        ZERO_MODULE
    }

    fn state_size(&self) -> usize {
        1
    }
}