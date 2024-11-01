use crate::modules::*;
use crate::*;
use ini::Properties;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;

const MODULE_TYPE: &str = "mono_output";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";
const OUTPUT_INDEX: &str = "output_index";
const INPUT_SIZE: usize = 1;

pub struct MonoOutputModuleSpec {
    name: String,
    output_index: usize,
    inputs: [InputSpec; INPUT_SIZE],
}

impl MonoOutputModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let name = props.get(MODULE_NAME).ok_or(ModuleError::MissingField {
            module_type: MODULE_TYPE.to_string(),
            field_name: MODULE_NAME.to_string(),
        })?;

        Ok(Self {
            name: name.to_string(),
            output_index: props
                .get(OUTPUT_INDEX)
                .map(|s| s.parse::<usize>())
                .unwrap_or(Ok(1))?,
            inputs: [props
                .get(SIGNAL_INPUT)
                .map(parse_input_spec)
                .unwrap_or(Ok(zero_input()))?],
        })
    }
}

impl ModuleSpec for MonoOutputModuleSpec {
    fn allocate_state(&mut self, _alloc: &mut StateAllocator) {
        /* do nothing */
    }

    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let mono_output = MonoOutput::new(
            self.output_index,
            synth_spec.input_state_index(&self.inputs[0])?,
        );

        Ok(Box::new(mono_output))
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        // One happy day even this module might have outputs :-P
        match state_field {
            _ => Err(ModuleError::MissingStateName {
                module_type: MODULE_TYPE.to_string(),
                module_name: self.name.clone(),
                field_name: state_field.to_string(),
            }),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn state_size(&self) -> usize {
        0
    }
}
