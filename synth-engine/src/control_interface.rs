use crate::midi::*;
use atomic_float::AtomicF32;
use core::mem::MaybeUninit;
use core::sync::atomic::AtomicU8;
use core::sync::atomic::Ordering;
use alloc::sync::Arc;

pub const CONTROLLER_COUNT: usize = 256;
pub const DEFAULT_ORDERING: Ordering = Ordering::Relaxed;

#[derive(Clone, Debug)]
pub struct ControlInterface {
    continuous_controls: [Arc<AtomicF32>; CONTROLLER_COUNT],
    key_pitch: Arc<AtomicF32>,
    gate: Arc<AtomicF32>,
    velocity: Arc<AtomicF32>,
    aftertouch: Arc<AtomicF32>,
    pitch_bend: Arc<AtomicF32>,
    pitch_code: Arc<AtomicU8>,
}

impl ControlInterface {
    pub fn new() -> Self {
        let continuous_controls = {
            let mut array: [MaybeUninit<Arc<AtomicF32>>; CONTROLLER_COUNT] =
                unsafe { MaybeUninit::uninit().assume_init() };

            for i in 0..CONTROLLER_COUNT {
                array[i] = MaybeUninit::new(Arc::new(AtomicF32::new(0.)));
            }

            unsafe { core::mem::transmute::<_, [Arc<AtomicF32>; CONTROLLER_COUNT]>(array) }
        };

        Self {
            continuous_controls,
            key_pitch: Arc::new(AtomicF32::new(1.0)),
            gate: Arc::new(AtomicF32::new(0.0)),
            pitch_bend: Arc::new(AtomicF32::new(0.0)),
            pitch_code: Arc::new(AtomicU8::new(0)),
            velocity: Arc::new(AtomicF32::new(0.0)),
            aftertouch: Arc::new(AtomicF32::new(0.0)),
        }
    }

    pub fn get_continuous_control(&self, controller: usize) -> f32 {
        if controller >= CONTROLLER_COUNT {
            0.
        } else {
            self.continuous_controls[controller].load(DEFAULT_ORDERING)
        }
    }

    pub fn get_gate(&self) -> f32 {
        self.gate.load(DEFAULT_ORDERING)
    }

    pub fn get_pitch(&self) -> f32 {
        self.key_pitch.load(DEFAULT_ORDERING)
    }

    pub fn get_pitchwheel(&self) -> f32 {
        self.pitch_bend.load(DEFAULT_ORDERING)
    }

    pub fn get_aftertouch(&self) -> f32 {
        self.aftertouch.load(DEFAULT_ORDERING)
    }

    pub fn get_velocity(&self) -> f32 {
        self.velocity.load(DEFAULT_ORDERING)
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
                        self.gate.store(1., DEFAULT_ORDERING);
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
                        let value = u7_to_f32(*v);
                        self.continuous_controls[usize::from(*c)]
                            .store(value, DEFAULT_ORDERING);
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
