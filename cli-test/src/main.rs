use clap::Parser;
use std::f32::consts::PI;
use synth_engine::stack_program::*;
use synth_engine::{modules::*, simulator::module::Module, simulator::rungekutta::RungeKutta};

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

fn test_simulator(simulator_name: &str, state_size: usize) -> RungeKutta {
    match simulator_name {
        "rk4" => RungeKutta::rk4(state_size),
        "rk38" => RungeKutta::rk38(state_size),
        "euler" => RungeKutta::euler(state_size),
        _ => panic!("Unsupported Runge Kutta simulator {}", simulator_name),
    }
}

fn test_modules(test: usize) -> Vec<Box<dyn Module>> {
    let mut result: Vec<Box<dyn Module>> = Vec::new();

    match test {
        0 => {
            result.push(Box::new(QuadratureOscillator::new(
                1.,
                4,
                5,
                StackProgram::constant(0.),
                StackProgram::constant(110.),
            )));
            result.push(Box::new(Folder::new(
                StackProgram::zero(),
                StackProgram::zero(),
                1,
            )));
            result.push(Box::new(MonoOutput::new(0, StackProgram::from_index(1))));
        }
        1 => {
            result.push(Box::new(QuadratureOscillator::new(
                1.,
                1,
                2,
                StackProgram::constant(0.),
                StackProgram::constant(110.),
            )));
            result.push(Box::new(MonoOutput::new(0, StackProgram::from_index(1))));
        }
        2 => {
            result.push(Box::new(BowedOscillator::new(
                1.,
                500.0,
                1,
                2,
                StackProgram::constant(0.),
                StackProgram::constant(100.),
                StackProgram::constant(500.),
                StackProgram::constant(0.3),
            )));
            result.push(Box::new(MonoOutput::new(0, StackProgram::from_index(2))));
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
            result.push(Box::new(Wavetable::new(
                0.,
                1,
                2,
                StackProgram::constant(0.),
                StackProgram::constant(5000.),
                StackProgram::constant(0.5),
                vec![wavetable2, wavetable1],
            )));
            result.push(Box::new(MonoOutput::new(0, StackProgram::from_index(2))));
        }
        4 => {
            result.push(Box::new(NoiseGenerator::new_with_default(1, 2)));
            result.push(Box::new(MonoOutput::new(0, StackProgram::from_index(2))));
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

    let mut simulator = test_simulator(&args.simulator, 32).with_modules(test_modules(args.test));

    let dt = 1.0 / args.sample_rate;

    for _i in 0..args.count {
        simulator.step(dt);

        let output = simulator.get_stereo_output();

        println!("{}", output.0);
    }
}
