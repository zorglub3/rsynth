use crate::midi::message::MidiMessage;
use crate::modules::Module;
use crate::state::OutputType;
use core::f32::consts::PI;

const STATE: usize = 0;
const SAW_OUT: usize = 1;
const SQR_OUT: usize = 2;
const TRI_OUT: usize = 3;
const SIN_OUT: usize = 4;

const FREQ0: usize = 0;
const FREQ: usize = 1;
const LIN_FREQ: usize = 2;

fn modulo_one(v: f32) -> f32 {
    ((v % 1.) + 1.) % 1.
}

pub struct RelaxationOscillator {
    pub inputs: Vec<usize>,
    pub outputs: Vec<usize>,
}

impl RelaxationOscillator {
    pub fn new() -> Self {
        Self {
            inputs: vec![0; 3],
            outputs: vec![0; 5],
        }
    }

    pub fn new_with_connections(ins: Vec<usize>, outs: Vec<usize>) -> Self {
        assert!(ins.len() == 3);
        assert!(outs.len() == 5);

        Self {
            inputs: ins.clone(),
            outputs: outs.clone(),
        }
    }
}

fn freq_to_step(f0: f32, freq: f32, lin_freq: f32) -> f32 {
    (f0 * 2.0_f32.powf(freq)) + lin_freq * 100.0
}

fn lookup_saw(v: f32) -> f32 {
    v * 2.0 - 1.0
}

fn lookup_tri(v: f32) -> f32 {
    if v < 0.5 {
        v * 4.0 - 1.0
    } else {
        3.0 - 4.0 * v
    }
}

fn lookup_sqr(v: f32) -> f32 {
    if v < 0.5 {
        1.0
    } else {
        -1.0
    }
}

fn lookup_sin(v: f32) -> f32 {
    (v * 2.0 * PI).sin()
}

impl Module for RelaxationOscillator {
    fn simulate(&self, _dt: f32, state: &Vec<f32>, out: &mut Vec<f32>) {
        out[self.outputs[STATE]] = freq_to_step(
            state[self.inputs[FREQ0]],
            state[self.inputs[FREQ]],
            state[self.inputs[LIN_FREQ]],
        );

        let pp = modulo_one(state[self.outputs[STATE]]);

        out[self.outputs[SAW_OUT]] = lookup_saw(pp);
        out[self.outputs[TRI_OUT]] = lookup_tri(pp);
        out[self.outputs[SQR_OUT]] = lookup_sqr(pp);
        out[self.outputs[SIN_OUT]] = lookup_sin(pp);
    }

    fn finalize(&self, state: &mut Vec<f32>) {
        let v = state[self.outputs[STATE]];
        state[self.outputs[STATE]] = modulo_one(v);
    }

    fn inputs(&self) -> Vec<usize> {
        self.inputs.clone()
    }

    fn outputs(&self) -> Vec<usize> {
        self.outputs.clone()
    }

    fn output_types(&self) -> Vec<OutputType> {
        vec![
            OutputType::Differentiable,
            OutputType::Absolute,
            OutputType::Absolute,
            OutputType::Absolute,
            OutputType::Absolute,
        ]
    }

    fn process_event(&mut self, _event: &MidiMessage, _channel: u8) {
        /* do nothing */
    }
}
