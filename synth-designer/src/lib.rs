pub mod modules;

use crate::modules::*;
use core::ops::Range;
use ini::Ini;
use std::collections::HashMap;
use synth_engine::simulator::module::Module;

#[derive(Debug)]
pub enum SynthError {
    FileError(ini::Error),
    ModuleError(ModuleError),
    UnknownModule(String),
}

impl From<ini::Error> for SynthError {
    fn from(err: ini::Error) -> Self {
        Self::FileError(err)
    }
}

impl From<ModuleError> for SynthError {
    fn from(err: ModuleError) -> Self {
        Self::ModuleError(err)
    }
}

pub struct StateAllocator(Range<usize>);

impl StateAllocator {
    pub fn new(size: usize) -> Self {
        let size = size.max(16);
        Self(0..size)
    }

    fn allocate_state_slot(&mut self) -> usize {
        if self.0.is_empty() {
            self.0 = Range {
                start: self.0.start,
                end: self.0.end * 2,
            };
        }

        let slot = self.0.start;

        self.0 = Range {
            start: self.0.start + 1,
            end: self.0.end,
        };

        slot
    }

    pub fn allocate(&mut self, state: &mut [usize]) {
        for i in 0..state.len() {
            state[i] = self.allocate_state_slot();
        }
    }
}

pub fn from_ini_file(
    filename: &str,
) -> Result<(HashMap<String, Box<dyn Module>>, usize), SynthError> {
    let mut modules = HashMap::new();
    let spec_file = Ini::load_from_file(filename).map_err(|e| SynthError::FileError(e))?;
    let mut synth_spec = SynthSpec::new();

    synth_spec.add_zero_module();

    for (section, props) in spec_file {
        if let Some(section) = section {
            match &section.to_lowercase()[..] {
                "amplifier" => {
                    let module_spec = AmpModuleSpec::from_ini_properties(props)?;
                    synth_spec.add_module(Box::new(module_spec));
                }
                "contour" => {
                    let module_spec = ContourModuleSpec::from_ini_properties(props)?;
                    synth_spec.add_module(Box::new(module_spec));
                }
                "lp_filter_24db" => {
                    let module_spec = LpFilter24dbModuleSpec::from_ini_properties(props)?;
                    synth_spec.add_module(Box::new(module_spec));
                }
                "filter_12db" => {
                    let module_spec = Filter12dbModuleSpec::from_ini_properties(props)?;
                    synth_spec.add_module(Box::new(module_spec));
                }
                "midi_cc" => {
                    let module_spec = MidiCCModuleSpec::from_ini_properties(props)?;
                    synth_spec.add_module(Box::new(module_spec));
                }
                "midi_mono" => {
                    let module_spec = MidiMonoModuleSpec::from_ini_properties(props)?;
                    synth_spec.add_module(Box::new(module_spec));
                }
                "mono_out" => {
                    let module_spec = MonoOutputModuleSpec::from_ini_properties(props)?;
                    synth_spec.add_module(Box::new(module_spec));
                }
                "oscillator" => {
                    let module_spec = OscillatorModuleSpec::from_ini_properties(props)?;
                    synth_spec.add_module(Box::new(module_spec));
                }
                "saw_oscillator" => {
                    let module_spec = SawOscillatorModuleSpec::from_ini_properties(props)?;
                    synth_spec.add_module(Box::new(module_spec));
                }
                x => return Err(SynthError::UnknownModule(x.to_string())),
            }
        } else {
            for (k, v) in props.iter() {
                println!("{}: {}", k, v);
            }
        }
    }

    let state_size = synth_spec.allocate_state();

    synth_spec.make_modules(&mut modules)?;

    Ok((modules, state_size))
}
