pub const MIDI_NOTE_OFF: u8 = 0x80;
pub const MIDI_NOTE_ON: u8 = 0x90;
pub const MIDI_POLY_AFTERTOUCH: u8 = 0xA0;
pub const MIDI_CC: u8 = 0xB0;
pub const MIDI_CHANNEL_AFTERTOUCH: u8 = 0xD0;
pub const MIDI_PROGRAM_CHANGE: u8 = 0xC0;
pub const MIDI_PITCH_WHEEL: u8 = 0xE0;
pub const PITCH_WHEEL_MID: u16 = 0x2000;

pub fn u7_to_f32(v: u8) -> f32 {
    ((v & 0x7F) as f32) / (0x7F as f32)
}

pub fn pitchwheel(msb: u8, lsb: u8) -> f32 {
    let x: u16 = (((msb & 0x7F) as u16) << 7) | ((lsb & 0x7F) as u16);
    ((x as f32) - (PITCH_WHEEL_MID as f32)) / (PITCH_WHEEL_MID as f32)
}
