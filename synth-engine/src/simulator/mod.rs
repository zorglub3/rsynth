//! The workhorse module for running the synth engine.
//!
//! TODO clean up the interface. The `process_event` function should
//! handle all adding and deleting of modules and connecting.

use crate::event::Event;
use crate::modules::Module;

pub mod module;
pub mod rk4;
pub mod rungekutta;
pub mod state;

/// Interface for a simulator that can simulate a synth module configuration.
pub trait Simulator: Send {
    /// Run one time step in the simulation. The length of the timestep will
    /// usually be equal to the inverse of the sampling frequency of the audio
    /// output device (unless oversampling).
    fn step(&mut self, dt: f32);

    /// Get the left- and right- output channels of the synth. Currently, only
    /// stereo is supported. TODO add support for monophonic and quadraphonic
    /// output.
    fn get_output(&self) -> (f32, f32);

    /// Process one control event.
    fn process_event(&mut self, event: Event);

    /// TODO remove
    fn delete_module(&mut self, id: String);

    /// TODO remove
    fn add_module(&mut self, id: String, module: Box<dyn Module>);

    /// TODO remove
    fn connect_module(
        &mut self,
        source_id: String,
        source_index: usize,
        target_id: String,
        target_index: usize,
    );
}
