pub mod allpass;
pub mod amplifier;
pub mod bowed;
pub mod control;
pub mod delay_line;
pub mod envelope;
pub mod filter_12db;
pub mod filter_24db;
pub mod filter_6db;
pub mod folder;
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
pub use folder::Folder;
pub use mono_keys::MonoKeys;
pub use mono_output::MonoOutput;
pub use noise::NoiseGenerator;
pub use quadrature::QuadratureOscillator;
pub use vosim::Vosim;
pub use wavetable::Wavetable;

use crate::control_interface::ControlInterface;
use crate::simulator::module::Module;
use crate::synth_math::SynthMath;

pub fn control_to_frequency(f0: f32, exp_fc: f32, lin_fc: f32) -> f32 {
    f0 * exp_fc.exp2() + lin_fc
}

pub enum SynthModule {
    Amp(Amplifier),
    Contour(Envelope),
    Filter1Pole(Filter6db),
    Filter2Pole(Filter12db),
    Filter4Pole(Filter24db),
    ContinuousControl(ContinuousControl),
    MonoKeys(MonoKeys),
    Output(MonoOutput),
    Noise(NoiseGenerator),
    QuadOscillator(QuadratureOscillator),
    WavetableOscillator(Wavetable),
    VosimOscillator(Vosim),
    Delay(DelayLine),
    Wavefolder(Folder),
    Bowed(BowedOscillator),
}

use crate::event::ControllerEvent;
use crate::simulator::state::{State, StateUpdate, UpdateType};

impl SynthModule {
    pub fn simulate(
        &self,
        control_interface: &ControlInterface,
        inputs: &[f32],
        state: &[f32],
        update: &mut [f32],
        dt: f32,
    ) {
        todo!();
    }

    pub fn finalize(&mut self, _inputs: &[f32], state: &mut [f32], outputs: &mut [f32], dt: f32) {
        todo!();
    }

    pub fn get_input_size(&self) -> usize {
        todo!();
    }

    pub fn get_state_size(&self) -> usize {
        todo!();
    }

    pub fn set_update_type(&self, update_types: &mut [UpdateType]) {
        todo!();
    }

    /*
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
            Wavefolder(f) => f.simulate(state, update, stack),
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

        // TODO add filters here - make sure they don't run wild
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
    */
}
