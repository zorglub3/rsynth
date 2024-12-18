use crate::modules::*;
use crate::synth_spec::SynthSpec;
use ini::Properties;
use proc_macro2::TokenStream;
use quote::quote;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;
use crate::codegen::Codegen;

const MODULE_TYPE: &str = "midi_cc";
const MODULE_NAME: &str = "name";
const CONTROL: &str = "control";
const SIGNAL_OUTPUT: &str = "signal_output";
const MIN_VALUE: &str = "min_value";
const MAX_VALUE: &str = "max_value";
const STATE_SIZE: usize = 1;

pub struct ControlModuleSpec {
    name: String,
    control: usize,
    state: [usize; STATE_SIZE],
    min_value: f32,
    max_value: f32,
}

impl ControlModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut control: usize = 0;
        let mut min_value: f32 = 0.;
        let mut max_value: f32 = 1.;

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                CONTROL => control = v.parse::<usize>()?,
                MIN_VALUE => min_value = v.parse::<f32>()?,
                MAX_VALUE => max_value = v.parse::<f32>()?,
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        Ok(Self {
            name,
            control,
            state: [0; 1],
            min_value,
            max_value,
        })
    }
}

impl ModuleSpec for ControlModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, _synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let midi_cc =
            ContinuousControl::new(self.state[0], self.control, self.min_value, self.max_value);

        Ok(Box::new(midi_cc))
    }

    fn codegen(&self, _synth_spec: &SynthSpec, _codegen: &mut Codegen) -> TokenStream {
        let s0 = self.state[0];
        let c = self.control;
        let min = self.min_value;
        let max = self.max_value;

        quote! { SynthModule::ContinuousControl(ContinuousControl::new(#s0, #c, #min, #max)) }
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            SIGNAL_OUTPUT => Ok(self.state[0]),
            _ => Err(ModuleError::MissingStateName(
                MODULE_TYPE.to_string(),
                self.name.clone(),
                state_field.to_string(),
            )),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn state_size(&self) -> usize {
        self.state.len()
    }
}
