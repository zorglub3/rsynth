use synth_engine::modules::*;
use synth_engine::simulator::module::Module;
use crate::*;
use ini::Properties;
use crate::modules::*;

const MODULE_TYPE: &str = "mono_output";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";

pub struct MonoOutputModuleSpec {
    name: String,
    state: [usize; 1],
}

impl MonoOutputModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let name =
            props.get(MODULE_NAME)
                .ok_or(ModuleError::MissingField {
                    module_type: MODULE_TYPE.to_string(),
                    field_name: MODULE_NAME.to_string(),
                })?;

        Ok(Self {
            name: name.to_string(),
            state: [0; 1],
        })
    }
}

impl ModuleSpec for MonoOutputModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        todo!()
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        // One happy day even this module might have outputs :-P
        match state_field {
            _ => Err(ModuleError::MissingStateName { module_type: MODULE_TYPE.to_string(), module_name: self.name.clone(), field_name: state_field.to_string() }),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

pub struct MonoOutputModule {
    name: String,
    inputs: [usize; 1],
}

impl MonoOutputModule {
    pub fn new() -> Self {
        Self {
            name: "mono output".to_string(),
            inputs: [0; 1],
        }
    }

    pub fn signal_input(&mut self) -> ModuleInput {
        ModuleInput {
            module_input_index: 0,
            module: self,
        }
    }
}

impl SynthModule for MonoOutputModule {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn state_size(&self) -> usize {
        0
    }

    fn allocate_state(&mut self, _state_allocator: &mut StateAllocator) {
        /* do nothing - this module has no internal state */
    }

    fn set_input(&mut self, input_index: usize, state_index: usize) {
        self.inputs[input_index] = state_index;
    }

    fn create(self) -> Box<dyn Module> {
        todo!()
    }
}
