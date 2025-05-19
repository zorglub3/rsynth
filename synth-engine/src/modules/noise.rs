use crate::control_interface::ControlInterface;
use crate::interpolation::Interpolation;
use crate::simulator::module::Module;
use crate::simulator::state::UpdateType;
use core::u32::MAX;

// From the book, "Musical applications of Microprocessors", Chamberlin
pub const A_PARAMETER_DEFAULT: u32 = 196314165;
pub const B_PARAMETER_DEFAULT: u32 = 907633515;

pub struct NoiseGenerator {
    a: u32,
    b: u32,
    data: [f32; 4],
    m: u32,
    // output_index: usize,
}

impl NoiseGenerator {
    pub const STATE_SIZE: usize = 1;
    pub const INPUT_SIZE: usize = 0;

    // states/output
    pub const SIGNAL_OUTPUT: usize = 0;

    pub fn new(
        a: u32, 
        b: u32, 
        m: u32, 
        // output_index: usize,
    ) -> Self {
        let data = [0.; 4];
        NoiseGenerator {
            a,
            b,
            data,
            m,
            // output_index,
        }
    }

    pub fn new_with_default(seed: u32/*, output_index: usize*/) -> Self {
        Self::new(A_PARAMETER_DEFAULT, B_PARAMETER_DEFAULT, seed/*, output_index*/)
    }

    pub fn next(&self, state: u32) -> u32 {
        state.wrapping_mul(self.a).wrapping_add(self.b)
    }
}

impl Module for NoiseGenerator {
    fn simulate(
        &self,
        control_interface: &ControlInterface,
        _inputs: &[f32],
        _state: &[f32],
        update: &mut [f32],
        dt: f32,
    ) {
        update[NoiseGenerator::SIGNAL_OUTPUT] = self.data.as_slice().cubic_interpolate(1. + dt);
    }

    fn finalize(&mut self, _inputs: &[f32], _state: &mut [f32], _outputs: &mut [f32], _dt: f32) {
        self.data.copy_within(0..3, 1);
        self.m = self.next(self.m);
        self.data[0] = 2. * (self.m as f32) / (MAX as f32) - 1.;
    }

    fn get_input_size(&self) -> usize {
        NoiseGenerator::INPUT_SIZE
    }

    fn get_state_size(&self) -> usize {
        NoiseGenerator::STATE_SIZE
    }

    fn set_update_type(&self, update_types: &mut [UpdateType]) {
        update_types[NoiseGenerator::SIGNAL_OUTPUT] = UpdateType::Absolute;
    }
}
