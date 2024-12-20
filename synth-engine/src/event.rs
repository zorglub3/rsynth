//! Types for events that the synth engine can receive for real time control.

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
    if bytes.len() == 0 {
        None
    } else {
        let chn = bytes[0] & 0xF;

        if channel.is_none() || Some(chn) == channel {
            use ControllerEvent::*;

            match (bytes[0] & 0xF0, bytes) {
                (MIDI_NOTE_OFF, [_, pitch, v]) => Some(NoteOff {
                    pitch: *pitch,
                    velocity: u7_to_f32(*v),
                }),
                (MIDI_NOTE_ON, [_, pitch, v]) => {
                    if *v == 0 {
                        Some(NoteOff {
                            pitch: *pitch,
                            velocity: 0.,
                        })
                    } else {
                        Some(NoteOn {
                            pitch: *pitch,
                            velocity: u7_to_f32(*v),
                            pitch_value: 0.,
                        })
                    }
                }
                (MIDI_POLY_AFTERTOUCH, [_, pitch, v]) => Some(PolyAftertouch {
                    pitch: *pitch,
                    amount: u7_to_f32(*v),
                }),
                (MIDI_CC, [_, c, v]) => Some(ContinuousControl {
                    control: *c as usize,
                    value: u7_to_f32(*v),
                }),
                (MIDI_CHANNEL_AFTERTOUCH, [_, v, ..]) => Some(Aftertouch {
                    amount: u7_to_f32(*v),
                }),
                (MIDI_PROGRAM_CHANGE, [_, _amount, ..]) => None,
                (MIDI_PITCH_WHEEL, [_, d1, d2]) => Some(PitchWheel {
                    amount: pitchwheel(*d2, *d1),
                }),
                _ => None,
            }
        } else {
            None
        }
    }
}
