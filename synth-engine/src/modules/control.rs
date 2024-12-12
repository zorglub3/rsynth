use crate::event::ControllerEvent;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use core::f32::consts::PI;

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

    fn compute_value(&self) -> f32 {
        (self.max_value - self.min_value) * self.value + self.min_value
    }
}

impl Module for ContinuousControl {
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
}
