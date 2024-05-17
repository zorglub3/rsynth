use synth_engine::simulator::module::Module;
use std::collections::HashMap;

use crate::RuntimeError;

pub fn make_model(
    model_name: &str,
    midi_channel: usize,
) -> Result<HashMap<String, Box<dyn Module>>, RuntimeError> {
    let mut modules = HashMap::new();

    match model_name {
        "subtractive" => {
            todo!("add modules");
        }
        _ => return Err(RuntimeError::UnknownModel(model_name.to_string())),
    }

    Ok(modules)
}
