use clap::Parser;
use std::collections::HashMap;
use synth_engine::{
    new_modules::saw::SawOsc, simulator::module::Module, simulator::rungekutta::RungeKutta,
    new_modules::sine::SinOsc,
    new_modules::folder::Folder,
};

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

fn test_modules(test: usize) -> HashMap<String, Box<dyn Module>> {
    let mut result: HashMap<String, Box<dyn Module>> = HashMap::new();

    match test {
        0 => {
            result.insert(
                "saw_osc".to_string(),
                Box::new(SawOsc::new(510., 0, 3, 4, 1, 20000.)),
            );
        }
        1 => {
            result.insert(
                "sin_osc".to_string(),
                Box::new(SinOsc::new(510., 0, 3, 1)),
            );
        }
        2 => {
            result.insert(
                "sin_osc".to_string(),
                Box::new(SinOsc::new(510., 0, 3, 4)),
            );
            result.insert(
                "folder".to_string(),
                Box::new(Folder::new(4, 5, 1)),
            );
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

        let output = simulator.get_output();

        println!("{}", output.0);
    }
}
