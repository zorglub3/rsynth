use crate::audio::sound_simulation;
use crate::midi::{Midi, MidiError};
use crate::models::make_model;
use clap::Parser;
use cpal::{BuildStreamError, PlayStreamError};
use std::io;
use std::io::prelude::*;
use std::sync::mpsc::channel;
use thiserror::Error;

mod audio;
mod midi;
mod models;

const DEFAULT_NAME: &str = "rsynth";
const DEFAULT_MODEL: &str = "subtractive";
const DEFAULT_SAMPLE_RATE: u32 = 44100;
const DEFAULT_BUFFER_SIZE: u32 = 2048;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long)]
    channel: Option<usize>,
    #[arg(short, long, default_value_t = DEFAULT_NAME.to_string())]
    name: String,
    #[arg(short, long, default_value_t = DEFAULT_MODEL.to_string())]
    model: String,
    #[arg(short, long, default_value_t = DEFAULT_SAMPLE_RATE)]
    sample_rate: u32,
    #[arg(short, long, default_value_t = DEFAULT_BUFFER_SIZE)]
    buffer_size: u32,
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
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to stop...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() -> Result<(), RuntimeError> {
    let args = CliArgs::parse();

    print!("Creating the synth model...");
    let model = make_model(args.model.as_str(), args.channel.unwrap_or(0))?;
    println!("done");

    print!("Creating communication channel...");
    let (send, receive) = channel();
    println!("done");

    print!("Creating the simulation runner...");
    let simulation = sound_simulation(args.sample_rate, args.buffer_size, model, receive)?;
    println!("done");

    print!("Creating the midi interface...");
    let midi = Midi::new(args.name.as_str(), send)?;
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
