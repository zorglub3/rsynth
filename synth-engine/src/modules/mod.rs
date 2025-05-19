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

use crate::simulator::state::UpdateType;

impl SynthModule {
    pub fn simulate(
        &self,
        control_interface: &ControlInterface,
        inputs: &[f32],
        state: &[f32],
        update: &mut [f32],
        dt: f32,
    ) {
        use SynthModule::*;

        match self {
            Amp(a) => a.simulate(control_interface, inputs, state, update, dt),
            Contour(c) => c.simulate(control_interface, inputs, state, update, dt),
            Filter1Pole(f) => f.simulate(control_interface, inputs, state, update, dt),
            Filter2Pole(f) => f.simulate(control_interface, inputs, state, update, dt),
            Filter4Pole(f) => f.simulate(control_interface, inputs, state, update, dt),
            ContinuousControl(c) => c.simulate(control_interface, inputs, state, update, dt),
            MonoKeys(m) => m.simulate(control_interface, inputs, state, update, dt),
            Noise(n) => n.simulate(control_interface, inputs, state, update, dt),
            QuadOscillator(q) => q.simulate(control_interface, inputs, state, update, dt),
            WavetableOscillator(w) => w.simulate(control_interface, inputs, state, update, dt),
            VosimOscillator(v) => v.simulate(control_interface, inputs, state, update, dt),
            Delay(d) => d.simulate(control_interface, inputs, state, update, dt),
            Wavefolder(w) => w.simulate(control_interface, inputs, state, update, dt),
            Bowed(b) => b.simulate(control_interface, inputs, state, update, dt),
            Output(o) => o.simulate(control_interface, inputs, state, update, dt),
        }
    }

    pub fn finalize(&mut self, inputs: &[f32], state: &mut [f32], outputs: &mut [f32], dt: f32) {
        use SynthModule::*;

        match self {
            Amp(a) => a.finalize(inputs, state, outputs, dt),
            Contour(c) => c.finalize(inputs, state, outputs, dt),
            Filter1Pole(f) => f.finalize(inputs, state, outputs, dt),
            Filter2Pole(f) => f.finalize(inputs, state, outputs, dt),
            Filter4Pole(f) => f.finalize(inputs, state, outputs, dt),
            ContinuousControl(c) => c.finalize(inputs, state, outputs, dt),
            MonoKeys(m) => m.finalize(inputs, state, outputs, dt),
            Noise(n) => n.finalize(inputs, state, outputs, dt),
            QuadOscillator(q) => q.finalize(inputs, state, outputs, dt),
            WavetableOscillator(w) => w.finalize(inputs, state, outputs, dt),
            VosimOscillator(v) => v.finalize(inputs, state, outputs, dt),
            Delay(d) => d.finalize(inputs, state, outputs, dt),
            Wavefolder(w) => w.finalize(inputs, state, outputs, dt),
            Bowed(b) => b.finalize(inputs, state, outputs, dt),
            Output(o) => o.finalize(inputs, state, outputs, dt),
        }
    }

    pub fn get_input_size(&self) -> usize {
        use SynthModule::*;

        match self {
            Amp(a) => a.get_input_size(),
            Contour(c) => c.get_input_size(),
            Filter1Pole(f) => f.get_input_size(),
            Filter2Pole(f) => f.get_input_size(),
            Filter4Pole(f) => f.get_input_size(),
            ContinuousControl(c) => c.get_input_size(),
            MonoKeys(m) => m.get_input_size(),
            Noise(n) => n.get_input_size(),
            QuadOscillator(q) => q.get_input_size(),
            WavetableOscillator(w) => w.get_input_size(),
            VosimOscillator(v) => v.get_input_size(),
            Delay(d) => d.get_input_size(),
            Wavefolder(w) => w.get_input_size(),
            Bowed(b) => b.get_input_size(),
            Output(o) => o.get_input_size(),
        }
    }

    pub fn get_state_size(&self) -> usize {
        use SynthModule::*;

        match self {
            Amp(a) => a.get_state_size(),
            Contour(c) => c.get_state_size(),
            Filter1Pole(f) => f.get_state_size(),
            Filter2Pole(f) => f.get_state_size(),
            Filter4Pole(f) => f.get_state_size(),
            ContinuousControl(c) => c.get_state_size(),
            MonoKeys(m) => m.get_state_size(),
            Noise(n) => n.get_state_size(),
            QuadOscillator(q) => q.get_state_size(),
            WavetableOscillator(w) => w.get_state_size(),
            VosimOscillator(v) => v.get_state_size(),
            Delay(d) => d.get_state_size(),
            Wavefolder(w) => w.get_state_size(),
            Bowed(b) => b.get_state_size(),
            Output(o) => o.get_state_size(),
        }
    }

    pub fn set_update_type(&self, update_types: &mut [UpdateType]) {
        use SynthModule::*;

        match self {
            Amp(a) => a.set_update_type(update_types),
            Contour(c) => c.set_update_type(update_types),
            Filter1Pole(f) => f.set_update_type(update_types),
            Filter2Pole(f) => f.set_update_type(update_types),
            Filter4Pole(f) => f.set_update_type(update_types),
            ContinuousControl(c) => c.set_update_type(update_types),
            MonoKeys(m) => m.set_update_type(update_types),
            Noise(n) => n.set_update_type(update_types),
            QuadOscillator(q) => q.set_update_type(update_types),
            WavetableOscillator(w) => w.set_update_type(update_types),
            VosimOscillator(v) => v.set_update_type(update_types),
            Delay(d) => d.set_update_type(update_types),
            Wavefolder(w) => w.set_update_type(update_types),
            Bowed(b) => b.set_update_type(update_types),
            Output(o) => o.set_update_type(update_types),
        }
    }
}
