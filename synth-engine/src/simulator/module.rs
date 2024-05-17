use crate::midi::message::MidiMessage;
use crate::simulator::state::{State, StateUpdate};

pub trait Module: Send {
    fn simulate(&self, state: &State, update: &mut StateUpdate);

    fn process_event(&mut self, event: &MidiMessage, channel: u8);

    fn finalize(&mut self, state: &mut State);
}
