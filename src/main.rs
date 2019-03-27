use std::process;

mod cpu;
mod ppu;
mod memory;

fn main() {

    let mut memory = memory::Memory::new();
        
    memory.load_rom().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let mut cpu = cpu::CPU::new(memory);
} 