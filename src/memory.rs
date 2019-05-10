use std::env;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

const TMC : usize = 0xFF07;
const DIV_REG : usize = 0xFF04;

enum MemoryBankMode {
    Mode0,
    Mode1,
    Mode2
}

pub struct Memory {
    ram: [u8; 0x10000],
    cartridge: Vec<u8>,
    stack_pointer:  u16,
    bank_mode: MemoryBankMode
}

impl Memory {
    pub fn new()-> Memory {
        let mut memory = Memory {
            ram: [0; 0x10000],
            cartridge: Vec::new(),
            stack_pointer: 0,
            bank_mode: MemoryBankMode::Mode0,
        };
        memory
    }

    pub fn load_rom(&mut self) -> Result<(), String> {

        let rom_path = "roms/boot-rom.gb";

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

        if addr < 0x8000 { }
        else if (addr >= 0xE000) && (addr < 0xFE00) 
        {
            self.ram[addr] = data;
            self.write_memory(addr - 0x2000, data);
        }
        else if (addr >= 0xFEA0) && (addr < 0xFEFF) { }
        else if addr == 0xFF07 
        {
            let currentfreq = self.get_clock_freq();
            self.ram[addr] = data;
            let newfreq = self.get_clock_freq();

            if currentfreq != newfreq {
                self.set_clock_freq();
            }
        }
        else if addr == DIV_REG {
            self.ram[addr] = 0;
        }
        else {
            self.ram[addr] = data;
        }
    }

    pub fn inc_divider_reg(&mut self) {
        self.ram[DIV_REG].wrapping_add(1);
    }

    pub fn get_clock_freq(&self) -> u8 {
        self.read_memory(TMC) & 0x3
    }

    pub fn set_clock_freq(&self) -> u32 {
        let freq = self.get_clock_freq();

        let value = match freq 
        {
            0 => 1024,
            1 => 16,
            2 => 64,
            3 => 256,
            _ => panic!("frequency not found")
        };
        value
    }

    pub fn set_stack_pointer(&mut self, stack_pointer: u16) {
        self.stack_pointer = stack_pointer;
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