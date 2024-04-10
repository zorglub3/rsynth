use crate::midi::message::MidiMessage;
use crate::modules::Module;
use crate::state::OutputType;

const GATE_OUT: usize = 0;
const PITCH_OUT: usize = 1;
const VELOCITY_OUT: usize = 2;
const AFTERTOUCH_OUT: usize = 3;

pub struct MidiMono {
    pub channel: Option<u8>,
    pub last_pitch: Option<u8>,
    pub outputs: Vec<usize>,
    pub pitch_value: f32,
    pub velocity_value: f32,
    pub gate_value: f32,
    pub aftertouch_value: f32,
}

impl MidiMono {
    pub fn new(channel: Option<u8>) -> Self {
        Self {
            channel,
            last_pitch: None,
            outputs: vec![0; 4],
            pitch_value: 0.,
            velocity_value: 0.,
            gate_value: 0.,
            aftertouch_value: 0.,
        }
    }

    pub fn new_with_connections(channel: Option<u8>, outs: Vec<usize>) -> Self {
        assert_eq!(outs.len(), 4);

        Self {
            channel,
            last_pitch: None,
            outputs: outs.clone(),
            pitch_value: 0.,
            velocity_value: 0.,
            gate_value: 0.,
            aftertouch_value: 0.,
        }
    }
}

impl Module for MidiMono {
    fn simulate(&self, _dt: f32, _state: &Vec<f32>, out: &mut Vec<f32>) {
        out[self.outputs[GATE_OUT]] = self.gate_value;
        out[self.outputs[PITCH_OUT]] = self.pitch_value;
        out[self.outputs[VELOCITY_OUT]] = self.velocity_value;
        out[self.outputs[AFTERTOUCH_OUT]] = self.aftertouch_value;
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
        vec![OutputType::Absolute; 4]
    }

    fn process_event(&mut self, event: &MidiMessage, channel: u8) {
        if self.channel.is_none() || Some(channel) == self.channel {
            match event {
                MidiMessage::NoteOn { pitch, velocity } => {
                    println!("Midi note on: {pitch}, {velocity}");

                    self.pitch_value = (*pitch as f32) / 12.;
                    self.velocity_value = (*velocity as f32) / 127.;
                    self.gate_value = 1.;
                    self.last_pitch = Some(*pitch);
                }
                MidiMessage::NoteOff { pitch, velocity } if self.last_pitch == Some(*pitch) => {
                    self.velocity_value = (*velocity as f32) / 127.;
                    self.gate_value = 0.;
                    self.last_pitch = None;
                }
                MidiMessage::ChannelAftertouch { amount } => {
                    self.aftertouch_value = (*amount as f32) / 127.;
                }
                MidiMessage::PolyphonicAftertouch { pitch, amount }
                    if Some(*pitch) == self.last_pitch =>
                {
                    self.aftertouch_value = (*amount as f32) / 127.;
                }
                _ => { /* do nothing */ }
            }
        }
    }
}
