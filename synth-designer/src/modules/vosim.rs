use crate::codegen::Codegen;
use crate::input_expr::*;
use crate::modules::wavetable::load_wavetable;
use crate::modules::*;
use crate::synth_resource::SynthResource;
use crate::synth_spec::SynthSpec;
use crate::DEFAULT_FREQUENCY_ZERO;
use ini::Properties;
use proc_macro2::TokenStream;
use quote::quote;
use synth_engine::modules::vosim::VosimOscillator;
use synth_engine::stack_program::StackProgram;

const MODULE_TYPE: &str = "wavetable_oscillator";
const MODULE_NAME: &str = "name";
const FREQUENCY_CONTROL: &str = "frequency_control";
const FREQUENCY_ZERO: &str = "frequency_zero";
const LINEAR_CONTROL: &str = "linear_modulation";
const GRAIN_FREQUENCY_CONTROL: &str = "grain_frequency_control";
const GRAIN_LINEAR_CONTROL: &str = "grain_linear_modulation";
const SCAN_CONTROL: &str = "scan_control";
const WAVETABLE_FIELD: &str = "wavetable";
const SIGNAL_OUTPUT: &str = "signal_output";
const INPUT_SIZE: usize = 5;
const STATE_SIZE: usize = 2;

pub struct VosimOscillatorModuleSpec {
    name: String,
    inputs: [Expr; INPUT_SIZE],
    state: [usize; STATE_SIZE],
    f0: f32,
    wavetables: Vec<Vec<f32>>,
}

/*
fn load_wavetable(filename: &str) -> Result<Vec<f32>, ModuleError> {
    let mut reader = hound::WavReader::open(filename)?;
    let mut result: Vec<f32> = Vec::new();

    for sample in reader.samples::<i16>() {
        let sample = ((sample?) as f32) / (i16::MAX as f32);
        result.push(sample);
    }

    Ok(result)
}
*/

impl VosimOscillatorModuleSpec {
    pub fn from_ini_properties(props: Properties) -> Result<Self, ModuleError> {
        let mut name: String = MODULE_TYPE.to_string();
        let mut f0: f32 = DEFAULT_FREQUENCY_ZERO;
        let mut fc: Expr = Expr::zero();
        let mut gfc: Expr = Expr::zero();
        let mut lc: Expr = Expr::zero();
        let mut glc: Expr = Expr::zero();
        let mut sc: Expr = Expr::zero();
        let mut wavetables: Vec<Vec<f32>> = Vec::new();

        for (k, v) in props {
            match k.as_str() {
                MODULE_NAME => name = v.to_string(),
                FREQUENCY_CONTROL => fc = Expr::parse(&v)?,
                GRAIN_FREQUENCY_CONTROL => gfc = Expr::parse(&v)?,
                LINEAR_CONTROL => lc = Expr::parse(&v)?,
                GRAIN_LINEAR_CONTROL => glc = Expr::parse(&v)?,
                SCAN_CONTROL => sc = Expr::parse(&v)?,
                FREQUENCY_ZERO => f0 = v.parse::<f32>()?,
                WAVETABLE_FIELD => wavetables.push(load_wavetable(&v)?),
                _ => return Err(ModuleError::InvalidField(MODULE_TYPE.to_string(), k)),
            }
        }

        if wavetables.len() == 0 {
            Err(ModuleError::MissingField(
                MODULE_TYPE.to_string(),
                WAVETABLE_FIELD.to_string(),
            ))
        } else {
            Ok(Self {
                name,
                inputs: [fc, lc, sc, gfc, glc],
                state: [0; STATE_SIZE],
                f0,
                wavetables,
            })
        }
    }
}

/*
fn codegen_table_data(data: &WavetableData) -> TokenStream {
    let samples = data.samples.clone();
    let len = data.len_f32;

    quote! {
        VosimData {
            samples: vec![ #(#samples),* ],
            len_f32: #len,
        }
    }
}

fn codegen_table_entries(entries: &Vec<WavetableEntry>) -> Vec<TokenStream> {
    entries
        .into_iter()
        .map(|entry| {
            let data: Vec<TokenStream> = (&entry.data)
                .into_iter()
                .map(|data| codegen_table_data(&data))
                .collect();
            let len = entry.base_data_len;

            quote! {
                VosimEntry {
                    data: vec![ #(#data),* ],
                    base_data_len: #len,
                }
            }
        })
        .collect()
}
*/

impl ModuleSpec for VosimOscillatorModuleSpec {
    fn allocate_state(&mut self, alloc: &mut StateAllocator) {
        alloc.allocate(&mut self.state);
    }

    fn create_module<'a>(
        &self,
        synth_resource: &'a SynthResource,
    ) -> Result<SynthModule<'a>, ModuleError> {
        let module = VosimOscillator::new_with_precompute(
            self.f0,
            self.state[0],
            self.state[1],
            StackProgram::new_compute_stack_size(
                &synth_resource.get_code_buffer(self.get_name(), 0)?,
            ),
            StackProgram::new_compute_stack_size(
                &synth_resource.get_code_buffer(self.get_name(), 1)?,
            ),
            StackProgram::new_compute_stack_size(
                &synth_resource.get_code_buffer(self.get_name(), 3)?,
            ),
            StackProgram::new_compute_stack_size(
                &synth_resource.get_code_buffer(self.get_name(), 4)?,
            ),
            StackProgram::new_compute_stack_size(
                &synth_resource.get_code_buffer(self.get_name(), 2)?,
            ),
            todo!("get wavetable resources"),
        );

        Ok(SynthModule::VosimOscillator(module))
    }

    fn create_resources(
        &self,
        synth_spec: &SynthSpec,
        synth_resources: &mut SynthResource,
    ) -> Result<(), ModuleError> {
        let mut code_buffers = Vec::new();

        for input in &self.inputs {
            code_buffers.push(input.compile_to_instructions(synth_spec)?);
        }

        synth_resources.add_code_buffers(self.get_name(), code_buffers);

        todo!("make the wavetable resources");

        Ok(())
    }

    fn codegen(&self, synth_spec: &SynthSpec, codegen: &mut Codegen) -> TokenStream {
        let f0 = self.f0;
        let s0 = self.state[0];
        let s1 = self.state[1];
        let i0 = codegen.add_stack_program(&self.inputs[0], &synth_spec);
        let i1 = codegen.add_stack_program(&self.inputs[1], &synth_spec);
        let i2 = codegen.add_stack_program(&self.inputs[2], &synth_spec);
        let i3 = codegen.add_stack_program(&self.inputs[3], &synth_spec);
        let i4 = codegen.add_stack_program(&self.inputs[4], &synth_spec);
        let wavetables = codegen.add_wavetables(&self.wavetables);

        quote! { SynthModule::Vosim(Vosim::new_with_precompute(
            #f0, #s0, #s1, #i0, #i1, #i3, #i4, #i2, #wavetables
        )) }
    }

    fn state_index(&self, state_field: &str) -> Result<usize, ModuleError> {
        match state_field {
            SIGNAL_OUTPUT => Ok(self.state[1]),
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
