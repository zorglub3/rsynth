use clap::Parser;
use std::f32::consts::PI;
use synth_engine::control_interface::ControlInterface;
use synth_engine::modules::SynthModule;
use synth_engine::simulator::Simulator;
use synth_engine::stack_program::*;
use synth_engine::{modules::*, simulator::rungekutta::RungeKutta};

const STACK_SIZE: usize = 256;
const OUTPUT_SIZE: usize = 2;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long)]
    test: usize,
    #[arg(short, long)]
    count: usize,
    #[arg(short, long)]
    sample_rate: f32,
    #[arg(long)]
    simulator: String,
}

fn test_simulator(
    simulator_name: &str,
    state_size: usize,
    input_size: usize,
) -> Box<dyn Simulator> {
    match simulator_name {
        "rk4" => Box::new(RungeKutta::rk4(
            state_size,
            input_size,
            OUTPUT_SIZE,
            STACK_SIZE,
        )),
        "rk38" => Box::new(RungeKutta::rk38(
            state_size,
            input_size,
            OUTPUT_SIZE,
            STACK_SIZE,
        )),
        "euler" => Box::new(RungeKutta::euler(
            state_size,
            input_size,
            OUTPUT_SIZE,
            STACK_SIZE,
        )),
        _ => panic!("Unsupported Runge Kutta simulator {}", simulator_name),
    }
}

fn test_modules(test: usize) -> Vec<SynthModule> {
    let mut result: Vec<SynthModule> = Vec::new();

    match test {
        0 => {
            result.push(SynthModule::QuadOscillator(QuadratureOscillator::new(
                1.,
                4,
                5,
                StackProgram::constant(0.),
                StackProgram::constant(110.),
            )));
            result.push(SynthModule::Wavefolder(Folder::new(
                StackProgram::zero(),
                StackProgram::zero(),
                1,
            )));
            result.push(SynthModule::Output(MonoOutput::new(
                0,
                StackProgram::from_index(1),
            )));
        }
        1 => {
            result.push(SynthModule::QuadOscillator(QuadratureOscillator::new(
                1.,
                1,
                2,
                StackProgram::constant(0.),
                StackProgram::constant(110.),
            )));
            result.push(SynthModule::Output(MonoOutput::new(
                0,
                StackProgram::from_index(1),
            )));
        }
        2 => {
            result.push(SynthModule::Bowed(BowedOscillator::new(
                1.,
                500.0,
                1,
                2,
                StackProgram::constant(0.),
                StackProgram::constant(100.),
                StackProgram::constant(500.),
                StackProgram::constant(0.3),
            )));
            result.push(SynthModule::Output(MonoOutput::new(
                0,
                StackProgram::from_index(2),
            )));
        }
        3 => {
            let mut wavetable1: Vec<f32> = Vec::new();
            let mut wavetable2: Vec<f32> = Vec::new();
            for i in 0..256 {
                let x = (i as f32) / 256.;
                let v = 0.5 * ((i as f32) * 2. * PI / 256.).sin() + x - 0.5;
                wavetable1.push(v);
                let v2 = ((i as f32) * 2. * PI / 256.).sin();
                wavetable2.push(v2);
            }
            result.push(SynthModule::WavetableOscillator(Wavetable::new(
                0.,
                1,
                2,
                StackProgram::constant(0.),
                StackProgram::constant(5000.),
                StackProgram::constant(0.5),
                vec![wavetable2, wavetable1],
            )));
            result.push(SynthModule::Output(MonoOutput::new(
                0,
                StackProgram::from_index(2),
            )));
        }
        4 => {
            result.push(SynthModule::Noise(NoiseGenerator::new_with_default(1, 2)));
            result.push(SynthModule::Output(MonoOutput::new(
                0,
                StackProgram::from_index(2),
            )));
        }
        5 => {
            todo!("filter sweep");
        }
        _ => panic!("No test for {}", test),
    }

    result
}

fn main() {
    let args = CliArgs::parse();

    let mut simulator = test_simulator(&args.simulator, 32, 32);

    simulator.set_modules(test_modules(args.test));

    let dt = 1.0 / args.sample_rate;

    let control_interface = ControlInterface::new();

    for _i in 0..args.count {
        simulator.step(dt, &control_interface);

        let output = simulator.get_stereo_output();

        println!("{}", output.0);
    }
}
