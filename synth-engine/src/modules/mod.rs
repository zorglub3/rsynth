pub mod allpass;
pub mod amplifier;
pub mod bowed;
pub mod control;
pub mod delay_line;
pub mod envelope;
pub mod filter_12db;
pub mod filter_24db;
pub mod filter_6db;
pub mod mono_keys;
pub mod mono_output;
pub mod noise;
pub mod quadrature;
pub mod vosim;
pub mod wavetable;

pub use allpass::AllpassFilter;
pub use amplifier::Amplifier;
pub use bowed::BowedOscillator;
pub use control::ContinuousControl;
pub use delay_line::DelayLine;
pub use envelope::Envelope;
pub use filter_12db::Filter12db;
pub use filter_24db::Filter24db;
pub use filter_6db::Filter6db;
pub use mono_keys::MonoKeys;
pub use mono_output::MonoOutput;
pub use noise::NoiseGenerator;
pub use quadrature::QuadratureOscillator;
pub use vosim::VosimOscillator;
pub use wavetable::WavetableOscillator;

use crate::simulator::module::Module;
use libm::exp2f;

pub fn control_to_frequency(f0: f32, exp_fc: f32, lin_fc: f32) -> f32 {
    f0 * exp2f(exp_fc) + lin_fc
}

pub enum SynthModule<'a> {
    Amp(Amplifier<'a>),
    Contour(Envelope<'a>),
    Filter1Pole(Filter6db<'a>),
    Filter2Pole(Filter12db<'a>),
    Filter4Pole(Filter24db<'a>),
    ContinuousControl(ContinuousControl),
    MonoKeys(MonoKeys),
    Output(MonoOutput<'a>),
    Noise(NoiseGenerator),
    QuadOscillator(QuadratureOscillator<'a>),
    WavetableOscillator(WavetableOscillator<'a, 'a>),
    VosimOscillator(VosimOscillator<'a, 'a>),
    Delay(DelayLine<'a, 'a>),
    Bowed(BowedOscillator<'a>),
}

use crate::event::ControllerEvent;
use crate::simulator::state::{State, StateUpdate};

impl<'a> SynthModule<'a> {
    pub fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]) {
        use SynthModule::*;
        match self {
            Amp(a) => a.simulate(state, update, stack),
            Contour(c) => c.simulate(state, update, stack),
            Filter1Pole(f) => f.simulate(state, update, stack),
            Filter2Pole(f) => f.simulate(state, update, stack),
            Filter4Pole(f) => f.simulate(state, update, stack),
            ContinuousControl(c) => c.simulate(state, update, stack),
            MonoKeys(mk) => mk.simulate(state, update, stack),
            Noise(n) => n.simulate(state, update, stack),
            QuadOscillator(q) => q.simulate(state, update, stack),
            WavetableOscillator(w) => w.simulate(state, update, stack),
            VosimOscillator(v) => v.simulate(state, update, stack),
            Delay(d) => d.simulate(state, update, stack),
            Bowed(b) => b.simulate(state, update, stack),
            _ => { /* do nothing */ }
        }
    }

    pub fn process_event(&mut self, event: &ControllerEvent) {
        use SynthModule::*;
        match self {
            ContinuousControl(c) => c.process_event(event),
            MonoKeys(m) => m.process_event(event),
            _ => { /* do nothing */ }
        }
    }

    pub fn finalize(&mut self, state: &mut State, time_step: f32, stack: &mut [f32]) {
        use SynthModule::*;

        match self {
            Noise(n) => n.finalize(state, time_step, stack),
            Output(m) => m.finalize(state, time_step, stack),
            Delay(d) => d.finalize(state, time_step, stack),
            Contour(c) => c.finalize(state, time_step, stack),
            QuadOscillator(q) => q.finalize(state, time_step, stack),
            WavetableOscillator(w) => w.finalize(state, time_step, stack),
            _ => {}
        }
    }
}
