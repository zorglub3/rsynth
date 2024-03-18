pub type MidiChannel = u8;

#[derive(Clone, Copy, Debug)]
pub enum MidiMessage {
    NoteOn { pitch: u8, velocity: u8 },
    NoteOff { pitch: u8, velocity: u8 },
    PolyphonicAftertouch { pitch: u8, amount: u8 },
    ChannelAftertouch { amount: u8 },
    ContinuousControl { control: u8, value: u8 },
    ProgramChange { program: u8 },
    PitchWheel { amount: u16 },
}

fn make_u16(msb: u8, lsb: u8) -> u16 {
    ((msb as u16) << 8) | (lsb as u16)
}

impl MidiMessage {
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

    pub fn decode(bytes: &[u8]) -> Option<(Self, MidiChannel)> {
        match bytes {
            [status, data1, data2] => MidiMessage::decode_data_message(*status, *data1, *data2),
            _ => None,
        }
    }
}
