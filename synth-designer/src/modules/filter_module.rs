use synth_engine::modules::*;
use synth_engine::simulator::module::Module;
use crate::*;
use ini::Properties;
use crate::modules::*;

const MODULE_TYPE: &str = "lowpass_filter";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";
const FREQ0: &str = "frequency_zero";
const CUTOFF_CONTROL: &str = "cutoff_frequency";
const RESONANCE_CONTROL: &str = "resonance";
const SIGNAL_OUTPUT: &str = "signal_output";

pub struct FilterModuleSpec {
    name: String,
    inputs: [InputSpec; 3],
    state: [usize; 4],
    f0: f32,
}

impl FilterModuleSpec {
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
                props.get(CUTOFF_CONTROL).map(parse_input_spec).unwrap_or(Ok(zero_input()))?,
                props.get(RESONANCE_CONTROL).map(parse_input_spec).unwrap_or(Ok(zero_input()))?,
            ],
            state: [0; 4],
            f0: props.get(FREQ0).map(|s| s.parse::<f32>()).unwrap_or(Ok(1.0_f32))?,
        })
    }
}

impl ModuleSpec for FilterModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let filter = MoogFilter::new(
            self.f0,
            self.state[0],
            self.state[1],
            self.state[2],
            self.state[3],
            synth_spec.input_state_index(&self.inputs[1])?,
            synth_spec.input_state_index(&self.inputs[2])?,
            synth_spec.input_state_index(&self.inputs[0])?,
        );

        Ok(Box::new(filter))
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            SIGNAL_OUTPUT => Ok(self.state[3]),
            _ => Err(ModuleError::MissingStateName { module_type: MODULE_TYPE.to_string(), module_name: self.name.clone(), field_name: state_field.to_string() }),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

pub struct FilterModule {
    name: String,
    inputs: [usize; 3],
    state: [usize; 4],
    f0: f32,
}

impl FilterModule {
    pub fn new(f0: f32) -> Self {
        Self {
            name: "Moog filter".to_string(),
            inputs: [0; 3],
            state: [0; 4],
            f0,
        }
    }

    pub fn signal_input(&mut self) -> ModuleInput {
        ModuleInput {
            module_input_index: 2,
            module: self,
        }
    }
    
    pub fn signal_output(&self) -> ModuleOutput {
        ModuleOutput {
            module_output_index: 3,
            state_index: self.state[3],
        }
    }

    pub fn frequency_control(&mut self) -> ModuleInput {
        ModuleInput {
            module_input_index: 0,
            module: self,
        }
    } 

    pub fn resonance_control(&mut self) -> ModuleInput {
        ModuleInput {
            module_input_index: 1,
            module: self,
        }
    }
}

impl SynthModule for FilterModule {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn state_size(&self) -> usize {
        4
    }

    fn allocate_state(&mut self, state_allocator: &mut StateAllocator) {
        state_allocator.allocate(&mut self.state);
    }

    fn set_input(&mut self, input_index: usize, state_index: usize) {
        self.inputs[input_index] = state_index;
    }

    fn create(self) -> Box<dyn Module> {
        Box::new(
            MoogFilter::new(
                self.f0,
                self.state[0],
                self.state[1],
                self.state[2],
                self.state[3],
                self.inputs[0],
                self.inputs[1],
                self.inputs[2],
            )
        )
    }
}
