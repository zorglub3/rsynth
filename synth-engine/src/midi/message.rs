//! Module to represent MIDI messages and also decode such messages from bytes.

/// Type alias for MIDI channel.
///
/// TODO This uses 4 bits too many. There are only 16 MIDI channels
pub type MidiChannel = u8;

/// Most of the MIDI messages a synth should understand.
#[derive(Clone, Copy, Debug)]
pub enum MidiMessage {
    /// The start of a note.
    NoteOn { pitch: u8, velocity: u8 },
    /// The end of a note. Most MIDI controllers will set `velocity` to zero
    /// so don't rely on it being any other value (unless using custom controller)    
    NoteOff { pitch: u8, velocity: u8 },
    /// Aftertouch as sent by polyphonic controllers. Note that a lot of
    /// controllers that send aftertouch, will do so as _channel_ aftertouch. This
    /// is sent as a MIDI continuous control message or as a channel aftertouch
    /// message.
    PolyphonicAftertouch { pitch: u8, amount: u8 },
    /// Aftertouch for the entire channel. Should not be sent by a controller
    /// that also sends poly aftertouch (but no guarantees).
    ChannelAftertouch { amount: u8 },
    /// Can control anything. There is a standard for what each `control` value
    /// means, but consult your manuals to see how different synthesizers actually
    /// interpret these messages.
    ContinuousControl { control: u8, value: u8 },
    /// Change to a different sound/patch.
    ProgramChange { program: u8 },
    /// Pitch bend control. Amount of bend depends on the synth, but should be
    /// symmetrical.
    PitchWheel { amount: u16 },
}

fn make_u16(msb: u8, lsb: u8) -> u16 {
    ((msb as u16) << 8) | (lsb as u16)
}

impl MidiMessage {
    /// Parse three bytes of data to a MIDI message. Return `None`, if there is no valid parse
    /// or if the message type is not supported.
    fn decode_data_message(status: u8, data1: u8, data2: u8) -> Option<(Self, MidiChannel)> {
        let channel = status & 0x0F;

        match status & 0xF0 {
            0x80 => Some((
                MidiMessage::NoteOff {
                    pitch: data1,
                    velocity: data2,
                },
                channel,
            )),
            0x90 => Some((
                MidiMessage::NoteOn {
                    pitch: data1,
                    velocity: data2,
                },
                channel,
            )),
            0xA0 => Some((
                MidiMessage::PolyphonicAftertouch {
                    pitch: data1,
                    amount: data2,
                },
                channel,
            )),
            0xB0 => Some((
                MidiMessage::ContinuousControl {
                    control: data1,
                    value: data2,
                },
                channel,
            )),
            0xD0 => Some((MidiMessage::ChannelAftertouch { amount: data1 }, channel)),
            0xC0 => Some((MidiMessage::ProgramChange { program: data1 }, channel)),
            0xE0 => Some((
                MidiMessage::PitchWheel {
                    amount: make_u16(data2, data1),
                },
                channel,
            )),
            _ => None,
        }
    }

    /// Parse the bytes in a slice as MIDI message. Return `None` if there is no parse or if the
    /// message type is not supported.
    pub fn decode(bytes: &[u8]) -> Option<(Self, MidiChannel)> {
        match bytes {
            [status, data1, data2] => MidiMessage::decode_data_message(*status, *data1, *data2),
            _ => None,
        }
    }
}
