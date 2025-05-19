use crate::modules::*;
use crate::synth_spec::SynthSpec;
use ini::Properties;
use proc_macro2::TokenStream;
use quote::quote;
use synth_engine::modules::*;
use synth_engine::simulator::state::StateInput;

const MODULE_TYPE: &str = "midi_cc";
const MODULE_NAME: &str = "name";
const CONTROL: &str = "control";
const SIGNAL_OUTPUT: &str = "signal_output";
const MIN_VALUE: &str = "min_value";
const MAX_VALUE: &str = "max_value";
const FILTER_VALUE: &str = "filter";
const STATE_SIZE: usize = 1;

pub struct ControlModuleSpec {
    name: String,
    control: usize,
    // state: [usize; STATE_SIZE],
    min_value: f32,
    max_value: f32,
    filter_freq: Option<f32>,
}

impl ControlModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut control: usize = 0;
        let mut min_value: f32 = 0.;
        let mut max_value: f32 = 1.;
        let mut filter_freq: Option<f32> = None;

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                CONTROL => control = v.parse::<usize>()?,
                MIN_VALUE => min_value = v.parse::<f32>()?,
                MAX_VALUE => max_value = v.parse::<f32>()?,
                FILTER_VALUE => filter_freq = Some(v.parse::<f32>()?),
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        Ok(Self {
            name,
            control,
            min_value,
            max_value,
            filter_freq,
        })
    }
}

impl ModuleSpec for ControlModuleSpec {
    fn allocate_state_input(&mut self, alloc: &mut StateAllocator) {
        let state_input_range = alloc.allocate(ContinuousControl::STATE_SIZE, ContinuousControl::INPUT_SIZE);
        self.state_range = state_input_range.state_range;
    }

    fn compile_input_exprs(
        &self,
        _synth_spec: &SynthSpec,
        _state_input: &mut StateInput,
    ) -> Result<(), ModuleError> {
        Ok(())
    }

    fn make_module_entry(&self) -> ModuleEntry {
        let m = ContinuousControl::new(
            self.control,
            self.min_value,
            self.max_value,
            self.filter_freq,
        );

        ModuleEntry {
            synth_module: SynthModule::ContinuousControl(m),
            input: 0..0,
            state: self.state_range.clone(),
        }
    }

    /*
    fn create_module(&self, _synth_spec: &SynthSpec) -> Result<SynthModule, ModuleError> {
        let midi_cc =
            ContinuousControl::new(self.state[0], self.control, self.min_value, self.max_value);

        Ok(SynthModule::ContinuousControl(midi_cc))
    }
    */

    fn codegen(&self, _synth_spec: &SynthSpec) -> TokenStream {
        /*
        let s0 = self.state[0];
        let c = self.control;
        let min = self.min_value;
        let max = self.max_value;

        quote! { SynthModule::ContinuousControl(ContinuousControl::new(#s0, #c, #min, #max)) }
        */
        todo!()
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            SIGNAL_OUTPUT => Ok(self.state_range.start + ContinuousControl::CONTROL_OUTPUT),
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
        ContinuousControl::STATE_SIZE
    }

    fn input_size(&self) -> usize {
        ContinuousControl::INPUT_SIZE
    }
}
