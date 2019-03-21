extern crate sdl2;

mod cpu;
mod memory;

fn main() {

    let memory = memory::Memory::new();
    let cpu = cpu::CPU::new();
} 