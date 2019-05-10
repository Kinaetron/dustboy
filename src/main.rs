use std::process;
use std::time::Duration;

mod cpu;
mod ppu;
mod timer;
mod memory;

const MAX_CYCLES : u32 = 69905;

fn main() {

    let mut memory = memory::Memory::new();

    memory.load_rom().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let mut cpu = cpu::CPU::new();
    let mut timer = timer::Timer::new();
    let mut cycles_this_update = 0;

    while cycles_this_update < MAX_CYCLES {
      cpu.execute_opcode(&mut memory);
      cycles_this_update = cpu.get_ticks();
      
      timer.update(cycles_this_update, &mut memory);
    }
} 