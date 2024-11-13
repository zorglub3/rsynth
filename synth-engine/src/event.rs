//! Types for events that the synth engine can receive for real time control.

/*
use crate::midi::message::MidiMessage;
use crate::simulator::module::Module;
use std::fmt::{Debug, Error, Formatter};

/// The basic type for event that the synth engine "understands"
pub enum Event {
    /// A MIDI event as it is received from a controller or sequencer.
    MidiEvent { event: MidiMessage, channel: u8 },
    /// Set the state of some parameter in the current state of the
    /// simulation. This is both for testing, but also to set fixed
    /// constants as if required for some modules.
    SetState { index: usize, value: f32 },
    /// Add a new module to the synth engine. The engine should eventually
    /// support reconfiguration of the modular setup while it is running.
    AddModule {
        /// Identifier for the new module
        id: String,
        /// The actual module as it is used in the synth engine simulation.
        module: Box<dyn Module>,
    },
    /// Remove a module from the synth engine while it is playing. This
    /// should also remove any connections to/from the deleted module.
    /// If the module has already been removed or if it doesn't exist at all,
    /// then this event has no effect.
    DeleteModule {
        /// The identifier for the module to remove.
        id: String,
    },
    /// Connect one output from a module to the input of a module. There
    /// are no other restrictions. A module can even take its own output as
    /// input.
    ///
    /// One output can be connected to several inputs.
    ConnectModules {
        /// Identifier for the source/output module
        source_id: String,
        /// Index of the specific output of the source module
        source_index: usize,
        /// Identifier for the target/input module.
        target_id: String,
        /// Index of the input parameter to connect to.
        target_index: usize,
    },
    /// Disconnect one input to a module.
    DisconnectModules {
        /// Identifier for the module
        target_id: String,
        /// Identifier of the parameter of the target module to disconnect.
        target_index: usize,
    },
    /// Set some constant for some module. TODO remove this event - all
    /// such constants should be part of the state vector and should be set with
    /// the [SetState] event.
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
*/

#[derive(Debug, Clone, Copy)]
pub enum ControllerEvent {
    NoteOn {
        pitch: u8,
        velocity: f32,
        pitch_value: f32,
    },
    NoteOff {
        pitch: u8,
        velocity: f32,
    },
    Aftertouch {
        amount: f32,
    },
    PolyAftertouch {
        pitch: u8,
        amount: f32,
    },
    ContinuousControl {
        control: usize,
        value: f32,
    },
    PitchWheel {
        amount: f32,
    },
}

const MIDI_NOTE_OFF: u8 = 0x80;
const MIDI_NOTE_ON: u8 = 0x90;
const MIDI_POLY_AFTERTOUCH: u8 = 0xA0;
const MIDI_CC: u8 = 0xB0;
const MIDI_CHANNEL_AFTERTOUCH: u8 = 0xD0;
const MIDI_PROGRAM_CHANGE: u8 = 0xC0;
const MIDI_PITCH_WHEEL: u8 = 0xE0;
const PITCH_WHEEL_MID: u16 = 0x2000;

fn u7_to_f32(v: u8) -> f32 {
    ((v & 0x7F) as f32) / (0x7F as f32)
}

fn pitchwheel(msb: u8, lsb: u8) -> f32 {
    let x: u16 = (((msb & 0x7F) as u16) << 7) | ((lsb & 0x7F) as u16);
    ((x as f32) - (PITCH_WHEEL_MID as f32)) / (PITCH_WHEEL_MID as f32)
}

pub fn decode_midi_bytes(bytes: &[u8], channel: Option<u8>) -> Option<ControllerEvent> {
    let mut bytes_mut = match bytes.len() {
        2 | 3 => [0_u8; 3],
        _ => return None,
    };

    bytes_mut.copy_from_slice(bytes);

    if channel.is_none() || Some(bytes_mut[0] & 0xF) == channel {
        bytes_mut[0] &= 0xF0;

        use ControllerEvent::*;

        match bytes_mut {
            [MIDI_NOTE_OFF, pitch, v] => Some(NoteOff {
                pitch,
                velocity: u7_to_f32(v),
            }),
            [MIDI_NOTE_ON, pitch, v] => Some(NoteOn {
                pitch,
                velocity: u7_to_f32(v),
                pitch_value: 0.,
            }),
            [MIDI_POLY_AFTERTOUCH, pitch, v] => Some(PolyAftertouch {
                pitch,
                amount: u7_to_f32(v),
            }),
            [MIDI_CC, c, v] => Some(ContinuousControl {
                control: c as usize,
                value: u7_to_f32(v),
            }),
            [MIDI_CHANNEL_AFTERTOUCH, v, ..] => Some(Aftertouch {
                amount: u7_to_f32(v),
            }),
            [MIDI_PROGRAM_CHANGE, amount, ..] => None,
            [MIDI_PITCH_WHEEL, d1, d2] => Some(PitchWheel {
                amount: pitchwheel(d1, d2),
            }),
            _ => None,
        }
    } else {
        None
    }
}
