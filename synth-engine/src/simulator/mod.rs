//! The workhorse module for running the synth engine.
//!

pub mod module;
pub mod rungekutta;
pub mod state;

use crate::event::ControllerEvent;
use crate::simulator::state::{State, StateUpdate};

pub trait Simulator {
    fn step<'a>(&mut self, dt: f32, updates: &mut [StateUpdate<'a>], temp_states: &mut [State<'a>]);
    fn process_event(&mut self, event: ControllerEvent);
    fn get_stereo_output(&self) -> (f32, f32);
}
