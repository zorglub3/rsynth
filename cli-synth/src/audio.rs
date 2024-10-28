use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::BuildStreamError;
use cpal::{BufferSize, PlayStreamError, SampleRate, StreamConfig};
use std::sync::mpsc::Receiver;
use synth_engine::event::Event;
use synth_engine::simulator::rungekutta::RungeKutta;

pub struct AudioStream(Box<dyn StreamTrait>);

impl AudioStream {
    pub fn play(&self) -> Result<(), PlayStreamError> {
        self.0.play()
    }
}

pub fn sound_simulation(
    sample_rate: u32,
    buffer_size: u32,
    mut simulation: Box<RungeKutta>,
    receiver: Receiver<Event>,
) -> Result<AudioStream, BuildStreamError> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");

    let config = StreamConfig {
        channels: 2,
        sample_rate: SampleRate(sample_rate),
        buffer_size: BufferSize::Fixed(buffer_size),
    };

    let dt = 1.0 / (config.sample_rate.0 as f32);

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(2) {
                simulation.step(dt);

                let (left, right) = simulation.get_stereo_output();

                // println!("left: {}, right: {}", left, right);

                frame[0] = left;
                frame[1] = right;
            }

            loop {
                if let Some(event) = receiver.try_recv().ok() {
                    println!("Got event: {:?}", event);
                    simulation.process_event(event);
                } else {
                    break;
                }
            }
        },
        move |err| {
            // react to errors here.
            eprintln!("Error occurred in the output stream: {:?}", err);
        },
        None, // None=blocking,
    );

    stream.map(|s| AudioStream(Box::new(s)))
}
