use crate::input_expr::*;
use crate::modules::*;
use crate::synth_spec::gen_stack_program;
use crate::synth_spec::SynthSpec;
use crate::DEFAULT_FREQUENCY_ZERO;
use ini::Properties;
use proc_macro2::TokenStream;
use quote::quote;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;
use crate::codegen::Codegen;

const MODULE_TYPE: &str = "bowed_oscillator";
const MODULE_NAME: &str = "name";
const FREQUENCY_CONTROL: &str = "frequency_control";
const LINEAR_FREQUENCY_CONTROL: &str = "linear_frequency_control";
const PRESSURE_CONTROL: &str = "pressure_control";
const VELOCITY_CONTROL: &str = "velocity_control";
const SIGNAL_1_OUTPUT: &str = "position_output";
const SIGNAL_2_OUTPUT: &str = "signal_output";
const FREQ0: &str = "frequency_zero";
const PARAM_A: &str = "param_a";
const INPUT_SIZE: usize = 4;
const STATE_SIZE: usize = 2;

pub struct BowedOscillatorModuleSpec {
    name: String,
    inputs: [Expr; INPUT_SIZE],
    state: [usize; STATE_SIZE],
    f0: f32,
    a: f32,
}

impl BowedOscillatorModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut fc: Expr = Expr::zero();
        let mut lc: Expr = Expr::zero();
        let mut pc: Expr = Expr::zero();
        let mut vc: Expr = Expr::zero();
        let mut f0: f32 = DEFAULT_FREQUENCY_ZERO;
        let mut a: f32 = 0.;

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                FREQUENCY_CONTROL => fc = Expr::parse(&v)?,
                LINEAR_FREQUENCY_CONTROL => lc = Expr::parse(&v)?,
                PRESSURE_CONTROL => pc = Expr::parse(&v)?,
                VELOCITY_CONTROL => vc = Expr::parse(&v)?,
                FREQ0 => f0 = v.parse::<f32>()?,
                PARAM_A => a = v.parse::<f32>()?,
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        Ok(Self {
            name,
            inputs: [fc, lc, pc, vc],
            state: [0; STATE_SIZE],
            f0,
            a,
        })
    }
}

impl ModuleSpec for BowedOscillatorModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module(&self, synth_spec: &SynthSpec) -> Result<Box<dyn Module>, ModuleError> {
        let osc = BowedOscillator::new(
            self.f0,
            self.a,
            self.state[0],
            self.state[1],
            self.inputs[0].compile(&synth_spec)?,
            self.inputs[1].compile(&synth_spec)?,
            self.inputs[2].compile(&synth_spec)?,
            self.inputs[3].compile(&synth_spec)?,
        );

        Ok(Box::new(osc))
    }

    fn codegen(&self, synth_spec: &SynthSpec, codegen: &mut Codegen) -> TokenStream {
        let f0 = self.f0;
        let a = self.a;
        let s0 = self.state[0];
        let s1 = self.state[1];
        let i0 = codegen.add_stack_program(&self.inputs[0], &synth_spec);
        let i1 = codegen.add_stack_program(&self.inputs[1], &synth_spec);
        let i2 = codegen.add_stack_program(&self.inputs[2], &synth_spec);
        let i3 = codegen.add_stack_program(&self.inputs[3], &synth_spec);

        quote! { SynthModule::Bowed(BowedOscillator::new( #f0, #a, #s0, #s1, #i0, #i1, #i2, #i3 )) }
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            SIGNAL_1_OUTPUT => Ok(self.state[0]),
            SIGNAL_2_OUTPUT => Ok(self.state[1]),
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
