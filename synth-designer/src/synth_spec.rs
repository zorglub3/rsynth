use crate::modules::ModuleError;
use crate::modules::ModuleSpec;
use crate::modules::*;
use crate::state_allocator::StateAllocator;
use crate::SynthError;
use ini::Ini;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;
use synth_engine::simulator::module::Module;
use synth_engine::stack_program::Function;
use synth_engine::stack_program::Instr;
use synth_engine::stack_program::StackProgram;

pub struct SynthSpec(BTreeMap<String, Box<dyn ModuleSpec>>);

impl SynthSpec {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn add_module(&mut self, module_spec: Box<dyn ModuleSpec>) -> Result<(), ModuleError> {
        let key = module_spec.get_name().to_string();

        if self.0.contains_key(&key) {
            Err(ModuleError::ModuleNameClash(key))
        } else {
            self.0.insert(key, module_spec);
            Ok(())
        }
    }

    pub fn input_state_index(
        &self,
        module_name: &str,
        module_field: &str,
    ) -> Result<usize, ModuleError> {
        let module_spec = self
            .0
            .get(module_name)
            .ok_or(ModuleError::MissingModule(module_name.to_string()));

        module_spec?.state_index(module_field)
    }

    pub fn model_size(&self) -> usize {
        self.0.len()
    }

    fn state_size(&self) -> usize {
        let mut state_size: usize = 0;

        for (_k, v) in self.0.iter() {
            state_size = state_size + v.state_size();
        }

        state_size
    }

    pub fn allocate_state(&mut self) -> usize {
        let size = self.state_size();

        let mut state_allocator = StateAllocator::new(size);

        for (_k, v) in self.0.iter_mut() {
            v.allocate_state(&mut state_allocator)
        }

        size
    }

    pub fn make_modules(&self, modules: &mut Vec<Box<dyn Module>>) -> Result<(), ModuleError> {
        for (_k, v) in self.0.iter() {
            modules.push(v.create_module(self)?);
        }

        Ok(())
    }

    pub fn codegen(&self) -> TokenStream {
        let mut module_code: Vec<TokenStream> = Vec::new();

        for (_k, v) in self.0.iter() {
            module_code.push(v.codegen(self));
        }

        let synth_state_size = self.state_size();

        quote! { ([#(#module_code),*], #synth_state_size) }
    }

    pub fn from_ini_file(filename: &str) -> Result<Self, SynthError> {
        let spec_file = Ini::load_from_file(filename).map_err(|e| SynthError::FileError(e))?;
        let mut synth_spec = SynthSpec::new();

        for (section, props) in spec_file {
            if let Some(section) = section {
                match &section.to_lowercase()[..] {
                    "amplifier" => {
                        let module_spec = AmpModuleSpec::from_ini_properties(props)?;
                        synth_spec.add_module(Box::new(module_spec))?;
                    }
                    "contour" => {
                        let module_spec = ContourModuleSpec::from_ini_properties(props)?;
                        synth_spec.add_module(Box::new(module_spec))?;
                    }
                    "filter_24db" => {
                        let module_spec = Filter24dbModuleSpec::from_ini_properties(props)?;
                        synth_spec.add_module(Box::new(module_spec))?;
                    }
                    "filter_12db" => {
                        let module_spec = Filter12dbModuleSpec::from_ini_properties(props)?;
                        synth_spec.add_module(Box::new(module_spec))?;
                    }
                    "filter_6db" => {
                        let module_spec = Filter6dbModuleSpec::from_ini_properties(props)?;
                        synth_spec.add_module(Box::new(module_spec))?;
                    }
                    "midi_cc" => {
                        let module_spec = MidiCCModuleSpec::from_ini_properties(props)?;
                        synth_spec.add_module(Box::new(module_spec))?;
                    }
                    "midi_mono" => {
                        let module_spec = MidiMonoModuleSpec::from_ini_properties(props)?;
                        synth_spec.add_module(Box::new(module_spec))?;
                    }
                    "mono_out" => {
                        let module_spec = MonoOutputModuleSpec::from_ini_properties(props)?;
                        synth_spec.add_module(Box::new(module_spec))?;
                    }
                    "bowed_oscillator" => {
                        let module_spec = BowedOscillatorModuleSpec::from_ini_properties(props)?;
                        synth_spec.add_module(Box::new(module_spec))?;
                    }
                    "saw_oscillator" => {
                        let module_spec = SawOscillatorModuleSpec::from_ini_properties(props)?;
                        synth_spec.add_module(Box::new(module_spec))?;
                    }
                    "wavetable" => {
                        let module_spec =
                            WavetableOscillatorModuleSpec::from_ini_properties(props)?;
                        synth_spec.add_module(Box::new(module_spec))?;
                    }
                    "quadrature_oscillator" => {
                        let module_spec = QuadOscillatorModuleSpec::from_ini_properties(props)?;
                        synth_spec.add_module(Box::new(module_spec))?;
                    }
                    "delay_line" => {
                        let module_spec = DelayLineModuleSpec::from_ini_properties(props)?;
                        synth_spec.add_module(Box::new(module_spec))?;
                    }
                    "noise" => {
                        let module_spec = NoiseGeneratorModuleSpec::from_ini_properties(props)?;
                        synth_spec.add_module(Box::new(module_spec))?;
                    }
                    x => return Err(SynthError::UnknownModule(x.to_string())),
                }
            } else {
                for (k, v) in props.iter() {
                    println!("{}: {}", k, v);
                }
            }
        }

        Ok(synth_spec)
    }
}

pub fn gen_stack_program(stack_program: &StackProgram) -> TokenStream {
    let mut prg: Vec<TokenStream> = Vec::new();

    for instr in &stack_program.code {
        use Instr::*;

        match instr {
            Add => prg.push(quote! { Add }),
            Subtract => prg.push(quote! { Subtract }),
            Multiply => prg.push(quote! { Multiply }),
            Divide => prg.push(quote! { Divide }),
            Negate => prg.push(quote! { Negate }),
            Const(v) => prg.push(quote! { Const(#v) }),
            State(s) => prg.push(quote! { State(#s) }),
            Call(f) => {
                use Function::*;

                match f {
                    Sin => prg.push(quote! { Call(Sin) }),
                    Cos => prg.push(quote! { Call(Cos) }),
                    Tan => prg.push(quote! { Call(Tan) }),
                    Tanh => prg.push(quote! { Call(Tanh) }),
                    Ln => prg.push(quote! { Call(Ln) }),
                    Exp => prg.push(quote! { Call(Exp) }),
                    Logistic => prg.push(quote! { Call(Logistic) }),
                    Abs => prg.push(quote! { Call(Abs) }),
                    Min => prg.push(quote! { Call(Min) }),
                    Max => prg.push(quote! { Call(Max) }),
                    Lerp => prg.push(quote! { Call(Lerp) }),
                }
            }
        }
    }

    let stack_size = stack_program.stack_size;

    quote! { StackProgram::new(vec![#(#prg),*], #stack_size) }
}
