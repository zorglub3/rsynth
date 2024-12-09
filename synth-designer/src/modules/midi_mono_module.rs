use super::gen_stack_program;
use crate::modules::*;
use crate::*;
use ini::Properties;
use proc_macro2::TokenStream;
use quote::quote;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;

const MODULE_TYPE: &str = "midi_mono";
const MODULE_NAME: &str = "name";
const PITCH_OUTPUT: &str = "pitch";
const GATE_OUTPUT: &str = "gate";
const PRESSURE_OUTPUT: &str = "aftertouch";
const VELOCITY_OUTPUT: &str = "velocity";
const PITCHWHEEL_OUTPUT: &str = "pitchwheel";
const STATE_SIZE: usize = 5;

pub struct MidiMonoModuleSpec {
    name: String,
    state: [usize; STATE_SIZE],
}

impl MidiMonoModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        Ok(Self {
            name,
            state: [0; STATE_SIZE],
        })
    }
}

impl ModuleSpec for MidiMonoModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, _synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let midi_mono = MidiMono::new(
            self.state[0],
            self.state[1],
            self.state[2],
            self.state[3],
            self.state[4],
        );

        Ok(Box::new(midi_mono))
    }

    fn codegen(&self, synth_spec: &SynthSpec) -> TokenStream {
        quote! { MidiMono::new(
                #(self.state[0]),
                #(self.state[1]),
                #(self.state[2]),
                #(self.state[3]),
                #(self.state[4]),
            )
        }
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            PITCH_OUTPUT => Ok(self.state[0]),
            GATE_OUTPUT => Ok(self.state[1]),
            PRESSURE_OUTPUT => Ok(self.state[2]),
            VELOCITY_OUTPUT => Ok(self.state[3]),
            PITCHWHEEL_OUTPUT => Ok(self.state[4]),
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
