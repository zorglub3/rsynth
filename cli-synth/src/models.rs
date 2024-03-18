use synth_engine::{
    modules::{midi_mono::MidiMono, osc::RelaxationOscillator},
    simulator::{rk4::Rk4, Simulator},
};

use crate::RuntimeError;

pub fn make_model(
    model_name: &str,
    midi_channel: usize,
) -> Result<Box<dyn Simulator>, RuntimeError> {
    let simulator: Rk4;

    match model_name {
        "subtractive" => {
            let midi_in = MidiMono::new_with_connections(Some(midi_channel as u8), vec![3, 4, 5, 6]);
            let oscillator = RelaxationOscillator::new_with_connections(vec![0, 4, 0], vec![7, 8, 9, 10, 11]);

            simulator = Rk4::with_modules(vec![
                ("midi_in".to_string(), Box::new(midi_in)),
                ("oscillator".to_string(), Box::new(oscillator)),
            ]);
        }
        _ => return Err(RuntimeError::UnknownModel(model_name.to_string())),
    }

    let simulator = Box::new(simulator);

    Ok(simulator)
}
