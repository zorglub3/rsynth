use crate::input_expr::*;
use crate::modules::*;
use crate::synth_spec::gen_stack_program;
use crate::synth_spec::SynthSpec;
use ini::Properties;
use proc_macro2::TokenStream;
use quote::quote;
use synth_engine::modules::*;
use synth_engine::simulator::state::StateInput;
use core::ops::Range;

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
    inputs: Vec<Expr>,
    f0: f32,
    data_size: usize,
    state_range: Range<usize>,
    input_range: Range<usize>,
}

impl DelayLineModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut f0: f32 = 1.;
        // let mut fc: Expr = Expr::zero();
        // let mut lc: Expr = Expr::zero();
        // let mut input: Expr = Expr::zero();
        let mut data_size: usize = DATA_SIZE_VALUE;
        let mut inputs = vec![Expr::zero(); DelayLine::INPUT_SIZE];

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                SIGNAL_INPUT => inputs[DelayLine::SIGNAL_INPUT] = Expr::parse(&v)?,
                FREQUENCY_ZERO => f0 = v.parse::<f32>()?,
                FREQUENCY_CONTROL => inputs[DelayLine::EXP_CONTROL_INPUT] = Expr::parse(&v)?,
                LINEAR_CONTROL => inputs[DelayLine::LINEAR_CONTROL_INPUT] = Expr::parse(&v)?,
                DATA_SIZE_FIELD => data_size = v.parse::<usize>()?,
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        Ok(Self {
            name,
            inputs,
            f0,
            data_size,
            state_range: 0..0,
            input_range: 0..0,
        })
    }
}

impl ModuleSpec for DelayLineModuleSpec {
    fn allocate_state_input(&mut self, alloc: &mut StateAllocator) {
        let state_input_range = alloc.allocate(DelayLine::STATE_SIZE, DelayLine::INPUT_SIZE);
        self.input_range = state_input_range.input_range;
        self.state_range = state_input_range.state_range;
    }

    fn compile_input_exprs(
        &self,
        synth_spec: &SynthSpec,
        state_input: &mut StateInput,
    ) -> Result<(), ModuleError> {
        for i in 0..DelayLine::INPUT_SIZE {
            let program = self.inputs[i].compile(synth_spec)?;
            let index = i + self.input_range.start;
            state_input.set_program(index, program);
        }
        Ok(())
    }

    fn make_module_entry(&self) -> ModuleEntry {
        let m = DelayLine::new(self.f0, self.data_size);

        ModuleEntry {
            synth_module: SynthModule::Delay(m),
            input: self.input_range.clone(),
            state: self.state_range.clone(),
        }
    }

    /*
    fn create_module(&self, synth_spec: &SynthSpec) -> Result<SynthModule, ModuleError> {
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
    */

    fn codegen(&self, synth_spec: &SynthSpec) -> TokenStream {
        /*
        let f0 = self.f0;
        let s0 = self.state[0];
        let i0 = gen_stack_program(&self.inputs[0].compile(&synth_spec).unwrap());
        let i1 = gen_stack_program(&self.inputs[1].compile(&synth_spec).unwrap());
        let i2 = gen_stack_program(&self.inputs[2].compile(&synth_spec).unwrap());
        let ds = self.data_size;
        */

        todo!()
        // quote! { SynthModule::Delay(DelayLine::new(#f0, #s0, #i0, #i1, #i2, #ds)) }
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            SIGNAL_OUTPUT => Ok(self.state_range.start + DelayLine::SIGNAL_OUTPUT),
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
        DelayLine::STATE_SIZE
    }

    fn input_size(&self) -> usize {
        DelayLine::INPUT_SIZE
    }
}
