use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::BuildStreamError;
use cpal::{
    Device, PlayStreamError, SampleFormat, SampleRate, SupportedBufferSize, SupportedStreamConfig,
    SupportedStreamConfigRange,
};
use scale::Scale;
use std::sync::mpsc::Receiver;
use synth_engine::event::ControllerEvent;
use synth_engine::simulator::rungekutta::RungeKutta;

pub struct AudioStream(Box<dyn StreamTrait>);

impl AudioStream {
    pub fn play(&self) -> Result<(), PlayStreamError> {
        self.0.play()
    }
}

fn supports_buffer_size(config_buffer_size: &SupportedBufferSize, buffer_size: u32) -> bool {
    match config_buffer_size {
        SupportedBufferSize::Range { min, max } => min <= &buffer_size && max >= &buffer_size,
        _ => false,
    }
}

fn get_sound_config(
    device: &Device,
    sample_rate: u32,
    buffer_size: u32,
) -> Option<SupportedStreamConfig> {
    let supported_configs = device
        .supported_output_configs()
        .expect("no supported configs");
    let mut chosen_config: Option<SupportedStreamConfigRange> = None;

    let sample_rate = SampleRate(sample_rate);

    for config in supported_configs {
        if sample_rate >= config.min_sample_rate()
            && sample_rate <= config.max_sample_rate()
            && supports_buffer_size(config.buffer_size(), buffer_size)
            && config.sample_format() == SampleFormat::F32
            && config.channels() == 2
        {
            chosen_config = Some(config);
        }
    }

    chosen_config.map(|config| config.with_sample_rate(sample_rate))
}

pub fn sound_simulation(
    sample_rate: u32,
    buffer_size: u32,
    mut simulation: Box<RungeKutta>,
    receiver: Receiver<ControllerEvent>,
    scale: Scale,
    pitch_wheel_range: f32,
    debug_events: bool,
) -> Result<AudioStream, BuildStreamError> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");

    let stream_config =
        get_sound_config(&device, sample_rate, buffer_size).expect("No applicable config");

    println!("sample rate: {}", stream_config.sample_rate().0);
    println!("buffer size: {:?}", stream_config.buffer_size());

    let dt = 1.0 / (stream_config.sample_rate().0 as f32);

    let num_channels = stream_config.channels() as usize;

    println!("channels: {}", num_channels);
    println!("delta: {}", dt);

    let stream = device.build_output_stream(
        &stream_config.config(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(num_channels) {
                simulation.step(dt);

                let (left, right) = simulation.get_stereo_output();

                let frame_size = frame.len();

                frame[0] = left;

                if frame_size > 1 {
                    frame[1] = right;
                }
            }

            loop {
                if let Some(event) = receiver.try_recv().ok() {
                    use ControllerEvent::*;

                    let event = match event {
                        NoteOn {
                            pitch, velocity, ..
                        } => NoteOn {
                            pitch,
                            velocity,
                            pitch_value: scale.pitch_value(pitch as usize).unwrap_or(0.),
                        },
                        PitchWheel { amount } => PitchWheel {
                            amount: amount * pitch_wheel_range,
                        },
                        e => e,
                    };

                    if debug_events {
                        println!("controller event: {:?}", event);
                    }

                    simulation.process_event(event);
                } else {
                    break;
                }
            }
        },
        move |err| {
            eprintln!("Error occurred in the output stream: {:?}", err);
        },
        None, // None=blocking,
    );

    stream.map(|s| AudioStream(Box::new(s)))
}
