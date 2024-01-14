#![feature(slice_pattern)]

mod cpu;
mod memory;

use crate::cpu::CPU;

fn main() {
    let args = std::env::args();
    let mut arg_iter = args.skip_while(|x| x.contains(env!("CARGO_PKG_NAME")));

    let rom = arg_iter.next();

    // TODO: Hande loading a boot rom

    let rom_buffer = if let Some(rom_file) = rom {
        buffer_from_file(rom_file.as_str())
    } else {
        panic!("Cannot run emulator without a rom");
    };

    let cpu = CPU::new(None, rom_buffer);
    run(cpu);
}

fn run(mut cpu: CPU) {
    cpu.memory.verify_logo();
}

fn buffer_from_file(path: &str) -> Vec<u8> {
    std::fs::read(path).expect(format!("Cannot read file at path: {}", path).as_str())
}
