use std::collections::HashMap;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;

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
            let vca = Amplifier::new(4, 5, 6, 7);
            let osc = BowedOscillator::new(F0, 100., 5., 8, 9, 0, 0, 0);
            let fil = MoogFilter::new(F0, 10, 11, 12, 13, 0, 0, 0);
            let mid = MidiMono::new(14, 15, midi_channel);
            let env = ADEnvelope::new(0, 16, 0, 0);

            modules.insert("vca".to_string(), Box::new(vca));
            modules.insert("osc".to_string(), Box::new(osc));
            modules.insert("fil".to_string(), Box::new(fil));
            modules.insert("mid".to_string(), Box::new(mid));
            modules.insert("env".to_string(), Box::new(env));
        }
        _ => return Err(RuntimeError::UnknownModel(model_name.to_string())),
    }

    Ok(modules)
}
