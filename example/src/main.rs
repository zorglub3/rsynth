#![no_std]

extern crate alloc;

use alloc::vec;
use panic_semihosting as _;
use static_alloc::Bump;
use synth_engine::modules::SynthModule;
use synth_engine::modules::*;
use synth_engine::stack_program::Function::*;
use synth_engine::stack_program::Instr::*;
use synth_engine::stack_program::StackProgram;

#[global_allocator]
static ALLOCATOR: Bump<[u8; 1 << 16]> = Bump::uninit();

fn main() {
    let (modules, state_size) = include!(concat!(env!("OUT_DIR"), "/synth_modules.rs"));

    debug_assert_eq!(modules.len(), 19);
    debug_assert_eq!(state_size, 27);
}
