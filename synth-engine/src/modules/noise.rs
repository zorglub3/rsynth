use crate::event::ControllerEvent;
use crate::interpolation::Interpolation;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use core::u32::MAX;

// From the book, "Musical applications of Microprocessors", Chamberlin
pub const A_PARAMETER_DEFAULT: u32 = 196314165;
pub const B_PARAMETER_DEFAULT: u32 = 907633515;

pub struct NoiseGenerator {
    a: u32,
    b: u32,
    data: [f32; 4],
    m: u32,
    output_index: usize,
}

impl NoiseGenerator {
    pub fn new(a: u32, b: u32, m: u32, output_index: usize) -> Self {
        let data = [0.; 4];
        NoiseGenerator {
            a,
            b,
            data,
            m,
            output_index,
        }
    }

    pub fn new_with_default(seed: u32, output_index: usize) -> Self {
        Self::new(A_PARAMETER_DEFAULT, B_PARAMETER_DEFAULT, seed, output_index)
    }

    pub fn next(&self, state: u32) -> u32 {
        state.wrapping_mul(self.a).wrapping_add(self.b)
    }
}

impl Module for NoiseGenerator {
    fn simulate(&self, _state: &State, update: &mut StateUpdate) {
        update.set(
            self.output_index,
            self.data
                .as_slice()
                .cubic_interpolate(1. + update.get_delta_time()),
            UpdateType::Absolute,
        );
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, _state: &mut State, _time_step: f32) {
        self.data.copy_within(0..3, 1);
        self.m = self.next(self.m);
        self.data[0] = 2. * (self.m as f32) / (MAX as f32) - 1.;
    }
}
