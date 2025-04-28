use crate::midi::*;
use atomic_float::AtomicF32;
use core::mem::MaybeUninit;
use core::sync::atomic::AtomicU8;
use core::sync::atomic::Ordering;

pub const CONTROLLER_COUNT: usize = 256;
pub const DEFAULT_ORDERING: Ordering = Ordering::Relaxed;

pub struct ControlInterface {
    continuous_controls: [AtomicF32; CONTROLLER_COUNT],
    key_pitch: AtomicF32,
    gate: AtomicF32,
    velocity: AtomicF32,
    aftertouch: AtomicF32,
    pitch_bend: AtomicF32,
    pitch_code: AtomicU8,
}

impl ControlInterface {
    pub fn new() -> Self {
        let continuous_controls = {
            let mut array: [MaybeUninit<AtomicF32>; CONTROLLER_COUNT] =
                unsafe { MaybeUninit::uninit().assume_init() };

            for i in 0..CONTROLLER_COUNT {
                array[i] = MaybeUninit::new(AtomicF32::new(0.));
            }

            unsafe { core::mem::transmute::<_, [AtomicF32; CONTROLLER_COUNT]>(array) }
        };

        Self {
            continuous_controls,
            key_pitch: 1.0.into(),
            gate: 0.0.into(),
            pitch_bend: 0.0.into(),
            pitch_code: 0_u8.into(),
            velocity: 0.0.into(),
            aftertouch: 0.0.into(),
        }
    }

    pub fn midi_input(&self, scale: &[f32], bytes: &[u8], channel: Option<u8>) {
        if bytes.len() > 1 {
            let chn = bytes[0] & 0xF;

            if channel.is_none() || Some(chn) == channel {
                let pitch_code = self.pitch_code.load(DEFAULT_ORDERING);

                match (bytes[0] & 0xF0, bytes) {
                    (MIDI_NOTE_OFF, [_, pitch, _]) if *pitch == pitch_code => {
                        self.pitch_code.store(0, DEFAULT_ORDERING);
                        self.gate.store(0., DEFAULT_ORDERING);
                    }
                    (MIDI_NOTE_ON, [_, pitch, 0]) if *pitch == pitch_code => {
                        self.pitch_code.store(0, DEFAULT_ORDERING);
                        self.gate.store(0., DEFAULT_ORDERING);
                    }
                    (MIDI_NOTE_ON, [_, pitch, velocity]) if usize::from(*pitch) < scale.len() => {
                        self.pitch_code.store(*pitch, DEFAULT_ORDERING);
                        self.key_pitch
                            .store(scale[usize::from(*pitch)], DEFAULT_ORDERING);
                        self.velocity.store(u7_to_f32(*velocity), DEFAULT_ORDERING);
                    }
                    (MIDI_POLY_AFTERTOUCH, [_, pitch, v]) if *pitch == pitch_code => {
                        self.aftertouch.store(u7_to_f32(*v), DEFAULT_ORDERING);
                    }
                    (MIDI_CHANNEL_AFTERTOUCH, [_, v, ..]) => {
                        self.aftertouch.store(u7_to_f32(*v), DEFAULT_ORDERING);
                    }
                    (MIDI_CC, [_, c, v]) if usize::from(*c) < CONTROLLER_COUNT => {
                        self.continuous_controls[usize::from(*c)]
                            .store(u7_to_f32(*v), DEFAULT_ORDERING);
                    }
                    (MIDI_PITCH_WHEEL, [_, d1, d2]) => {
                        self.pitch_bend
                            .store(pitchwheel(*d2, *d1), DEFAULT_ORDERING);
                    }
                    _ => { /* do nothing */ }
                }
            }
        }
    }
}
