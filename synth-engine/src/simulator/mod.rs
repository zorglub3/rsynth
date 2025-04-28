//! The workhorse module for running the synth engine.
//!

pub mod module;
pub mod rungekutta;
pub mod state;

use crate::control_interface::ControlInterface;
use crate::event::ControllerEvent;
use crate::modules::SynthModule;
use alloc::vec::Vec;

pub trait Simulator: Send {
    fn set_modules(&mut self, modules: Vec<SynthModule>);
    fn step(&mut self, dt: f32, control_interface: &ControlInterface);
    fn get_stereo_output(&self) -> (f32, f32);
    // fn process_event(&mut self, event: ControllerEvent);
}
