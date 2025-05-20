use crate::input_expr::*;
use crate::modules::*;
use crate::synth_spec::gen_stack_program;
use crate::synth_spec::SynthSpec;
use ini::Properties;
use proc_macro2::TokenStream;
use quote::quote;
use synth_engine::modules::*;
use synth_engine::simulator::state::StateInput;
use synth_engine::modules::envelope::EnvType;
use core::ops::Range;

const MODULE_TYPE: &str = "contour";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";
const SIGNAL_OUTPUT: &str = "signal_output";
const RISE_CONTROL: &str = "rise_control";
const DECAY_CONTROL: &str = "decay_control";
const SHAPE_CONTROL: &str = "shape_control";
const INPUT_SIZE: usize = 4;
const STATE_SIZE: usize = 2;

pub struct ContourModuleSpec {
    name: String,
    inputs: Vec<Expr>,
    state_range: Range<usize>,
    input_range: Range<usize>,
}

impl ContourModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut inputs = vec![Expr::zero(); Envelope::INPUT_SIZE];
        /*
        let mut signal_in: Expr = Expr::zero();
        let mut rise_control: Expr = Expr::zero();
        let mut decay_control: Expr = Expr::zero();
        let mut shape_control: Expr = Expr::zero();
        */

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                SIGNAL_INPUT => inputs[Envelope::SIGNAL_INPUT] = Expr::parse(&v)?,
                RISE_CONTROL => inputs[Envelope::ATTACK_INPUT] = Expr::parse(&v)?,
                DECAY_CONTROL => inputs[Envelope::DECAY_INPUT] = Expr::parse(&v)?,
                SHAPE_CONTROL => inputs[Envelope::SHAPE_SELECT] = Expr::parse(&v)?,
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        Ok(Self {
            name,
            inputs,
            state_range: 0..0,
            input_range: 0..0,
        })
    }
}

impl ModuleSpec for ContourModuleSpec {
    fn allocate_state_input(&mut self, alloc: &mut StateAllocator) {
        let state_input_range = alloc.allocate(Envelope::STATE_SIZE, Envelope::INPUT_SIZE);
        self.state_range = state_input_range.state_range;
        self.input_range = state_input_range.input_range;
    }

    fn compile_input_exprs(
        &self,
        synth_spec: &SynthSpec,
        state_input: &mut StateInput,
    ) -> Result<(), ModuleError> {
        for i in 0..Envelope::INPUT_SIZE {
            let program = self.inputs[i].compile(synth_spec)?;
            let index = i + self.input_range.start;
            state_input.set_program(index, program);
        }

        Ok(())
    }
    
    fn make_module_entry(&self) -> ModuleEntry {
        ModuleEntry {
            synth_module: SynthModule::Contour(Envelope::new(EnvType::AttackRelease)),
            input: self.input_range.clone(),
            state: self.state_range.clone(),
        }
    }

    /*
    fn create_module(&self, synth_spec: &SynthSpec) -> Result<SynthModule, ModuleError> {
        let env = Envelope::new(
            self.inputs[0].compile(&synth_spec)?,
            self.inputs[1].compile(&synth_spec)?,
            self.inputs[2].compile(&synth_spec)?,
            self.inputs[3].compile(&synth_spec)?,
            self.state[0],
            self.state[1],
        );

        Ok(SynthModule::Contour(env))
    }
    */

    fn codegen(&self, synth_spec: &SynthSpec) -> TokenStream {
        /*
        let i0 = gen_stack_program(&self.inputs[0].compile(&synth_spec).unwrap());
        let i1 = gen_stack_program(&self.inputs[1].compile(&synth_spec).unwrap());
        let i2 = gen_stack_program(&self.inputs[2].compile(&synth_spec).unwrap());
        let i3 = gen_stack_program(&self.inputs[3].compile(&synth_spec).unwrap());
        let s0 = self.state[0];
        let s1 = self.state[1];
        */

        todo!()
        // quote! { SynthModule::Contour(Envelope::new(#i0, #i1, #i2, #i3, #s0, #s1)) }
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            SIGNAL_OUTPUT => Ok(self.state_range.start + Envelope::CONTROL_OUTPUT),
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
        Envelope::STATE_SIZE
    }

    fn input_size(&self) -> usize {
        Envelope::INPUT_SIZE
    }
}
