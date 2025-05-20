use crate::input_expr::*;
use crate::modules::*;
use crate::synth_spec::gen_stack_program;
use crate::synth_spec::SynthSpec;
use crate::DEFAULT_FREQUENCY_ZERO;
use ini::Properties;
use proc_macro2::TokenStream;
use quote::quote;
use synth_engine::modules::*;
use synth_engine::simulator::state::StateInput;
use core::ops::Range;

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
// const INPUT_SIZE: usize = 4;
// const STATE_SIZE: usize = 2;

pub struct BowedOscillatorModuleSpec {
    name: String,
    inputs: Vec<Expr>,
    // state: [usize; STATE_SIZE],
    f0: f32,
    a: f32,
    state_range: Range<usize>,
    input_range: Range<usize>,
}

impl BowedOscillatorModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut inputs = vec![Expr::zero(); BowedOscillator::INPUT_SIZE];
        /*
        let mut fc: Expr = Expr::zero();
        let mut lc: Expr = Expr::zero();
        let mut pc: Expr = Expr::zero();
        let mut vc: Expr = Expr::zero();
        */
        let mut f0: f32 = DEFAULT_FREQUENCY_ZERO;
        let mut a: f32 = 0.;

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                FREQUENCY_CONTROL => inputs[BowedOscillator::EXP_CONTROL_INPUT] = Expr::parse(&v)?,
                LINEAR_FREQUENCY_CONTROL => inputs[BowedOscillator::LINEAR_CONTROL_INPUT] = Expr::parse(&v)?,
                PRESSURE_CONTROL => inputs[BowedOscillator::PRESSURE_CONTROL_INPUT] = Expr::parse(&v)?,
                VELOCITY_CONTROL => inputs[BowedOscillator::VELOCITY_CONTROL_INPUT] = Expr::parse(&v)?,
                FREQ0 => f0 = v.parse::<f32>()?,
                PARAM_A => a = v.parse::<f32>()?,
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        Ok(Self {
            name,
            inputs,
            // state: [0; STATE_SIZE],
            f0,
            a,
            state_range: 0..0,
            input_range: 0..0,
        })
    }
}

impl ModuleSpec for BowedOscillatorModuleSpec {
    fn allocate_state_input(&mut self, alloc: &mut StateAllocator) {
        let state_input_range = alloc.allocate(BowedOscillator::STATE_SIZE, BowedOscillator::INPUT_SIZE);
        self.state_range = state_input_range.state_range;
        self.input_range = state_input_range.input_range;
        // alloc.allocate(&mut self.state);
    }

    fn compile_input_exprs(
        &self,
        synth_spec: &SynthSpec,
        state_input: &mut StateInput,
    ) -> Result<(), ModuleError> {
        for i in 0..BowedOscillator::INPUT_SIZE {
            let program = self.inputs[i].compile(synth_spec)?;
            let index = i + self.input_range.start;
            state_input.set_program(index, program);
        }

        Ok(())
    }

    fn make_module_entry(&self) -> ModuleEntry {
        ModuleEntry {
            synth_module: SynthModule::Bowed(BowedOscillator::new(self.f0, self.a)),
            input: self.input_range.clone(),
            state: self.state_range.clone(),
        }
    }
    /*
    fn create_module(&self, synth_spec: &SynthSpec) -> Result<SynthModule, ModuleError> {
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

        Ok(SynthModule::Bowed(osc))
    }
    */

    fn codegen(&self, synth_spec: &SynthSpec) -> TokenStream {
        let f0 = self.f0;
        let a = self.a;
        /*
        let s0 = self.state[0];
        let s1 = self.state[1];
        let i0 = gen_stack_program(&self.inputs[0].compile(&synth_spec).unwrap());
        let i1 = gen_stack_program(&self.inputs[0].compile(&synth_spec).unwrap());
        let i2 = gen_stack_program(&self.inputs[0].compile(&synth_spec).unwrap());
        let i3 = gen_stack_program(&self.inputs[0].compile(&synth_spec).unwrap());
        */

        todo!("should include state- and input- ranges")
        // quote! { SynthModule::Bowed(BowedOscillator::new( #f0, #a, #s0, #s1, #i0, #i1, #i2, #i3 )) }
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            SIGNAL_1_OUTPUT => Ok(self.state_range.start + BowedOscillator::STATE_U),
            SIGNAL_2_OUTPUT => Ok(self.state_range.start + BowedOscillator::STATE_V),
            // SIGNAL_1_OUTPUT => Ok(self.state[0]),
            // SIGNAL_2_OUTPUT => Ok(self.state[1]),
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
        BowedOscillator::STATE_SIZE
    }

    fn input_size(&self) -> usize {
        BowedOscillator::INPUT_SIZE
    }
}
