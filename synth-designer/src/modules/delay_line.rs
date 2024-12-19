use crate::input_expr::*;
use crate::modules::*;
use crate::synth_spec::gen_stack_program;
use crate::synth_spec::SynthSpec;
use ini::Properties;
use proc_macro2::TokenStream;
use quote::quote;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;
use crate::codegen::Codegen;
use crate::synth_resource::SynthResource;

const MODULE_TYPE: &str = "delay_line";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";
const FREQUENCY_ZERO: &str = "frequency_zero";
const FREQUENCY_CONTROL: &str = "pitch_control";
const LINEAR_CONTROL: &str = "linear_control";
const SIGNAL_OUTPUT: &str = "signal_output";
const DATA_SIZE_FIELD: &str = "data_size";

const INPUT_SIZE: usize = 3;
const STATE_SIZE: usize = 1;
const DATA_SIZE_VALUE: usize = 1024;

pub struct DelayLineModuleSpec {
    name: String,
    inputs: [Expr; INPUT_SIZE],
    f0: f32,
    state: [usize; STATE_SIZE],
    data_size: usize,
}

impl DelayLineModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut f0: f32 = 1.;
        let mut fc: Expr = Expr::zero();
        let mut lc: Expr = Expr::zero();
        let mut input: Expr = Expr::zero();
        let mut data_size: usize = DATA_SIZE_VALUE;

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                SIGNAL_INPUT => input = Expr::parse(&v)?,
                FREQUENCY_ZERO => f0 = v.parse::<f32>()?,
                FREQUENCY_CONTROL => fc = Expr::parse(&v)?,
                LINEAR_CONTROL => lc = Expr::parse(&v)?,
                DATA_SIZE_FIELD => data_size = v.parse::<usize>()?,
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        Ok(Self {
            name,
            inputs: [input, fc, lc],
            f0,
            state: [0; STATE_SIZE],
            data_size,
        })
    }
}

impl ModuleSpec for DelayLineModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, synth_spec: &SynthSpec, synth_resource: &SynthResource) -> Result<SynthModule, ModuleError> {
        let delay_line = DelayLine::new(
            self.f0,
            self.state[0],
            self.inputs[0].compile(&synth_spec)?,
            self.inputs[1].compile(&synth_spec)?,
            self.inputs[2].compile(&synth_spec)?,
            self.data_size,
        );

        Ok(SynthModule::Delay(delay_line))
    }

    fn codegen(&self, synth_spec: &SynthSpec, codegen: &mut Codegen) -> TokenStream {
        let f0 = self.f0;
        let s0 = self.state[0];
        let i0 = codegen.add_stack_program(&self.inputs[0], &synth_spec);
        let i1 = codegen.add_stack_program(&self.inputs[1], &synth_spec);
        let i2 = codegen.add_stack_program(&self.inputs[2], &synth_spec);
        let buffer = codegen.add_databuffer(self.data_size);

        quote! { SynthModule::Delay(DelayLine::new(#f0, #s0, #i0, #i1, #i2, #buffer)) }
    }

    fn create_resources(&self, synth_spec: &SynthSpec, synth_resources: &mut SynthResource) -> Result<(), ModuleError> {
        todo!()
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
