use std::process;

extern crate sdl2;

mod cpu;
mod memory;

fn main() {

    let mut memory = memory::Memory::new();
        
    memory.load_rom().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let cpu = cpu::CPU::new();
} 