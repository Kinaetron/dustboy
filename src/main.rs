extern crate sdl2;

mod cpu;
mod memory;

fn main() {

    let memory = memory::RAM::new();
    let cpu = cpu::CPU::new();
} 