use midir::os::unix::VirtualInput;
use midir::{MidiInput, MidiInputConnection};
use std::error::Error;
use std::fmt;
use std::sync::mpsc::Sender;
use synth_engine::event::decode_midi_bytes;
use synth_engine::event::ControllerEvent;
// use synth_engine::event::Event;
// use synth_engine::midi::message::MidiMessage;

// TODO cleanup
#[derive(Debug)]
pub enum MidiError {
    InputFail(String),
    ConnectFail(String),
}

impl fmt::Display for MidiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MidiError::InputFail(s) => write!(f, "Midi error, input failure: {s}"),
            MidiError::ConnectFail(s) => write!(f, "Midi error, connection failure: {s}"),
        }
    }
}

impl Error for MidiError {}

pub struct Midi {
    conn: MidiInputConnection<()>,
}

impl Midi {
    pub fn new(
        name: &str,
        channel: Option<u8>,
        sender: Sender<ControllerEvent>,
    ) -> Result<Self, MidiError> {
        let input = MidiInput::new(name).map_err(|err| MidiError::InputFail(err.to_string()))?;

        let conn = input
            .create_virtual(
                name,
                move |_, message, _| {
                    if let Some(event) = decode_midi_bytes(message, channel) {
                        // if let Some((message, channel)) = MidiMessage::decode(message) {
                        let _ = sender.send(event);
                        /*
                        let _ = sender.send(Event::MidiEvent {
                            event: message,
                            channel,
                        });
                        */
                    } /*else {
                          println!("Undecodable midi message: {:?}", message);
                      }*/
                },
                (),
            )
            .map_err(|err| MidiError::ConnectFail(err.to_string()))?;

        Ok(Midi { conn })
    }

    pub fn close(self) {
        self.conn.close();
    }
}
