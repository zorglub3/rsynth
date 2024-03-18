use crate::midi::message::MidiMessage;
use crate::modules::Module;
use crate::state::OutputType;
use core::f32::consts::PI;

const OUT_HP: usize = 0;
const OUT_BP: usize = 1;
const OUT_LP: usize = 2;
const OUT_NOTCH: usize = 3;

const IN_FREQ0: usize = 0;
const IN_FREQ: usize = 1;
const IN_Q: usize = 2;
const IN: usize = 3;

pub struct Filter12db {
    pub t: f32,
    pub inputs: Vec<usize>,
    pub outputs: Vec<usize>,
}

fn amp(t: f32, f0: f32, f: f32) -> f32 {
    let x = f0 * 2.0_f32.powf(f);

    1. - (-2. * PI * x * t).exp()
}

fn feedback(q: f32) -> f32 {
    1. / q.clamp(0.0001, 1000.)
}

impl Filter12db {
    pub fn new(t: f32) -> Self {
        Self {
            t,
            inputs: vec![0; 4],
            outputs: vec![0; 4],
        }
    }

    pub fn new_with_connections(t: f32, ins: Vec<usize>, outs: Vec<usize>) -> Self {
        assert_eq!(ins.len(), 4);
        assert_eq!(outs.len(), 4);

        Self {
            t,
            inputs: ins.clone(),
            outputs: outs.clone(),
        }
    }
}

impl Module for Filter12db {
    fn simulate(&self, _dt: f32, state: &Vec<f32>, out: &mut Vec<f32>) {
        let a = amp(
            self.t,
            state[self.inputs[IN_FREQ0]],
            state[self.outputs[IN_FREQ]],
        );
        let b = feedback(state[self.inputs[IN_Q]]);

        let hp =
            state[self.inputs[IN]] - state[self.outputs[OUT_LP]] - state[self.outputs[OUT_BP]] * b;
        let bp = state[self.outputs[OUT_HP]] * a;
        let lp = state[self.outputs[OUT_BP]] * a;

        out[self.outputs[OUT_HP]] = hp;
        out[self.outputs[OUT_BP]] = bp;
        out[self.outputs[OUT_LP]] = lp;

        out[self.outputs[OUT_NOTCH]] = hp + lp;
    }

    fn finalize(&self, _state: &mut Vec<f32>) {
        /* do nothing */
    }

    fn inputs(&self) -> Vec<usize> {
        self.inputs.clone()
    }

    fn outputs(&self) -> Vec<usize> {
        self.outputs.clone()
    }

    fn output_types(&self) -> Vec<OutputType> {
        vec![OutputType::Differentiable; 4]
    }

    fn process_event(&mut self, _event: &MidiMessage, _channel: u8) {
        /* do nothing */
    }
}
