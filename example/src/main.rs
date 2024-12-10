use synth_engine::modules::SynthModule;
use synth_engine::modules::*;
use synth_engine::stack_program::Function::*;
use synth_engine::stack_program::Instr::*;
use synth_engine::stack_program::StackProgram;

fn main() {
    let (modules, state_size) = include!(concat!(env!("OUT_DIR"), "/synth_modules.rs"));

    println!("{} modules", modules.len());
    println!("state size is {}", state_size);
}
