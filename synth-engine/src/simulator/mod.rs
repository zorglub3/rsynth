//! The workhorse module for running the synth engine.
//!

pub mod module;
pub mod rungekutta;
pub mod state;

use crate::control_interface::ControlInterface;
use crate::modules::SynthModule;
use crate::simulator::state::StateInput;
use alloc::vec::Vec;
use core::ops::Range;

pub struct ModuleEntry {
    pub synth_module: SynthModule,
    pub input: Range<usize>,
    pub state: Range<usize>,
}

pub trait Simulator: Send {
    fn set_model(&mut self, modules: Vec<ModuleEntry>, state_input: StateInput);
    fn step(&mut self, dt: f32, control_interface: &ControlInterface);
    fn get_stereo_output(&self) -> (f32, f32);
}
