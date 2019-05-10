use crate::memory::*;

const TIMA : u16 = 0xFF05;
const TMA : u16 = 0xFF06;
const TMC : u16 = 0xFF07;
const CLOCKSPEED : u32 = 4194304;

pub struct Timer {
    time_counter: u32,
    divide_counter: u32,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
          time_counter: 1024,
          divide_counter : 0,
        }
    }

    pub fn update(&mut self, cycles: u32, memory_bus: &mut Memory) {
        self.do_divider_register(cycles, memory_bus);
        
        if self.is_clock_enabled(memory_bus) 
        {
            self.time_counter -= cycles;
            if self.time_counter <= 0 
            {   
                self.time_counter = memory_bus.set_clock_freq();

                let tma_value = memory_bus.read_memory(TMA as usize);

                if  tma_value == 255 {
                    memory_bus.write_memory(TIMA as usize, tma_value);
                }
                else {
                   memory_bus.write_memory(TIMA as usize, 
                    memory_bus.read_memory(TIMA as usize)+1); 
                }
            }
            
        }
    }

    fn is_clock_enabled(&self, memory_bus: &mut Memory) -> bool {
        let value = memory_bus.read_memory(TMC as usize);
        ((value & 0x2) >> 1) != 0 
    }

    fn do_divider_register(&mut self, cycles: u32, memory_bus: &mut Memory) {
         self.divide_counter += cycles;

        if self.divide_counter >= 255 {
            self.divide_counter = 0;
            memory_bus.inc_divider_reg();
        }
    }
}