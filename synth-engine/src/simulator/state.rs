use crate::stack_program::StackProgram;
use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq)]
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
    // update_types: Vec<UpdateType>,
    delta_time: f32,
    time_step: f32,
}

impl State {
    pub fn new(values_size: usize) -> Self {
        Self {
            values: vec![0.0_f32; values_size],
            // outputs: vec![0.0_f32; 2],
        }
    }

    pub fn borrow_values(&self) -> &[f32] {
        &self.values
    }

    pub fn values_mut(&mut self, lo: usize, hi: usize) -> &mut [f32] {
        &mut self.values[lo..hi]
    }

    /*
    pub fn new_with_values(values: &[f32]) -> Self {
        Self {
            values: values.to_vec(),
            outputs: vec![0.; 2],

        }
    }
    */

    /*
    pub fn len(&self) -> usize {
        self.values.len()
    }
    */

    pub fn update_data(&self, delta_time: f32, time_step: f32) -> StateUpdate {
        let len = self.values.len();

        StateUpdate {
            updates: vec![0.0_f32; len],
            // update_types: vec![UpdateType::Differentiable; len],
            delta_time,
            time_step,
        }
    }

    /*
    pub fn get(&self, index: usize) -> f32 {
        self.values[index]
    }

    pub fn set(&mut self, index: usize, v: f32) {
        self.values[index] = v;
    }
    */

    pub fn apply_updates(
        &mut self,
        updates: &[StateUpdate],
        update_types: &[UpdateType],
        weights: &[f32],
        c: &[f32],
        dt: f32,
        update_count: usize,
    ) {
        for i in 0..self.values.len() {
            let mut update = 0.0_f32;
            let mut previous_value = self.values[i];

            for j in 0..update_count.min(updates.len()) {
                match update_types[i] {
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

    pub fn copy_from(&mut self, other: &State) {
        self.values.copy_from_slice(&other.values);
        // self.outputs.copy_from_slice(&other.outputs);
    }

    /*
    pub fn set_output(&mut self, index: usize, v: f32) {
        self.outputs[index] = v;
    }

    pub fn get_output(&self, index: usize) -> f32 {
        self.outputs[index]
    }
    */
}

impl StateUpdate {
    pub fn new(size: usize) -> Self {
        Self {
            updates: vec![0.; size],
            // update_types: vec![UpdateType::Differentiable; size],
            delta_time: 0.,
            time_step: 0.,
        }
    }

    pub fn init(&mut self, delta_time: f32, time_step: f32) {
        self.delta_time = delta_time;
        self.time_step = time_step;
        self.updates.fill(0.);
        // self.update_types.fill(UpdateType::Differentiable);
    }

    /*
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
    */

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
