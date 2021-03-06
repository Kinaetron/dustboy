use std::process;
use std::time::Duration;

mod cpu;
mod ppu;
mod memory;

fn main() {

    let mut memory = memory::Memory::new();

    memory.load_rom().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let mut cpu = cpu::CPU::new();
    let mut ppu = ppu::PPU::new();

    loop {
        cpu.execute_opcode(&mut memory);
        ppu.render(&mut memory);

        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 2));
    }
} 