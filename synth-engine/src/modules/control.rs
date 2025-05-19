use crate::control_interface::ControlInterface;
use crate::simulator::module::Module;
use crate::simulator::state::UpdateType;
use core::f32::consts::PI;

pub struct ContinuousControl {
    // output_index: usize,
    control: usize,
    value: f32,
    min_value: f32,
    max_value: f32,
    filter_freq: Option<f32>,
}

impl ContinuousControl {
    pub const STATE_SIZE: usize = 1;
    pub const INPUT_SIZE: usize = 0;

    // state/outputs
    pub const CONTROL_OUTPUT: usize = 0;

    // no inputs/control (the control is external to the simulation)

    pub fn new(
        // output_index: usize, 
        control: usize, 
        min_value: f32, 
        max_value: f32,
        filter_freq: Option<f32>,
    ) -> Self {
        Self {
            // output_index,
            control,
            value: 0.,
            min_value,
            max_value,
            filter_freq,
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
            let d = v - state[ContinuousControl::CONTROL_OUTPUT];

            update[ContinuousControl::CONTROL_OUTPUT] = k * d;
        } else {
            update[ContinuousControl::CONTROL_OUTPUT] = 
                self.compute_value(control_interface.get_continuous_control(self.control));
        }
    }

    fn finalize(&mut self, _inputs: &[f32], _state: &mut [f32], _outputs: &mut [f32], _dt: f32) {
        /* do nothing */
    }

    fn get_input_size(&self) -> usize {
        ContinuousControl::INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        ContinuousControl::STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        if self.filter_freq.is_some() {
            update_types[ContinuousControl::CONTROL_OUTPUT] = UpdateType::Differentiable;
        } else {
            update_types[ContinuousControl::CONTROL_OUTPUT] = UpdateType::Absolute;
        }
    }
}
