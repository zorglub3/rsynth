use crate::event::ControllerEvent;
use crate::interpolation::Interpolation;
use crate::modules::input_expr::InputExpr;
use crate::simulator::module::Module;
use crate::simulator::state::{State, StateUpdate, UpdateType};
use std::f32::consts::PI;

pub struct Wavetable {
    f0: f32,
    position_state_index: usize,
    signal_output: usize,
    pitch_control: InputExpr,
    linear_modulation: InputExpr,
    wavetable: Vec<f32>,
    cumulative_sum: Vec<f32>,
}

fn compute_cumulative_sum(wavetable: &Vec<f32>) -> Vec<f32> {
    let mut result = vec![0.; wavetable.len()];
    let l = wavetable.len();

    for index in 0..l {
        let a = index;
        let b = (index + 1) % l;

        let f_a = wavetable[a];
        let f_b = wavetable[b];
        let f_mid = wavetable.cubic_interpolate((index as f32) + 0.5);

        // Simpsons rule, see
        // https://en.wikipedia.org/wiki/Simpson%27s_rule
        let v = (((b - a) as f32) / 6.) * (f_a + 4. * f_mid + f_b);

        result[index] = v;
    }

    for index in 1..l {
        result[index] += result[index - 1];
    }

    result
}

impl Wavetable {
    pub fn new(
        f0: f32,
        position_state_index: usize,
        signal_output: usize,
        pitch_control: InputExpr,
        linear_modulation: InputExpr,
        wavetable: Vec<f32>,
    ) -> Self {
        let cumulative_sum = compute_cumulative_sum(&wavetable);

        Self {
            f0,
            position_state_index,
            signal_output,
            pitch_control,
            linear_modulation,
            wavetable,
            cumulative_sum,
        }
    }

    fn integral(start: f32, end: f32) -> f32 {
        todo!()
    }

    fn control_to_frequency(&self, exp_fc: f32, lin_fc: f32) -> f32 {
        let f = self.f0 * 2.0_f32.powf(exp_fc) + lin_fc;

        f * (self.wavetable.len() as f32)
    }
}

impl Module for Wavetable {
    fn simulate(&self, state: &State, update: &mut StateUpdate) {
        let exp_fc = self.pitch_control.from_state(state);
        let lin_fc = self.linear_modulation.from_state(state);

        let velocity = self.control_to_frequency(exp_fc, lin_fc);

        todo!()
    }

    fn process_event(&mut self, _event: &ControllerEvent) {
        /* do nothing */
    }

    fn finalize(&mut self, state: &mut State, _time_step: f32) {
        todo!()
    }
}
