use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UpdateType {
    Differentiable,
    Absolute,
}

#[derive(Debug, Clone)]
pub struct State {
    values: Vec<f32>,
    outputs: Vec<f32>,
}

pub struct StateUpdate {
    updates: Vec<f32>,
    update_types: Vec<UpdateType>,
    delta_time: f32,
    time_step: f32,
}

impl State {
    pub fn new(size: usize) -> Self {
        Self {
            values: vec![0.0_f32; size],
            outputs: vec![0.0_f32; 2],
        }
    }

    pub fn new_with_values(values: &[f32]) -> Self {
        Self {
            values: values.to_vec(),
            outputs: vec![0.; 2],
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn update_data(&self, delta_time: f32, time_step: f32) -> StateUpdate {
        StateUpdate {
            updates: vec![0.0_f32; self.len()],
            update_types: vec![UpdateType::Differentiable; self.len()],
            delta_time,
            time_step,
        }
    }

    pub fn get(&self, index: usize) -> f32 {
        debug_assert!(index < self.values.len());

        self.values[index]
    }

    pub fn set(&mut self, index: usize, v: f32) {
        debug_assert!(index < self.values.len());

        self.values[index] = v;
    }

    pub fn apply_updates(&mut self, updates: &[StateUpdate], weights: &[f32], c: &[f32], dt: f32) {
        debug_assert!(updates.len() == weights.len());
        debug_assert!(updates.len() <= c.len());

        for i in 0..self.len() {
            let mut update = 0.0_f32;
            let mut previous_value = self.values[i];

            for j in 0..updates.len() {
                match updates[j].update_types[i] {
                    UpdateType::Absolute => {
                        if j == 0 {
                            previous_value = updates[j].updates[i];
                        } else {
                            update += (updates[j].updates[i] - previous_value) * weights[j] / c[j];
                        }
                    }
                    UpdateType::Differentiable => {
                        update += updates[j].updates[i] * weights[j] * dt;
                    }
                }
            }

            self.values[i] = previous_value + update;
        }
    }

    pub fn set_output(&mut self, index: usize, v: f32) {
        self.outputs[index] = v;
    }

    pub fn get_output(&self, index: usize) -> f32 {
        self.outputs[index]
    }
}

impl StateUpdate {
    pub fn set(&mut self, index: usize, update: f32, update_type: UpdateType) {
        debug_assert!(index < self.updates.len());

        self.updates[index] = update;
        self.update_types[index] = update_type;
    }

    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn get_time_step(&self) -> f32 {
        self.time_step
    }
}
