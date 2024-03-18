use crate::midi::message::MidiMessage;
use crate::modules::Module;
use std::fmt::{Debug, Error, Formatter};

pub enum Event {
    MidiEvent {
        event: MidiMessage,
        channel: u8,
    },
    SetState {
        index: usize,
        value: f32,
    },
    AddModule {
        id: String,
        module: Box<dyn Module>,
    },
    DeleteModule {
        id: String,
    },
    ConnectModules {
        source_id: String,
        source_index: usize,
        target_id: String,
        target_index: usize,
    },
    DisconnectModules {
        target_id: String,
        target_index: usize,
    },
    SetConstant {
        module_id: String,
        module_index: String,
        value: f32,
    },
}

impl Debug for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Event::MidiEvent { event, channel } => {
                f.write_str(&format!("MidiEvent({event:?}, {channel})"))
            }
            _ => f.write_str("blah"),
        }
    }
}
