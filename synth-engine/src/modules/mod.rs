pub mod amplifier;
pub mod ar_envelope;
pub mod filter12db;
pub mod midi_cc;
pub mod midi_mono;
pub mod modulator;
pub mod osc;

use crate::midi::message::MidiMessage;
use crate::state::OutputType;

pub trait Module: Send {
    fn simulate(&self, dt: f32, state: &Vec<f32>, out: &mut Vec<f32>);
    fn finalize(&self, state: &mut Vec<f32>);
    fn inputs(&self) -> Vec<usize>;
    fn outputs(&self) -> Vec<usize>;
    fn output_types(&self) -> Vec<OutputType>;

    fn process_event(&mut self, event: &MidiMessage, channel: u8);

    fn max_output_index(&self) -> usize {
        *self.outputs().iter().max().unwrap_or(&0)
    }
}
