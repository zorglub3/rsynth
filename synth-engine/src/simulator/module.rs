// use crate::event::ControllerEvent;
use crate::control_interface::ControlInterface;
use crate::simulator::state::{State, StateUpdate, UpdateType};

pub trait Module: Send {
    fn simulate(
        &self,
        control_interface: &ControlInterface,
        inputs: &[f32],
        state: &mut [f32],
        dt: f32,
    );
    fn finalize(&mut self, state: &mut [f32], outputs: &mut [f32], dt: f32);
    fn get_input_size(&self) -> usize;
    fn get_state_size(&self) -> usize;
    fn set_update_type(&self, update_types: &mut [UpdateType]);

    /*
    fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]);

    fn process_event(&mut self, event: &ControllerEvent);

    fn finalize(&mut self, state: &mut State, time_step: f32, stack: &mut [f32]);
    */
}
