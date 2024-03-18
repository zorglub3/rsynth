use crate::midi::message::MidiMessage;
use crate::modules::Module;
use crate::state::OutputType;

const OUT: usize = 0;

const IN: usize = 0;
const SLOPE_UP: usize = 1;
const SLOPE_DOWN: usize = 2;

pub struct AREnvelope {
    pub inputs: Vec<usize>,
    pub outputs: Vec<usize>,
}

impl AREnvelope {
    pub fn new() -> Self {
        Self {
            inputs: vec![0; 3],
            outputs: vec![0; 1],
        }
    }

    pub fn new_with_connections(ins: Vec<usize>, outs: Vec<usize>) -> Self {
        Self {
            inputs: ins.clone(),
            outputs: outs.clone(),
        }
    }
}

fn slope(v: f32) -> f32 {
    1. - v.clamp(0.0001, 0.999) // TODO - this is place holder for now
}

impl Module for AREnvelope {
    fn simulate(&self, _dt: f32, state: &Vec<f32>, out: &mut Vec<f32>) {
        let current_out = state[self.outputs[OUT]];
        let current_in = state[self.inputs[IN]];

        if current_in > 0.5 {
            if current_out < 1. {
                out[self.outputs[OUT]] = slope(state[self.inputs[SLOPE_UP]]);
            }
        } else if current_out > 0. {
            out[self.outputs[OUT]] = slope(state[self.inputs[SLOPE_DOWN]]);
        }
    }

    fn finalize(&self, state: &mut Vec<f32>) {
        let out = state[self.outputs[OUT]];
        state[self.outputs[OUT]] = out.clamp(0., 1.);
    }

    fn inputs(&self) -> Vec<usize> {
        self.inputs.clone()
    }

    fn outputs(&self) -> Vec<usize> {
        self.outputs.clone()
    }

    fn output_types(&self) -> Vec<OutputType> {
        vec![OutputType::Differentiable]
    }

    fn process_event(&mut self, _event: &MidiMessage, _channel: u8) {
        /* do nothing */
    }
}
