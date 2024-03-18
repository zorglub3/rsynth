use crate::midi::message::MidiMessage;
use crate::modules::Module;
use crate::state::OutputType;

const OUT: usize = 0;

const IN: usize = 0;
const CTRL_LIN: usize = 1;
const CTRL_EXP: usize = 2;

pub struct Amplifier {
    pub inputs: Vec<usize>,
    pub outputs: Vec<usize>,
}

impl Amplifier {
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

fn exp_gain(v: f32) -> f32 {
    2. * (2.0_f32.powf(v) - 1.)
}

impl Module for Amplifier {
    fn simulate(&self, _dt: f32, state: &Vec<f32>, out: &mut Vec<f32>) {
        let gain = state[self.inputs[CTRL_LIN]] * 2. + exp_gain(state[self.inputs[CTRL_EXP]]);

        out[self.outputs[OUT]] = state[self.inputs[IN]] * gain;
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
        vec![OutputType::Absolute]
    }

    fn process_event(&mut self, _event: &MidiMessage, _channel: u8) {
        /* do nothing */
    }
}
