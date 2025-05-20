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

const MODULE_TYPE: &str = "folder";
const MODULE_NAME: &str = "name";
const SIGNAL_INPUT: &str = "signal_input";
const CONTROL: &str = "control";
const SIGNAL_OUTPUT: &str = "signal_output";

pub struct FolderModuleSpec {
    name: String,
    inputs: Vec<Expr>,
    state_range: Range<usize>,
    input_range: Range<usize>,
}

impl FolderModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut inputs = vec![Expr::zero(); Folder::INPUT_SIZE];
        // let mut signal_in: Expr = Expr::zero();
        // let mut control: Expr = Expr::zero();

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                SIGNAL_INPUT => inputs[Folder::SIGNAL_INPUT] = Expr::parse(&v)?,
                CONTROL => inputs[Folder::CONTROL_INPUT] = Expr::parse(&v)?,
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

impl ModuleSpec for FolderModuleSpec {
    fn allocate_state_input(&mut self, alloc: &mut StateAllocator) {
        let state_input_range = alloc.allocate(Folder::STATE_SIZE, Folder::INPUT_SIZE);
        self.state_range = state_input_range.state_range;
        self.input_range = state_input_range.input_range;
    }

    fn compile_input_exprs(
        &self,
        synth_spec: &SynthSpec,
        state_input: &mut StateInput,
    ) -> Result<(), ModuleError> {
        for i in 0..Folder::INPUT_SIZE {
            let program = self.inputs[i].compile(synth_spec)?;
            let index = self.input_range.start + i;
            state_input.set_program(index, program);
        }

        Ok(())
    }

    fn make_module_entry(&self) -> ModuleEntry {
        let m = Folder::new();

        ModuleEntry {
            synth_module: SynthModule::Wavefolder(m),
            state: self.state_range.clone(),
            input: self.input_range.clone(),
        }
    }

    /*
    fn create_module(&self, synth_spec: &SynthSpec) -> Result<SynthModule, ModuleError> {
        let folder = Folder::new(
            self.inputs[0].compile(&synth_spec)?,
            self.inputs[1].compile(&synth_spec)?,
            self.state[0],
        );

        Ok(SynthModule::Wavefolder(folder))
    }
    */

    fn codegen(&self, synth_spec: &SynthSpec) -> TokenStream {
        /*
        let s0 = self.state[0];
        let i0 = gen_stack_program(&self.inputs[0].compile(&synth_spec).unwrap());
        let i1 = gen_stack_program(&self.inputs[1].compile(&synth_spec).unwrap());

        quote! { SynthModule::Wavefolder(Folder::new(#i0, #i1, #s0)) }
        */
        todo!()
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            SIGNAL_OUTPUT => Ok(self.state_range.start + Folder::SIGNAL_OUTPUT),
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
        Folder::STATE_SIZE
    }

    fn input_size(&self) -> usize {
        Folder::INPUT_SIZE
    }
}
