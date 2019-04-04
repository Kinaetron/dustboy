use std::env;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

pub struct Memory {
    ram: [u8; 0x10000],
    cartridge: Vec<u8>,
    stack_pointer:  u16,
}

impl Memory {
    pub fn new()-> Memory {
        let mut memory = Memory {
            ram: [0; 0x10000],
            cartridge: Vec::new(),
            stack_pointer: 0xFFFE,
        };

        memory.ram[0xFF10] = 0x80;
        memory.ram[0xFF11] = 0xBF;
        memory.ram[0xFF10] = 0x80;
        memory.ram[0xFF11] = 0xBF;
        memory.ram[0xFF12] = 0xF3;
        memory.ram[0xFF14] = 0xBF;
        memory.ram[0xFF16] = 0x3F;
        memory.ram[0xFF19] = 0xBF;
        memory.ram[0xFF1A] = 0x7F;
        memory.ram[0xFF1B] = 0xFF;
        memory.ram[0xFF1C] = 0x9F;
        memory.ram[0xFF1E] = 0xBF;
        memory.ram[0xFF20] = 0xFF;
        memory.ram[0xFF23] = 0xBF;
        memory.ram[0xFF24] = 0x77;
        memory.ram[0xFF25] = 0xF3;
        memory.ram[0xFF26] = 0xF1;
        memory.ram[0xFF40] = 0x91;
        memory.ram[0xFF47] = 0xFC;
        memory.ram[0xFF48] = 0xFF;
        memory.ram[0xFF49] = 0xFF;
        memory
    }

    pub fn load_rom(&mut self) -> Result<(), String> {

        let rom_path = "roms/06-ld r,r.gb";

        let mut rom = File::open(rom_path).map_err(|e| e.description().to_string())?;
        rom.read_to_end(&mut self.cartridge).map_err(|e| e.description().to_string())?;

       for (i, byte) in self.cartridge.bytes().enumerate() {
            self.ram[i] =  byte.map_err(|e| e.description().to_string())?;
        }
        Ok(())
    }

    pub fn read_memory(&self, addr: usize) -> u8 {
        self.ram[addr]
    }

    pub fn write_memory(&mut self, addr: usize, data: u8) {
        self.ram[addr] = data;

        if addr == 0xFF01 {
            print!("{:X}", data);
        }
    }

    pub fn set_stack_point(&mut self, stack_pointer: u16) {
        self.stack_pointer = stack_pointer
    }

    pub fn get_stack_pointer(&self) -> u16 {
        self.stack_pointer
    }

    pub fn push_16(&mut self, value: u16) {
        let byte_one = ((value >> 8) & 0xFF) as u8;
        let byte_two = (value & 0xFF) as u8;

        self.stack_pointer -= 1;
        self.ram[self.stack_pointer as usize] = byte_one;

        self.stack_pointer -= 1;
        self.ram[self.stack_pointer as usize] = byte_two;
    }

    pub fn pop_16(&mut self) -> u16 {

        let byte_one = self.ram[self.stack_pointer as usize];
        self.stack_pointer += 1;

        let byte_two = self.ram[self.stack_pointer as usize];
        self.stack_pointer += 1;

        (byte_two as u16) << 8 | (byte_one as u16)
    }
}