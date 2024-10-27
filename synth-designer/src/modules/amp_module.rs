use synth_engine::modules::*;
use synth_engine::simulator::module::Module;
use crate::*;
use ini::Properties;
use crate::modules::*;

const MODULE_TYPE: &str = "amplifier";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";
const LINEAR_CONTROL: &str = "linear_control";
const EXP_CONTROL: &str = "exp_control";
const SIGNAL_OUTPUT: &str = "signal_output";

pub struct AmpModuleSpec {
    name: String,
    inputs: [InputSpec; 3],
    state: [usize; 1],
}

impl AmpModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let name =
            props.get(MODULE_NAME)
                .ok_or(ModuleError::MissingField { 
                    module_type: MODULE_TYPE.to_string(), 
                    field_name: MODULE_NAME.to_string(),
                })?;

        Ok(Self {
            name: name.to_string(),
            inputs: [
                props.get(SIGNAL_INPUT).map(parse_input_spec).unwrap_or(Ok(zero_input()))?,
                props.get(LINEAR_CONTROL).map(parse_input_spec).unwrap_or(Ok(zero_input()))?,
                props.get(EXP_CONTROL).map(parse_input_spec).unwrap_or(Ok(zero_input()))?,
            ],
            state: [0],
        })
    }
}

impl ModuleSpec for AmpModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let amplifier = Amplifier::new(
            synth_spec.input_state_index(&self.inputs[0])?,
            self.state[0],
            synth_spec.input_state_index(&self.inputs[0])?,
            synth_spec.input_state_index(&self.inputs[0])?,
        );

        Ok(Box::new(amplifier))
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            SIGNAL_OUTPUT => Ok(self.state[0]),
            _ => Err(ModuleError::MissingStateName { module_type: MODULE_TYPE.to_string(), module_name: self.name.clone(), field_name: state_field.to_string() }),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

pub struct AmpModule {
    name: String,
    inputs: [usize; 3],
    state: [usize; 1],
}

impl AmpModule {
    pub fn new() -> Self {
        Self {
            name: "amplifier".to_string(),
            inputs: [0; 3],
            state: [0; 1],
        }
    }

    pub fn signal_input(&mut self) -> ModuleInput {
        ModuleInput {
            module_input_index: 0,
            module: self,
        }
    }

    pub fn signal_output(&self) -> ModuleOutput {
        ModuleOutput {
            module_output_index: 0,
            state_index: self.state[0],
        }
    }

    pub fn linear_control_input(&mut self) -> ModuleInput {
        ModuleInput {
            module_input_index: 1,
            module: self,
        }
    }

    pub fn exponential_control_input(&mut self) -> ModuleInput {
        ModuleInput {
            module_input_index: 2,
            module: self,
        }
    }
}

impl SynthModule for AmpModule {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn state_size(&self) -> usize {
        1
    }

    fn allocate_state(&mut self, state_allocator: &mut StateAllocator) {
        state_allocator.allocate(&mut self.state);
    }

    fn set_input(&mut self, input_index: usize, state_index: usize) {
        self.inputs[input_index] = state_index;
    }

    fn create(self) -> Box<dyn Module> {
        Box::new(
            Amplifier::new(
                self.inputs[0],
                self.state[0],
                self.inputs[1],
                self.inputs[2],
            )
        )
    }
}
