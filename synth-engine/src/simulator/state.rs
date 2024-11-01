#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UpdateType {
    Differentiable,
    Absolute,
    ClampedDifferentiable(f32, f32),
    // TODO - more value types? Something for delay lines?
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
}

impl State {
    pub fn new(size: usize) -> Self {
        Self {
            values: vec![0.0_f32; size],
            outputs: vec![0.0_f32; 2],
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn update_data(&self, delta_time: f32) -> StateUpdate {
        StateUpdate {
            updates: vec![0.0_f32; self.len()],
            update_types: vec![UpdateType::Differentiable; self.len()],
            delta_time,
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

    pub fn temp_update(&self, update: &StateUpdate, weight: f32) -> State {
        let mut temp_copy = Self {
            values: self.values.clone(),
            outputs: self.outputs.clone(),
        };

        for i in 0..self.len() {
            match update.update_types[i] {
                UpdateType::Differentiable => temp_copy.values[i] += update.updates[i] * weight,
                UpdateType::Absolute => temp_copy.values[i] = update.updates[i],
                UpdateType::ClampedDifferentiable(lo, hi) => {
                    let v = temp_copy.values[i] + (update.updates[i] * weight).min(hi).max(lo);
                    temp_copy.values[i] = v;
                }
            }
        }

        temp_copy
    }

    pub fn apply_updates(&mut self, updates: &[StateUpdate], weights: &[f32], dt: f32) {
        debug_assert!(updates.len() == weights.len());

        let weights_sum: f32 = weights.iter().sum();

        for i in 0..self.len() {
            let mut is_absolute = false;
            let mut update = 0.0_f32;

            let mut lo_bound = -f32::MAX;
            let mut hi_bound = f32::MAX;

            for j in 0..updates.len() {
                match updates[j].update_types[i] {
                    UpdateType::Absolute => {
                        is_absolute = true;
                        update += updates[j].updates[i] * weights[j];
                    }
                    UpdateType::Differentiable => {
                        update += updates[j].updates[i] * weights[j] * dt;
                    }
                    UpdateType::ClampedDifferentiable(lo, hi) => {
                        update += updates[j].updates[i] * weights[j] * dt;
                        lo_bound = lo_bound.max(lo);
                        hi_bound = hi_bound.min(hi);
                    }
                }
            }

            if is_absolute {
                if weights_sum <= f32::EPSILON {
                    self.values[i] = update;
                } else {
                    self.values[i] = update / weights_sum;
                }
            } else {
                self.values[i] = (self.values[i] + update).max(lo_bound).min(hi_bound);
            }
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
}
