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
const SIGNAL_1_OUTPUT: &str = "signal1";
const SIGNAL_2_OUTPUT: &str = "signal2";
const FREQ0: &str = "frequency_zero";
const INPUT_SIZE: usize = 2;
const STATE_SIZE: usize = 2;

pub struct QuadOscillatorModuleSpec {
    name: String,
    inputs: [Expr; INPUT_SIZE],
    f0: f32,
    state_range: Range<usize>,
    input_range: Range<usize>,
}

impl QuadOscillatorModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut f0: f32 = DEFAULT_FREQUENCY_ZERO;
        let mut inputs = vec![Expr::zero(); QuadratureOscillator::INPUT_SIZE];
        // let mut fc: Expr = Expr::zero();
        // let mut lc: Expr = Expr::zero();

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                FREQUENCY_CONTROL => inputs[QuadratureOscillator::EXP_CONTROL_INPUT] = Expr::parse(&v)?,
                LINEAR_FREQUENCY_CONTROL => inputs[QuadratureOscillator::LINEAR_CONTROL_INPUT] = Expr::parse(&v)?,
                FREQ0 => f0 = v.parse::<f32>()?,
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        Ok(Self {
            name,
            inputs,
            f0,
            state_range: 0..0,
            input_range: 0..0,
        })
    }
}

impl ModuleSpec for QuadOscillatorModuleSpec {
    fn allocate_state_input(&mut self, alloc: &mut StateAllocator) {
        let state_input_range = alloc.allocate(QuadratureOscillator::STATE_SIZE, QuadratureOscillator::INPUT_SIZE);
        self.state_range = state_input_range.state_range;
        self.input_range = state_input_range.input_range;
    }

    fn compile_input_exprs(
        &self,
        synth_spec: &SynthSpec,
        state_input: &mut StateInput,
    ) -> Result<(), ModuleError> {
        for i in 0..QuadratureOscillator::INPUT_SIZE {
            let program = self.inputs[i].compile(synth_spec)?;
            let index = self.input_range.start + i;
            state_input.set_program(index, program);
        }

        Ok(())
    }

    fn make_module_entry(&self) -> ModuleEntry {
        let m = QuadratureOscillator::new(self.f0);

        ModuleEntry {
            synth_module: SynthModule::QuadOscillator(m),
            state: self.state_range.clone(),
            input: self.input_range.clone(),
        }
    }

    /*
    fn create_module(&self, synth_spec: &SynthSpec) -> Result<SynthModule, ModuleError> {
        let osc = QuadratureOscillator::new(
            self.f0,
            self.state[0],
            self.state[1],
            self.inputs[0].compile(&synth_spec)?,
            self.inputs[1].compile(&synth_spec)?,
        );

        Ok(SynthModule::QuadOscillator(osc))
    }
    */

    fn codegen(&self, synth_spec: &SynthSpec) -> TokenStream {
        /*
        let f0 = self.f0;
        let s0 = self.state[0];
        let s1 = self.state[1];
        let i0 = gen_stack_program(&self.inputs[0].compile(&synth_spec).unwrap());
        let i1 = gen_stack_program(&self.inputs[1].compile(&synth_spec).unwrap());

        quote! { SynthModule::QuadOscillator(QuadratureOscillator::new(#f0, #s0, #s1, #i0, #i1)) }
        */

        todo!()
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            SIGNAL_1_OUTPUT => Ok(self.state_range.start + QuadratureOscillator::STATE_X),
            SIGNAL_2_OUTPUT => Ok(self.state_range.start + QuadratureOscillator::STATE_Y),
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
        QuadratureOscillator::STATE_SIZE
    }

    fn input_size(&self) -> usize {
        QuadratureOscillator::INPUT_SIZE
    }
}
