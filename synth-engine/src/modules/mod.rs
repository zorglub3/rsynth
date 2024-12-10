pub mod ad_envelope;
pub mod allpass;
pub mod amplifier;
pub mod bowed;
pub mod delay_line;
pub mod filter_12db;
pub mod filter_24db;
pub mod filter_6db;
pub mod folder;
pub mod midi_cc;
pub mod midi_mono;
pub mod modulator;
pub mod mono_output;
pub mod noise;
pub mod quadrature;
pub mod saw_osc;
pub mod wavetable;

pub use ad_envelope::ADEnvelope;
pub use allpass::AllpassFilter;
pub use amplifier::Amplifier;
pub use bowed::BowedOscillator;
pub use delay_line::DelayLine;
pub use filter_12db::Filter12db;
pub use filter_24db::Filter24db;
pub use filter_6db::Filter6db;
pub use folder::Folder;
pub use midi_cc::MidiCC;
pub use midi_mono::MidiMono;
pub use modulator::Modulator;
pub use mono_output::MonoOutput;
pub use noise::NoiseGenerator;
pub use quadrature::QuadratureOscillator;
pub use saw_osc::SawOscillator;
pub use wavetable::Wavetable;

pub fn control_to_frequency(f0: f32, exp_fc: f32, lin_fc: f32) -> f32 {
    f0 * 2.0_f32.powf(exp_fc) + lin_fc
}

////// ALL BELOW IS FOR GETTING RID OF A BOX :-p //////

#[allow(dead_code)]
pub enum SynthModule {
    Amp(Amplifier),
    Contour(ADEnvelope),
    Filter1Pole(Filter6db),
    Filter2Pole(Filter12db),
    Filter4Pole(Filter24db),
    ContinuousControl(MidiCC),
    MonoKeys(MidiMono),
    Output(MonoOutput),
    Noise(NoiseGenerator),
    QuadOscillator(QuadratureOscillator),
    WavetableOscillator(Wavetable),
    Delay(DelayLine),
    Wavefolder(Folder),
    Bowed(BowedOscillator),
}

use crate::event::ControllerEvent;
use crate::simulator::state::{State, StateUpdate};

impl SynthModule {
    #[allow(dead_code)]
    fn simulate(&self, _state: &State, _update: &mut StateUpdate, _stack: &mut [f32]) {
        todo!()
    }

    #[allow(dead_code)]
    fn process_event(&mut self, _event: &ControllerEvent) {
        todo!()
    }

    #[allow(dead_code)]
    fn finalize(&mut self, _state: &mut State, _time_step: f32, _stack: &mut [f32]) {
        todo!()
    }
}
