// #[cfg(any(feature = "allocator", test))]
// use alloc::vec;

const OUTPUTS: usize = 2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UpdateType {
    Differentiable,
    Absolute,
}

#[derive(Debug)]
pub struct State<'a> {
    values: &'a mut [f32],
    outputs: [f32; OUTPUTS],
}

pub struct StateUpdate<'a> {
    updates: &'a mut [f32],
    update_types: &'a mut [UpdateType],
    delta_time: f32,
    time_step: f32,
}

impl<'a> State<'a> {
    pub fn new(values: &'a mut [f32], outputs: [f32; OUTPUTS]) -> Self {
        Self { values, outputs }
    }

    pub fn new_with_values(values: &'a mut [f32]) -> Self {
        Self {
            values,
            outputs: [0.; OUTPUTS],
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    /*
    pub fn update_data(&self, delta_time: f32, time_step: f32) -> StateUpdate {
        StateUpdate {
            updates: vec![0.0_f32; self.len()],
            update_types: vec![UpdateType::Differentiable; self.len()],
            delta_time,
            time_step,
        }
    }
    */

    // TODO check if we really need delta_time _and time_step
    pub fn clear_update_data(
        &self,
        update_data: &mut StateUpdate,
        delta_time: f32,
        time_step: f32,
    ) {
        for i in 0..update_data.updates.len() {
            update_data.updates[i] = 0.;
            update_data.update_types[i] = UpdateType::Differentiable;
        }

        update_data.delta_time = delta_time;
        update_data.time_step = time_step;
    }

    pub fn copy_values_to(&self, target: &mut Self) {
        for i in 0..self.values.len() {
            target.values[i] = self.values[i];
        }
    }

    pub fn get(&self, index: usize) -> f32 {
        self.values[index]
    }

    pub fn set(&mut self, index: usize, v: f32) {
        self.values[index] = v;
    }

    pub fn apply_updates(
        &mut self,
        updates: &[StateUpdate],
        weights: &[f32],
        c: &[f32],
        dt: f32,
        max_index: usize,
    ) {
        debug_assert!(updates.len() == weights.len());
        debug_assert!(updates.len() <= c.len());

        for i in 0..self.len() {
            let mut update = 0.0_f32;
            let mut previous_value = self.values[i];

            for j in 0..max_index {
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

impl<'a> StateUpdate<'a> {
    pub fn set(&mut self, index: usize, update: f32, update_type: UpdateType) {
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
