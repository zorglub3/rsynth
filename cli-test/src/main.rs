use clap::Parser;
use std::collections::HashMap;
use synth_engine::{
    modules::bowed::BowedOscillator, modules::folder::Folder,
    modules::quadrature::QuadratureOscillator, simulator::module::Module,
    simulator::rungekutta::RungeKutta,
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
                "quad_osc".to_string(),
                Box::new(QuadratureOscillator::new(110., 4, 5, 6)),
            );
            result.insert("folder".to_string(), Box::new(Folder::new(4, 5, 1)));
        }
        1 => {
            result.insert(
                "quad_osc".to_string(),
                Box::new(QuadratureOscillator::new(110., 1, 2, 3)),
            );
        }
        2 => {
            result.insert(
                "bowed_osc".to_string(),
                Box::new(BowedOscillator::new(50., 5.0, 2, 1, 0, 0, 0)),
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
