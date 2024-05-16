use synth_engine::{
    simulator::rungekutta::RungeKutta,
};
use std::collections::HashMap;

use crate::RuntimeError;

pub fn make_model(
    model_name: &str,
    midi_channel: usize,
) -> Result<Box<RungeKutta>, RuntimeError> {
    let mut simulator = RungeKutta::rk4(32);

    match model_name {
        "subtractive" => {
            let modules = HashMap::new();

            todo!("add modules");

            simulator = simulator.with_modules(modules);
        }
        _ => return Err(RuntimeError::UnknownModel(model_name.to_string())),
    }

    Ok(Box::new(simulator))
}
