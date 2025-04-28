use crate::control_interface::ControlInterface;
use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use crate::stack_program::*;
use crate::synth_math::SynthMath;

pub struct Folder {
    signal_input: StackProgram,
    control_input: StackProgram,
    output_index: usize,
}

impl Folder {
    pub fn new(
        signal_input: StackProgram,
        control_input: StackProgram,
        output_index: usize,
    ) -> Self {
        Self {
            signal_input,
            control_input,
            output_index,
        }
    }
}

impl Module for Folder {
    fn simulate(
        &self,
        control_interface: &ControlInterface,
        inputs: &[f32],
        state: &mut [f32],
        dt: f32,
    ) {
        todo!()
    }

    fn finalize(&mut self, state: &mut [f32], outputs: &mut [f32], dt: f32) {
        todo!()
    }

    fn get_input_size(&self) -> usize {
        todo!()
    }

    fn get_state_size(&self) -> usize {
        todo!()
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        todo!()
    }

    /*
    fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]) {
        let i = self.signal_input.run(state, stack).unwrap_or(0.);
        let c = self
            .control_input
            .run(state, stack)
            .unwrap_or(0.)
            .max(0.)
            .min(5.)
            + 1.;

        update.set(self.output_index, (i * c).sin(), UpdateType::Absolute);
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, _state: &mut State, _time_step: f32, _stack: &mut [f32]) {
        /* do nothing */
    }
    */
}
