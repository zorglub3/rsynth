use crate::control_interface::ControlInterface;
use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use core::f32::consts::PI;

pub const STATE_SIZE: usize = 1;
pub const INPUT_SIZE: usize = 0;

// state/outputs
const CONTROL_OUTPUT: usize = 0;

// no inputs/control (the control is external to the simulation)

pub struct ContinuousControl {
    output_index: usize,
    control: usize,
    value: f32,
    min_value: f32,
    max_value: f32,
    filter_freq: Option<f32>,
}

impl ContinuousControl {
    pub fn new(output_index: usize, control: usize, min_value: f32, max_value: f32) -> Self {
        Self {
            output_index,
            control,
            value: 0.,
            min_value,
            max_value,
            filter_freq: Some(50.),
        }
    }

    fn compute_value(&self, controller_value: f32) -> f32 {
        (self.max_value - self.min_value) * controller_value + self.min_value
    }
}

impl Module for ContinuousControl {
    fn simulate(
        &self,
        control_interface: &ControlInterface,
        _inputs: &[f32],
        state: &[f32],
        update: &mut [f32],
        _dt: f32,
    ) {
        if let Some(freq) = self.filter_freq {
            let k = 2. * PI * freq;
            let v = 
                self.compute_value(control_interface.get_continuous_control(self.control));
            let d = v - state[CONTROL_OUTPUT];

            update[CONTROL_OUTPUT] = k * d;
        } else {
            update[CONTROL_OUTPUT] = 
                self.compute_value(control_interface.get_continuous_control(self.control));
        }
    }

    fn finalize(&mut self, _inputs: &[f32], _state: &mut [f32], _outputs: &mut [f32], _dt: f32) {
        /* do nothing */
    }

    fn get_input_size(&self) -> usize {
        INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        if self.filter_freq.is_some() {
            update_types[CONTROL_OUTPUT] = UpdateType::Differentiable;
        } else {
            update_types[CONTROL_OUTPUT] = UpdateType::Absolute;
        }
    }

    /*
    fn simulate(&self, state: &State, update: &mut StateUpdate, _stack: &mut [f32]) {
        if let Some(freq) = self.filter_freq {
            let k = 2. * PI * freq;
            let v = self.compute_value();
            let d = v - state.get(self.output_index);

            update.set(self.output_index, k * d, UpdateType::Differentiable);
        } else {
            update.set(
                self.output_index,
                self.compute_value(),
                UpdateType::Absolute,
            );
        }
    }

    fn process_event(&mut self, event: &ControllerEvent) {
        match event {
            ControllerEvent::ContinuousControl { control, value } if *control == self.control => {
                self.value = *value;
            }
            _ => { /* do nothing */ }
        }
    }

    fn finalize(&mut self, _state: &mut State, _time_step: f32, _stack: &mut [f32]) {
        /* do nothing */
    }
    */
}
