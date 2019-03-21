use std::env;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

pub struct Memory {
    ram: [u16; 4096],
    cartridge: [u16; 1048576]
}

impl Memory {
    pub fn new()-> Memory {
        Memory {
            ram: [0; 4096],
            cartridge: [0; 1048576]
        }
    }

    pub fn load_rom(&mut self) -> Result<(), String> {

       let args: Vec<String> = env::args().collect();

        if args.len() <= 1 {
            return  Err("file path to the rom is required".to_string());
        }

        let rom_path = &args[1];
        
        Ok(())
    }
}