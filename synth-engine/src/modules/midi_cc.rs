use crate::midi::message::MidiMessage;
use crate::modules::Module;
use crate::state::OutputType;

const OUT: usize = 0;

pub struct MidiCC {
    pub channel: Option<u8>,
    pub controller: u8,
    pub outputs: Vec<usize>,
    pub value: f32,
}

impl MidiCC {
    pub fn new(channel: Option<u8>, controller: u8) -> Self {
        let outputs = vec![0];
        let value = 0.;

        Self {
            channel,
            controller,
            outputs,
            value,
        }
    }

    pub fn new_with_connections(channel: Option<u8>, controller: u8, outs: Vec<usize>) -> Self {
        assert_eq!(outs.len(), 1);

        Self {
            channel,
            controller,
            outputs: outs.clone(),
            value: 0.,
        }
    }
}

impl Module for MidiCC {
    fn simulate(&self, _dt: f32, _state: &Vec<f32>, out: &mut Vec<f32>) {
        out[self.outputs[OUT]] = self.value;
    }

    fn finalize(&self, _state: &mut Vec<f32>) {
        /* do nothing */
    }

    fn inputs(&self) -> Vec<usize> {
        vec![]
    }

    fn outputs(&self) -> Vec<usize> {
        self.outputs.clone()
    }

    fn output_types(&self) -> Vec<OutputType> {
        vec![OutputType::Absolute]
    }

    fn process_event(&mut self, event: &MidiMessage, channel: u8) {
        if self.channel.is_none() || Some(channel) == self.channel {
            match event {
                MidiMessage::ContinuousControl { control, value }
                    if *control == self.controller =>
                {
                    self.value = (*value as f32) / 127.
                }
                _ => {}
            }
        }
    }
}
