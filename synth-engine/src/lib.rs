//! This library provides a simulator for a virtual analog synthesizer. It also
//! gives some modules to construct arbitrary synthesizer configurations. Lastly,
//! it provides a set of messages that can be sent to the simulation for interaction.
//!
//! This library does _not_ provide an interface with audio or MIDI. This should
//! be provided in the program using the synth simulator. See the `cli-synth` example
//! program for suggestions.

pub mod event;
pub mod interpolation;
pub mod modules;
pub mod simulator;
pub mod sinc_filter;
pub mod stack_program;
