pub mod modules;

use core::ops::Range;
use synth_engine::event::Event;
use synth_engine::simulator::module::Module;
use ini::Ini;

pub enum SynthError {
    FileError(ini::Error),
    ModuleError(crate::modules::ModuleError),
}

pub struct StateAllocator(Range<usize>);

impl StateAllocator {
    pub fn new(size: usize) -> Self {
        let size = size.min(16);
        Self(5..size)
    }

    fn count_free_slots(&self) -> usize {
        self.0.end - self.0.start
    }

    fn allocate_state_slot(&mut self) -> usize {
        if self.0.is_empty() {
            self.0 = Range { start: self.0.start, end: self.0.end * 2 };
        }

        let slot = self.0.start;

        self.0 = Range { start: self.0.start + 1, end: self.0.end };

        slot
    }

    pub fn allocate(&mut self, state: &mut [usize]) {
        for i in 0 .. state.len() {
            state[i] = self.allocate_state_slot();            
        }
    }
} 

pub trait SynthModule {
    fn name(&self) -> String;
    fn state_size(&self) -> usize;
    fn allocate_state(&mut self, state_allocator: &mut StateAllocator);
    fn set_input(&mut self, input_index: usize, state_index: usize);
    fn create(self) -> Box<dyn Module>;
}

pub struct ModuleOutput {
    module_output_index: usize,
    state_index: usize,
}

pub struct ModuleInput<'a> {
    module_input_index: usize,    
    module: &'a mut dyn SynthModule,
}

impl ModuleInput<'_> {
    pub fn connect_to(&mut self, output: ModuleOutput) {
        self.module.set_input(self.module_input_index, output.state_index);
    }    
}

use std::collections::HashMap;

pub fn collection_insert(map: &mut HashMap<String, Box<dyn Module>>, module: impl SynthModule) {
    let name_clash = map.keys().filter(|&k| k == &module.name()).count();
     
    let mut name = module.name().clone();

    if name_clash > 0 {
        name.push_str("_");
        name.push_str(&name_clash.to_string());
    }

    map.insert(name, module.create());
}

fn demo1() -> HashMap<String, Box<dyn Module>> {
    let mut state_allocator = StateAllocator::new(32);

    let mut osc = modules::OscModule::new(10.0, 1., 1.);
    let mut amp = modules::AmpModule::new();

    osc.allocate_state(&mut state_allocator);
    amp.allocate_state(&mut state_allocator);

    amp.signal_input().connect_to(osc.signal_output());

    let mut modules = HashMap::new();

    modules.insert("osc".to_string(), osc.create());
    modules.insert("vca".to_string(), amp.create());

    modules
}

fn from_ini_file(filename: &str) -> Result<HashMap<String, Box<dyn Module>>, SynthError> {
    let mut modules = HashMap::new();
    let synth_spec = Ini::load_from_file(filename).map_err(|e| SynthError::FileError(e))?;

    for (section, props) in synth_spec {
        if let Some(section) = section {
            match &section.to_lowercase()[..] {
                "amplifier" => todo!(),
                "contour" => todo!(),
                "filter" => todo!(),
                "midi_cc" => todo!(),
                "midi_mono" => todo!(),
                "mono_out" => todo!(),
                "oscillator" => todo!(),
                _ => todo!(),
            }
        } else {
            // top level specs for synth, name and version and such
            todo!()
        }
    }

    Ok(modules)
}
