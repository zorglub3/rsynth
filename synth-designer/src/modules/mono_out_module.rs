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
        let mut name = MODULE_TYPE.to_string();
        let mut output_index: usize = 0;
        let mut signal_in: InputSpec = InputSpec::zero();

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                SIGNAL_INPUT => signal_in = InputSpec::parse(&v)?,
                OUTPUT_INDEX => output_index = v.parse::<usize>()?,
                _ => {
                    return Err(ModuleError::InvalidField {
                        module_type: MODULE_TYPE.to_string(),
                        field_name: k,
                    })
                }
            }
        }

        Ok(Self {
            name,
            output_index,
            inputs: [signal_in],
        })
    }
}

impl ModuleSpec for MonoOutputModuleSpec {
    fn allocate_state(&mut self, _alloc: &mut StateAllocator) {
        /* do nothing */
    }

    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let mono_output =
            MonoOutput::new(self.output_index, synth_spec.input_expr(&self.inputs[0])?);

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
