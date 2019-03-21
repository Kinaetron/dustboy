use std::env;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

pub struct Memory {
    ram: [u16; 4096],
    cartridge: Vec<u8>
}

impl Memory {
    pub fn new()-> Memory {
        Memory {
            ram: [0; 4096],
            cartridge: Vec::new()
        }
    }

    pub fn load_rom(&mut self) -> Result<(), String> {

       let args: Vec<String> = env::args().collect();

        if args.len() <= 1 {
            return  Err("file path to the rom is required".to_string());
        }

        let rom_path = &args[1];

        let mut rom = File::open(rom_path).map_err(|e| e.description().to_string())?;
        rom.read_to_end(&mut self.cartridge).map_err(|e| e.description().to_string())?;
        self.cartridge.shrink_to_fit();

        Ok(())
    }
}