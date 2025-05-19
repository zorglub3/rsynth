use super::control_to_frequency;
use crate::control_interface::ControlInterface;
use crate::interpolation::Interpolation;
use crate::simulator::module::Module;
use crate::simulator::state::UpdateType;
// use crate::stack_program::*;
use alloc::vec;
use alloc::vec::Vec;

pub struct DelayLine {
    data: Vec<f32>,
    current_index: usize,
    // signal_output: usize,
    f0: f32,
    // signal_input: StackProgram,
    // pitch_control: StackProgram,
    // linear_modulation: StackProgram,
}

impl DelayLine {
    pub const STATE_SIZE: usize = 1;
    pub const INPUT_SIZE: usize = 3;

    // state/outputs
    pub const SIGNAL_OUTPUT: usize = 0;

    // inputs/control
    pub const SIGNAL_INPUT: usize = 0;
    pub const EXP_CONTROL_INPUT: usize = 1;
    pub const LINEAR_CONTROL_INPUT: usize = 2;

    pub fn new(
        f0: f32,
        // signal_output: usize,
        // signal_input: StackProgram,
        // pitch_control: StackProgram,
        // linear_modulation: StackProgram,
        data_size: usize,
    ) -> Self {
        Self {
            f0,
            // signal_output,
            // signal_input,
            // pitch_control,
            // linear_modulation,
            current_index: 0,
            data: vec![0.; data_size],
        }
    }

    fn index_modulo(&self, index: usize) -> usize {
        let size = self.data.len();

        ((index % size) + size) % size
    }

    fn increment_index(&mut self) {
        self.current_index = self.index_modulo(self.current_index + 1);
    }

    fn write_index(&self) -> usize {
        self.current_index
    }
}

impl Module for DelayLine {
    fn simulate(
        &self,
        _control_interface: &ControlInterface,
        inputs: &[f32],
        state: &[f32],
        update: &mut [f32],
        dt: f32,
    ) {
        todo!()
        /*
        let wi = self.write_index() as f32;
        let l = self.data.len() as f32;
        let d: f32 = todo!(); // old update.get_time_step();
        let s: f32 = todo!(); // old update.get_delta_time();
        
        let f = control_to_frequency(
            self.f0,
            inputs[EXP_CONTROL_INPUT],
            inputs[LINEAR_CONTROL_INPUT],
        );

        let index = (1. / (d * f) - s / d).clamp(5., l - 5.);
        let index = (((wi - index) % l) + l) % l;

        update[SIGNAL_OUTPUT] = self.data.lagrange_interpolate(index);
        */
    }

    fn finalize(&mut self, inputs: &[f32], _state: &mut [f32], _outputs: &mut [f32], _dt: f32) {
        let write_index = self.write_index();

        self.data[write_index] = inputs[DelayLine::SIGNAL_INPUT];

        self.increment_index();
    }

    fn get_input_size(&self) -> usize {
        DelayLine::INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        DelayLine::STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        update_types[DelayLine::SIGNAL_OUTPUT] = UpdateType::Absolute;
    }
}
