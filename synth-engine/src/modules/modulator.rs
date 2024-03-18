use crate::midi::message::MidiMessage;
use crate::modules::Module;
use crate::state::OutputType;

const OUT: usize = 0;

const IN1: usize = 0;
const IN2: usize = 1;

pub struct Modulator {
    pub inputs: Vec<usize>,
    pub outputs: Vec<usize>,
}

impl Modulator {
    pub fn new() -> Self {
        Self {
            inputs: vec![0; 2],
            outputs: vec![0; 1],
        }
    }

    pub fn new_with_connections(ins: Vec<usize>, outs: Vec<usize>) -> Self {
        assert_eq!(ins.len(), 2);
        assert_eq!(outs.len(), 1);

        Self {
            inputs: ins.clone(),
            outputs: outs.clone(),
        }
    }
}

impl Module for Modulator {
    fn simulate(&self, _dt: f32, state: &Vec<f32>, out: &mut Vec<f32>) {
        out[self.outputs[OUT]] = state[self.inputs[IN1]] * state[self.inputs[IN2]];
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
