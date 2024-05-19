use std::collections::HashMap;
use synth_engine::modules::*;
use synth_engine::simulator::module::Module;

use crate::RuntimeError;

pub fn make_model(
    model_name: &str,
    midi_channel: usize,
) -> Result<HashMap<String, Box<dyn Module>>, RuntimeError> {
    let mut modules = HashMap::new();

    match model_name {
        "subtractive" => {
            let vca1 = Amplifier::new(4, 5, 6, 7);
            todo!("add modules");
        }
        _ => return Err(RuntimeError::UnknownModel(model_name.to_string())),
    }

    Ok(modules)
}
