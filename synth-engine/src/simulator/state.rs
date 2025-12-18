use crate::stack_program::StackProgram;
use crate::distortion::{DistortionType, Distort};
use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone)]
pub enum UpdateType {
    Differentiable,
    Absolute,
}

impl Default for UpdateType {
    fn default() -> Self {
        Self::Differentiable
    }
}

#[derive(Debug, Clone)]
pub struct State {
    values: Vec<f32>,
}

pub struct StateUpdate {
    updates: Vec<f32>,
    delta_time: f32,
    time_step: f32,
}

impl State {
    pub fn new(values_size: usize) -> Self {
        Self {
            values: vec![0.0_f32; values_size],
        }
    }

    pub fn borrow_all_values(&self) -> &[f32] {
        &self.values
    }

    pub fn borrow_values(&self, lo: usize, hi: usize) -> &[f32] {
        &self.values[lo..hi]
    }

    pub fn values_mut(&mut self, lo: usize, hi: usize) -> &mut [f32] {
        &mut self.values[lo..hi]
    }

    pub fn update_data(&self, delta_time: f32, time_step: f32) -> StateUpdate {
        let len = self.values.len();

        StateUpdate {
            updates: vec![0.0_f32; len],
            delta_time,
            time_step,
        }
    }

    pub fn apply_updates(
        &mut self,
        updates: &[StateUpdate],
        update_types: &[UpdateType],
        weights: &[f32],
        c: &[f32],
        dt: f32,
        update_count: usize,
    ) {
        let update_count = update_count.min(updates.len());

        if update_count < 1 {
            return
        };

        for i in 0..self.values.len() {
            match &update_types[i] {
                UpdateType::Absolute => {
                    // TODO consider 
                    // let previous_value = f(update[j].updates[i], self.values[i]);
                    // for some function `f` (eg average)
                    let previous_value = updates[0].updates[i];
                    let mut update = 0.0_f32;
                    for j in 1..update_count {
                        update += (updates[j].updates[i] - previous_value) * weights[j] / c[j];
                    }
                    self.values[i] = previous_value + update;
                }
                UpdateType::Differentiable => {
                    for j in 0..update_count {
                        self.values[i] += updates[j].updates[i] * weights[j] * dt;
                    }
                }
            }
        }
    }

    pub fn copy_from(&mut self, other: &State) {
        self.values.copy_from_slice(&other.values);
    }
}

impl StateUpdate {
    pub fn new(size: usize) -> Self {
        Self {
            updates: vec![0.; size],
            delta_time: 0.,
            time_step: 0.,
        }
    }

    pub fn init(&mut self, delta_time: f32, time_step: f32) {
        self.delta_time = delta_time;
        self.time_step = time_step;
        self.updates.fill(0.);
    }

    pub fn values_mut(&mut self, lo: usize, hi: usize) -> &mut [f32] {
        &mut self.updates[lo..hi]
    }
}

pub struct StateInput {
    inputs: Vec<f32>,
    input_programs: Vec<StackProgram>,
    stack: Vec<f32>,
}

impl StateInput {
    pub fn new(inputs_size: usize, stack_size: usize) -> Self {
        Self {
            inputs: vec![0.0_f32; inputs_size],
            input_programs: vec![StackProgram::zero(); inputs_size],
            stack: vec![0.0_f32; stack_size],
        }
    }

    pub fn set_program(&mut self, index: usize, program: StackProgram) {
        self.input_programs[index] = program;
    }

    pub fn compute_inputs(&mut self, values: &[f32]) {
        for i in 0..self.input_programs.len() {
            self.inputs[i] = self.input_programs[i].eval(values, &mut self.stack);
        }
    }

    pub fn borrow_inputs(&self, lo: usize, hi: usize) -> &[f32] {
        &self.inputs[lo..hi]
    }
}
