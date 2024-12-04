use crate::event::ControllerEvent;
use crate::simulator::state::{State, StateUpdate};

pub trait Module: Send {
    fn simulate(&self, state: &State, update: &mut StateUpdate, stack: &mut [f32]);

    fn process_event(&mut self, event: &ControllerEvent);

    fn finalize(&mut self, state: &mut State, time_step: f32, stack: &mut [f32]);
}
