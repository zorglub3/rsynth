use std::collections::BTreeMap;
use synth_engine::stack_program::Instr;
use synth_engine::modules::wavetable::WavetableEntry;
use crate::modules::ModuleError;

pub struct SynthResource {
    pub data_buffers: BTreeMap<String, Vec<Vec<f32>>>,
    pub code_buffers: BTreeMap<String, Vec<Vec<Instr>>>,
    pub wavetable_buffers: BTreeMap<String, Vec<WavetableEntry>>,
}

impl SynthResource {
    pub fn new() -> Self {
        Self {
            data_buffers: BTreeMap::new(),
            code_buffers: BTreeMap::new(),
            wavetable_buffers: BTreeMap::new(),
        }
    }

    pub fn add_data_buffers(&mut self, module_name: &str, data_buffers: Vec<Vec<f32>>) {
        self.data_buffers.insert(module_name.to_string(), data_buffers);
    }

    pub fn add_code_buffers(&mut self, module_name: &str, code_buffers: Vec<Vec<Instr>>) {
        self.code_buffers.insert(module_name.to_string(), code_buffers);
    }

    pub fn add_wavetable_entries(&mut self, module_name: &str, wavetable_entries: Vec<WavetableEntry>) {
        todo!()
    }

    pub fn get_code_buffer(&self, module_name: &str, index: usize) -> Result<&[Instr], ModuleError> {
        let Some(buffers) = self.code_buffers.get(module_name) else {
            return Err(ModuleError::MissingModule(module_name.to_string()));
        };

        Ok(&buffers[index])
    }
}

