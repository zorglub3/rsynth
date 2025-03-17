use crate::midi::*;

pub const CONTROLLER_COUNT: usize = 256;

pub struct ControlInterface {
    continuous_controls: [f32; CONTROLLER_COUNT],
    key_pitch: f32,
    gate: f32,
    velocity: f32,
    aftertouch: f32,
    pitch_bend: f32,
    pitch_code: Option<u8>,
}

impl ControlInterface {
    pub fn new() -> Self {
        Self {
            continuous_controls: [0.; CONTROLLER_COUNT],
            key_pitch: 1.,
            gate: 0.,
            pitch_bend: 0.,
            pitch_code: None,
            velocity: 0.,
            aftertouch: 0.,
        }
    }

    pub fn midi_input(&mut self, scale: &[f32], bytes: &[u8], channel: Option<u8>) {
        if bytes.len() > 1 {
            let chn = bytes[0] & 0xF;

            if channel.is_none() || Some(chn) == channel {
                match (bytes[0] & 0xF0, bytes) {
                    (MIDI_NOTE_OFF, [_, pitch, _]) if Some(*pitch) == self.pitch_code => {
                        self.pitch_code = None;
                        self.gate = 0.;
                    }
                    (MIDI_NOTE_ON, [_, pitch, 0]) if Some(*pitch) == self.pitch_code => {
                        self.pitch_code = None;
                        self.gate = 0.
                    }
                    (MIDI_NOTE_ON, [_, pitch, velocity]) if usize::from(*pitch) < scale.len() => {
                        self.pitch_code = Some(*pitch);
                        self.key_pitch = scale[usize::from(*pitch)];
                        self.velocity = u7_to_f32(*velocity);
                    }
                    (MIDI_POLY_AFTERTOUCH, [_, pitch, v]) if Some(*pitch) == self.pitch_code => {
                        self.aftertouch = u7_to_f32(*v);
                    }
                    (MIDI_CHANNEL_AFTERTOUCH, [_, v, ..]) => {
                        self.aftertouch = u7_to_f32(*v);
                    }
                    (MIDI_CC, [_, c, v]) if usize::from(*c) < CONTROLLER_COUNT => {
                        self.continuous_controls[usize::from(*c)] = u7_to_f32(*v);
                    }
                    (MIDI_PITCH_WHEEL, [_, d1, d2]) => {
                        self.pitch_bend = pitchwheel(*d2, *d1);
                    }
                    _ => { /* do nothing */ }
                }
            }
        }
    }
}
