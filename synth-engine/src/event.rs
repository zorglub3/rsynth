//! Types for events that the synth engine can receive for real time control.

use crate::midi::message::MidiMessage;
use crate::modules::Module;
use std::fmt::{Debug, Error, Formatter};

/// The basic type for event that the synth engine "understands"
pub enum Event {
    /// A MIDI event as it is received from a controller or sequencer.
    MidiEvent { event: MidiMessage, channel: u8 },
    /// Set the state of some parameter in the current state of the
    /// simulation. This is both for testing, but also to set fixed
    /// constants as if required for some modules.
    SetState { index: usize, value: f32 },
    /// Add a new module to the synth engine. The engine should eventually
    /// support reconfiguration of the modular setup while it is running.
    AddModule {
        /// Identifier for the new module
        id: String,
        /// The actual module as it is used in the synth engine simulation.
        module: Box<dyn Module>,
    },
    /// Remove a module from the synth engine while it is playing. This
    /// should also remove any connections to/from the deleted module.
    /// If the module has already been removed or if it doesn't exist at all,
    /// then this event has no effect.
    DeleteModule {
        /// The identifier for the module to remove.
        id: String,
    },
    /// Connect one output from a module to the input of a module. There
    /// are no other restrictions. A module can even take its own output as
    /// input.
    ///
    /// One output can be connected to several inputs.
    ConnectModules {
        /// Identifier for the source/output module
        source_id: String,
        /// Index of the specific output of the source module
        source_index: usize,
        /// Identifier for the target/input module.
        target_id: String,
        /// Index of the input parameter to connect to.
        target_index: usize,
    },
    /// Disconnect one input to a module.
    DisconnectModules {
        /// Identifier for the module
        target_id: String,
        /// Identifier of the parameter of the target module to disconnect.
        target_index: usize,
    },
    /// Set some constant for some module. TODO remove this event - all
    /// such constants should be part of the state vector and should be set with
    /// the [SetState] event.
    SetConstant {
        module_id: String,
        module_index: String,
        value: f32,
    },
}

impl Debug for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Event::MidiEvent { event, channel } => {
                f.write_str(&format!("MidiEvent({event:?}, {channel})"))
            }
            _ => f.write_str("blah"),
        }
    }
}
