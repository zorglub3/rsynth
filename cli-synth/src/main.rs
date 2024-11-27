use crate::audio::sound_simulation;
use crate::midi::{Midi, MidiError};
use clap::Parser;
use cpal::{BuildStreamError, PlayStreamError};
use scale::scl::SclError;
use scale::Scale;
use std::io;
use std::io::prelude::*;
use std::sync::mpsc::channel;
use synth_designer::from_ini_file;
use synth_engine::simulator::rungekutta::RungeKutta;
use thiserror::Error;

mod audio;
mod midi;

const DEFAULT_NAME: &str = "rsynth";
const DEFAULT_SAMPLE_RATE: u32 = 44100;
const DEFAULT_BUFFER_SIZE: u32 = 2048;
const DEFAULT_SIMULATOR: &str = "rk4";
const DEFAULT_BASE_PITCH: usize = 0;
const DEFAULT_PITCH_WHEEL_RANGE: f32 = 1.;
const DEFAULT_DEBUG_EVENTS: bool = false;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long)]
    channel: Option<usize>,
    #[arg(short, long, default_value_t = DEFAULT_NAME.to_string())]
    name: String,
    #[arg(short, long)]
    model: String,
    #[arg(long, default_value_t = DEFAULT_SIMULATOR.to_string())]
    simulator: String,
    #[arg(short, long, default_value_t = DEFAULT_SAMPLE_RATE)]
    sample_rate: u32,
    #[arg(short, long, default_value_t = DEFAULT_BUFFER_SIZE)]
    buffer_size: u32,
    #[arg(short, long, default_value_t = DEFAULT_PITCH_WHEEL_RANGE)]
    pitch_wheel_range: f32,
    #[arg(long)]
    scale: Option<String>,
    #[arg(long, default_value_t = DEFAULT_BASE_PITCH)]
    base_pitch: usize,
    #[arg(long, default_value_t = DEFAULT_DEBUG_EVENTS)]
    debug_events: bool,
}

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Unknown model: {0}")]
    UnknownModel(String),
    #[error("Audio setup error: {0:?}")]
    AudioError(#[from] BuildStreamError),
    #[error("Audio play error: {0:?}")]
    PlayError(#[from] PlayStreamError),
    #[error("Midi setup error: {0:?}")]
    MidiSetupError(#[from] MidiError),
    #[error("Scale error: {0:?}")]
    SclError(#[from] SclError),
}

fn make_simulator(simulator_name: &str, state_size: usize) -> RungeKutta {
    match simulator_name {
        "rk4" => RungeKutta::rk4(state_size),
        "rk38" => RungeKutta::rk38(state_size),
        "euler" => RungeKutta::euler(state_size),
        "second_order" => RungeKutta::second_order(0.5, state_size),
        _ => panic!("Unsupported simulator: {}", simulator_name),
    }
}

fn make_scale(scale: Option<String>, base_pitch: usize) -> Result<Scale, RuntimeError> {
    match scale {
        None => Ok(Scale::equal_temperament()),
        Some(filename) => Ok(Scale::from_file(&filename, base_pitch)?),
    }
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to stop...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() -> Result<(), RuntimeError> {
    let args = CliArgs::parse();

    println!("Reading model definition from {}", args.model.as_str());
    let (model, state_size) = match from_ini_file(args.model.as_str()) {
        Ok((m, s)) => (m, s),
        Err(err) => {
            panic!("Error initializing the synth model: {:?}", err);
        }
    };
    println!("done");

    println!("Getting scale...");
    let scale = make_scale(args.scale, args.base_pitch).expect("Could not get scale");
    println!("done");

    print!("Creating simulator...");
    let simulator =
        Box::new(make_simulator(args.simulator.as_str(), state_size).with_modules(model));
    println!("done");

    print!("Creating communication channel...");
    let (send, receive) = channel();
    println!("done");

    print!("Creating the simulation runner...");
    let simulation = sound_simulation(
        args.sample_rate,
        args.buffer_size,
        simulator,
        receive,
        scale,
        args.pitch_wheel_range,
        args.debug_events,
    )?;
    println!("done");

    print!("Creating the midi interface...");
    let midi = Midi::new(
        args.name.as_str(),
        args.channel.map(|x| x.try_into().unwrap()),
        send,
    )?;
    println!("done");

    println!("Running the simulation...");
    simulation.play()?;

    pause();

    println!(" ... done");

    print!("Closing midi...");
    midi.close();
    println!("done");

    Ok(())
}
