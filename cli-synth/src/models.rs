use std::collections::HashMap;
use synth_engine::simulator::module::Module;
use synth_designer::modules::*;
use synth_designer::{StateAllocator, SynthModule, collection_insert};

use crate::RuntimeError;

const F0: f32 = 8.18; // MIDI note pitch zero

pub fn make_model(
    model_name: &str,
    midi_channel: usize,
) -> Result<HashMap<String, Box<dyn Module>>, RuntimeError> {
    let midi_channel = midi_channel as u8;
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

    match model_name {
        "subtractive" => {
            let mut state_allocator = StateAllocator::new(32);

            let mut osc = OscModule::new(F0, 1., 1.);
            let mut filter = FilterModule::new(F0);
            let mut amp = AmpModule::new();
            let mut mono_out = MonoOutputModule::new();
            
            let mut midi = MidiMonoModule::new(midi_channel as u8);
            let mut attack_control = MidiCCModule::new(midi_channel as u8, 1);
            let mut decay_control = MidiCCModule::new(midi_channel as u8, 2);
            let mut contour = ContourModule::new();

            osc.allocate_state(&mut state_allocator);
            filter.allocate_state(&mut state_allocator);
            amp.allocate_state(&mut state_allocator);
            mono_out.allocate_state(&mut state_allocator);
            midi.allocate_state(&mut state_allocator);
            attack_control.allocate_state(&mut state_allocator);
            decay_control.allocate_state(&mut state_allocator);
            contour.allocate_state(&mut state_allocator);
            
            osc.frequency_control().connect_to(midi.pitch_output());
            filter.signal_input().connect_to(osc.signal_output());
            amp.signal_input().connect_to(filter.signal_output());

            contour.signal_input().connect_to(midi.gate_output());
            contour.rise_control_input().connect_to(attack_control.value_output());
            contour.fall_control_input().connect_to(decay_control.value_output());

            filter.frequency_control().connect_to(contour.signal_output());
            amp.exponential_control_input().connect_to(contour.signal_output());

            mono_out.signal_input().connect_to(amp.signal_output());

            collection_insert(&mut modules, osc);
            collection_insert(&mut modules, filter);
            collection_insert(&mut modules, amp);
            collection_insert(&mut modules, mono_out);
            collection_insert(&mut modules, midi);
            collection_insert(&mut modules, attack_control);
            collection_insert(&mut modules, decay_control);
            collection_insert(&mut modules, contour);
        }
        _ => return Err(RuntimeError::UnknownModel(model_name.to_string())),
    }

    Ok(modules)
}
