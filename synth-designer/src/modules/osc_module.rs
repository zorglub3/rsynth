use synth_engine::modules::*;
use synth_engine::simulator::module::Module;
use crate::*;
use ini::Properties;
use crate::modules::*;

const MODULE_TYPE: &str = "oscillator";
const MODULE_NAME: &str = "name";
const FREQUENCY_CONTROL: &str = "frequency";
const PRESSURE_CONTROL: &str = "pressure";
const VELOCITY_CONTROL: &str = "velocity";
const SIGNAL_1_OUTPUT: &str = "signal1";
const SIGNAL_2_OUTPUT: &str = "signal2";

pub struct OscillatorModuleSpec {
    name: String,
    inputs: [InputSpec; 3],
    state: [usize; 2],
    f0: f32,
    a: f32,
    b: f32,
}

impl OscillatorModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        todo!()
    }
}

impl ModuleSpec for OscillatorModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        todo!()
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        todo!()
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

pub struct OscModule {
    name: String,
    inputs: [usize; 3],
    state: [usize; 2],
    f0: f32,
    a: f32,
    b: f32,
}

impl OscModule {
    pub fn new(f0: f32, a: f32, b: f32) -> Self {
        Self {
            name: "bowed oscillator".to_string(),
            inputs: [0; 3],
            state: [0; 2],
            f0,
            a,
            b,
        }
    }

    pub fn signal_output(&self) -> ModuleOutput {
        ModuleOutput {
            module_output_index: 0,
            state_index: self.state[0],
        }
    }

    pub fn frequency_control(&mut self) -> ModuleInput {
        ModuleInput {
            module_input_index: 0,
            module: self,
        }
    }

    pub fn pressure_control(&mut self) -> ModuleInput {
        ModuleInput {
            module_input_index: 1,
            module: self,
        }
    }

    pub fn velocity_control(&mut self) -> ModuleInput {
        ModuleInput {
            module_input_index: 2,
            module: self,
        }
    }
}

impl SynthModule for OscModule {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn state_size(&self) -> usize {
        2
    }

    fn allocate_state(&mut self, state_allocator: &mut StateAllocator) {
        state_allocator.allocate(&mut self.state);
    }

    fn set_input(&mut self, input_index: usize, state_index: usize) {
        self.inputs[input_index] = state_index;
    }

    fn create(self) -> Box<dyn Module> {
        Box::new(
            BowedOscillator::new(
                self.f0,
                self.a,
                self.b,
                self.state[0],
                self.state[1],
                self.inputs[0],
                self.inputs[1],
                self.inputs[2],
            )
        )
    }
}
