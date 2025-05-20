use crate::modules::*;
use crate::synth_spec::SynthSpec;
use ini::Properties;
use proc_macro2::TokenStream;
use quote::quote;
use synth_engine::modules::*;
use synth_engine::simulator::state::StateInput;
use core::ops::Range;

const MODULE_TYPE: &str = "midi_mono";
const MODULE_NAME: &str = "name";
const PITCH_OUTPUT: &str = "pitch";
const GATE_OUTPUT: &str = "gate";
const PRESSURE_OUTPUT: &str = "aftertouch";
const VELOCITY_OUTPUT: &str = "velocity";
const PITCHWHEEL_OUTPUT: &str = "pitchwheel";

pub struct MonoKeysModuleSpec {
    name: String,
    state_range: Range<usize>,
    input_range: Range<usize>,
}

impl MonoKeysModuleSpec {
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
            state_range: 0..0,
            input_range: 0..0,
        })
    }
}

impl ModuleSpec for MonoKeysModuleSpec {
    fn allocate_state_input(&mut self, alloc: &mut StateAllocator) {
        let state_input_range = alloc.allocate(MonoKeys::STATE_SIZE, MonoKeys::INPUT_SIZE);
        self.state_range = state_input_range.state_range;
        self.input_range = state_input_range.input_range;
    }

    fn compile_input_exprs(
        &self,
        synth_spec: &SynthSpec,
        state_input: &mut StateInput,
    ) -> Result<(), ModuleError> {
        /* do nothing */
        Ok(())
    }

    fn make_module_entry(&self) -> ModuleEntry {
        let m = MonoKeys::new();

        ModuleEntry {
            synth_module: SynthModule::MonoKeys(m),
            state: self.state_range.clone(),
            input: self.input_range.clone(),
        }
    }

    /*
    fn create_module(&self, _synth_spec: &SynthSpec) -> Result<SynthModule, ModuleError> {
        let midi_mono = MonoKeys::new(
            self.state[0],
            self.state[1],
            self.state[2],
            self.state[3],
            self.state[4],
        );

        Ok(SynthModule::MonoKeys(midi_mono))
    }
    */

    fn codegen(&self, _synth_spec: &SynthSpec) -> TokenStream {
        /*
        let s0 = self.state[0];
        let s1 = self.state[1];
        let s2 = self.state[2];
        let s3 = self.state[3];
        let s4 = self.state[4];

        quote! { SynthModule::MonoKeys(MonoKeys::new(#s0, #s1, #s2, #s3, #s4)) }
        */
        todo!()
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            PITCH_OUTPUT => Ok(self.state_range.start + MonoKeys::PITCH_OUTPUT),
            GATE_OUTPUT => Ok(self.state_range.start + MonoKeys::GATE_OUTPUT),
            PRESSURE_OUTPUT => Ok(self.state_range.start + MonoKeys::PRESSURE_OUTPUT),
            VELOCITY_OUTPUT => Ok(self.state_range.start + MonoKeys::VELOCITY_OUTPUT),
            PITCHWHEEL_OUTPUT => Ok(self.state_range.start + MonoKeys::PITCHWHEEL_OUTPUT),
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
        MonoKeys::STATE_SIZE
    }

    fn input_size(&self) -> usize {
        MonoKeys::INPUT_SIZE
    }
}
