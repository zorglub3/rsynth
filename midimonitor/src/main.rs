use clap::Parser;
use midir::MidiInput;
use midir::os::unix::VirtualInput;
use std::process::exit;
use synth_engine::midi::message::MidiMessage;
use std::io;
use std::io::prelude::*;

#[derive(Parser, Debug, Clone)]
struct CliArgs {
    #[arg(short, long)]
    name: String,
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to stop...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    let args = CliArgs::parse();

    let input = match MidiInput::new(&args.name) {
        Ok(midi_input) => midi_input,
        Err(err) => {
            println!("Midi error: {:?}", err);
            exit(1);
        }
    };

    let conn = match input.create_virtual(&args.name, move |_, message, _| {
        if let Some( (message, channel) ) = MidiMessage::decode(message) {
            println!("Channel: {}, Message: {:?}", channel, message);
        } else {
            println!("Malformed midi message: {:?}", message);
    }}, ()) {
        Ok(conn) => conn,
        Err(err) => {
            println!("Midi connection error: {:?}", err);
            exit(2);
        }
    };

    pause();

    conn.close();
}
