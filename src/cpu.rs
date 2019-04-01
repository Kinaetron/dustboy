use crate::memory::*;

enum ProgramCounter {
    Next,
    Skip,
    Jump(u16)
}

impl ProgramCounter {
    fn skip_if(condition: bool) -> ProgramCounter {
        match condition {
            true => ProgramCounter::Skip,
            _ => ProgramCounter::Next
        }
    }
}

pub struct Register {
    value: u16
}

impl Register {
    pub fn new() -> Register {
        Register {
            value: 0
        }
    }

    pub fn set(&mut self, value: u16) {
        self.value = value;
    }

    pub fn get(&self) -> u16 {
        self.value
    }

    pub fn set_left(&mut self, value: u8) {
        self.value = (value as u16) << 8 | self.value;
    }

    pub fn get_left(&self) -> u8 {
        (self.value >> 8) as u8
    }

    pub fn set_right(&mut self, value: u8) {
        self.value = (value as u16) | self.value;
    }

    pub fn get_right(&self) -> u8 {
        self.value as u8
    }
}

pub struct CPU<'a> {
    ticks: u32,
    memory_bus: &'a mut Memory,
    register_af: Register,
    register_bc: Register,
    register_de: Register,
    register_hl: Register,
    stack_pointer:  u16,
    program_counter: u16,
}

impl<'a> CPU<'a> {
    pub fn new(memory_bus: &'a mut Memory) -> CPU<'a> {
        let mut cpu = CPU {
            memory_bus,
            ticks: 0,
            register_af: Register::new(),
            register_bc: Register::new(),
            register_de: Register::new(),
            register_hl: Register::new(),
            stack_pointer: 0xFFFE,
            program_counter: 0x100,
        };
        cpu.register_af.set(0x01B0);
        cpu.register_bc.set(0x0013);
        cpu.register_de.set(0x00D8);
        cpu.register_hl.set(0x014D);
        cpu
    }

    pub fn get_ticks(&mut self) -> u32 {
        let ticks = self.ticks; 
        self.ticks = 0;

        ticks
    }

    pub fn execute_opcode(&mut self) {
        let opcode = self.fetch_opcode();

        let nn = (self.memory_bus.read_memory((self.program_counter + 1) as usize) as u16) << 8 | 
                 (self.memory_bus.read_memory((self.program_counter + 2) as usize) as u16);

           let pc_change = match opcode {
            0x00 => self.opcode_nop(),
            0x40 => self.opcode_load_bb(),
            0x41 => self.opcode_load_bc(),
            0x42 => self.opcode_load_bd(),
            0x43 => self.opcode_load_be(),
            0x44 => self.opcode_load_bh(),
            0x45 => self.opcode_load_bl(),
            0x46 => self.opcode_load_bhl(),
            0x47 => self.opcode_load_ba(),
            0x48 => self.opcode_load_cb(),
            0x49 => self.opcode_load_cc(),
            0x4A => self.opcode_load_cd(),
            0x4B => self.opcode_load_ce(),
            0x4C => self.opcode_load_ch(),
            0x4D => self.opcode_load_cl(),
            0x4E => self.opcode_load_chl(),
            0x4F => self.opcode_load_ca(),
            0x50 => self.opcode_load_db(),
            0x51 => self.opcode_load_dc(),
            0x52 => self.opcode_load_dd(),
            0x53 => self.opcode_load_de(),
            0x54 => self.opcode_load_dh(),
            0x55 => self.opcode_load_dl(),
            0x56 => self.opcode_load_dhl(),
            0x57 => self.opcode_load_da(),
            0x58 => self.opcode_load_eb(),
            0x59 => self.opcode_load_ec(),
            0x5A => self.opcode_load_ed(),
            0x5B => self.opcode_load_ee(),
            0x5C => self.opcode_load_eh(),
            0x5D => self.opcode_load_el(),
            0x5E => self.opcode_load_ehl(),
            0x5F => self.opcode_load_ea(),
            0x60 => self.opcode_load_hb(),
            0x61 => self.opcode_load_hc(),
            0x62 => self.opcode_load_hd(),
            0x63 => self.opcode_load_he(),
            0x64 => self.opcode_load_hh(),
            0x65 => self.opcode_load_hl(),
            0x66 => self.opcode_load_hhl(),
            0x67 => self.opcode_load_ha(),
            0x68 => self.opcode_load_lb(),
            0x69 => self.opcode_load_lc(),
            0x6A => self.opcode_load_ld(),
            0x6B => self.opcode_load_le(),
            0x6C => self.opcode_load_lh(),
            0x6D => self.opcode_load_ll(),
            0x6E => self.opcode_load_lhl(),
            0x6F => self.opcode_load_la(),
            0x70 => self.opcode_load_hlb(),
            0x71 => self.opcode_load_hlc(),
            0x72 => self.opcode_load_hld(),
            0x73 => self.opcode_load_hle(),
            0x74 => self.opcode_load_hlh(),
            0x75 => self.opcode_load_hll(),
            0x77 => self.opcode_load_hla(),
            0x78 => self.opcode_load_ab(),
            0x79 => self.opcode_load_ac(),
            0x7A => self.opcode_load_ad(),
            0x7B => self.opcode_load_ae(),
            0x7C => self.opcode_load_ah(),
            0x7D => self.opcode_load_al(),
            0x7E => self.opcode_load_ahl(),
            0x7F => self.opcode_load_aa(),
            0xC3 => self.opcode_jmp(nn),

            _ => panic!("Opcode {:X} isn't implemented", opcode)
        };

         match pc_change {
            ProgramCounter::Next => self.program_counter += 1,
            ProgramCounter::Skip => self.program_counter += 2 ,
            ProgramCounter::Jump(address) => self.program_counter = address
        }
    }


    fn fetch_opcode(&mut self) -> u8 {
        self.memory_bus.read_memory(self.program_counter as usize)
    }

    fn opcode_load_bb(&mut self) -> ProgramCounter {
        self.register_bc.set_left(self.register_bc.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_bc(&mut self) -> ProgramCounter {
        self.register_bc.set_left(self.register_bc.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_bd(&mut self) -> ProgramCounter {
        self.register_bc.set_left(self.register_de.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_be(&mut self) -> ProgramCounter {
        self.register_bc.set_left(self.register_de.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_bh(&mut self) -> ProgramCounter {
        self.register_bc.set_left(self.register_hl.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_bl(&mut self) -> ProgramCounter {
        self.register_bc.set_left(self.register_hl.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_bhl(&mut self) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        self.register_bc.set_left(self.memory_bus.read_memory(addr));
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_ba(&mut self) -> ProgramCounter {
        self.register_bc.set_left(self.register_af.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }



    fn opcode_load_cb(&mut self) -> ProgramCounter {
        self.register_bc.set_right(self.register_bc.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_cc(&mut self) -> ProgramCounter {
        self.register_bc.set_right(self.register_bc.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_cd(&mut self) -> ProgramCounter {
        self.register_bc.set_right(self.register_de.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_ce(&mut self) -> ProgramCounter {
        self.register_bc.set_right(self.register_de.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_ch(&mut self) -> ProgramCounter {
        self.register_bc.set_right(self.register_hl.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_cl(&mut self) -> ProgramCounter {
        self.register_bc.set_right(self.register_hl.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_chl(&mut self) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        self.register_bc.set_right(self.memory_bus.read_memory(addr));
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_ca(&mut self) -> ProgramCounter {
        self.register_bc.set_right(self.register_af.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }



    fn opcode_load_db(&mut self) -> ProgramCounter {
        self.register_de.set_left(self.register_bc.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_dc(&mut self) -> ProgramCounter {
        self.register_de.set_left(self.register_bc.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_dd(&mut self) -> ProgramCounter {
        self.register_de.set_left(self.register_de.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_de(&mut self) -> ProgramCounter {
        self.register_de.set_left(self.register_de.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_dh(&mut self) -> ProgramCounter {
        self.register_de.set_left(self.register_hl.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_dl(&mut self) -> ProgramCounter {
        self.register_de.set_left(self.register_hl.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_dhl(&mut self) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        self.register_de.set_left(self.memory_bus.read_memory(addr));
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_da(&mut self) -> ProgramCounter {
        self.register_de.set_left(self.register_af.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }



    fn opcode_load_eb(&mut self) -> ProgramCounter {
        self.register_de.set_right(self.register_bc.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_ec(&mut self) -> ProgramCounter {
        self.register_de.set_right(self.register_bc.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_ed(&mut self) -> ProgramCounter {
        self.register_de.set_right(self.register_de.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_ee(&mut self) -> ProgramCounter {
        self.register_de.set_right(self.register_de.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_eh(&mut self) -> ProgramCounter {
        self.register_de.set_right(self.register_hl.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_el(&mut self) -> ProgramCounter {
        self.register_de.set_right(self.register_hl.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_ehl(&mut self) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        self.register_de.set_right(self.memory_bus.read_memory(addr));
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_ea(&mut self) -> ProgramCounter {
        self.register_de.set_right(self.register_af.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }



    fn opcode_load_hb(&mut self) -> ProgramCounter {
        self.register_hl.set_left(self.register_bc.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_hc(&mut self) -> ProgramCounter {
        self.register_hl.set_left(self.register_bc.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_hd(&mut self) -> ProgramCounter {
        self.register_hl.set_left(self.register_de.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_he(&mut self) -> ProgramCounter {
        self.register_hl.set_left(self.register_de.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_hh(&mut self) -> ProgramCounter {
        self.register_hl.set_left(self.register_hl.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_hl(&mut self) -> ProgramCounter {
        self.register_hl.set_left(self.register_hl.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_hhl(&mut self) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        self.register_hl.set_left(self.memory_bus.read_memory(addr));
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_ha(&mut self) -> ProgramCounter {
        self.register_hl.set_left(self.register_af.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }



    fn opcode_load_lb(&mut self) -> ProgramCounter {
        self.register_hl.set_right(self.register_bc.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_lc(&mut self) -> ProgramCounter {
        self.register_hl.set_right(self.register_bc.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_ld(&mut self) -> ProgramCounter {
        self.register_hl.set_right(self.register_de.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_le(&mut self) -> ProgramCounter {
        self.register_hl.set_right(self.register_de.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_lh(&mut self) -> ProgramCounter {
        self.register_hl.set_right(self.register_hl.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_ll(&mut self) -> ProgramCounter {
        self.register_hl.set_right(self.register_hl.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_lhl(&mut self) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        self.register_hl.set_right(self.memory_bus.read_memory(addr));
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_la(&mut self) -> ProgramCounter {
        self.register_hl.set_right(self.register_af.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }



    fn opcode_load_hlb(&mut self) -> ProgramCounter {

        let addr = self.register_hl.get() as usize;

        self.memory_bus.write_memory(addr, self.register_bc.get_left());
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_hlc(&mut self) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;
        
        self.memory_bus.write_memory(addr, self.register_bc.get_right());
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_hld(&mut self) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        self.memory_bus.write_memory(addr, self.register_de.get_left());
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_hle(&mut self) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        self.memory_bus.write_memory(addr, self.register_de.get_right());
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_hlh(&mut self) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        self.memory_bus.write_memory(addr, self.register_hl.get_left());
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_hll(&mut self) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        self.memory_bus.write_memory(addr, self.register_hl.get_right());
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_hla(&mut self) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        self.memory_bus.write_memory(addr, self.register_af.get_left());
        self.ticks += 8;

        ProgramCounter::Next
    }



    fn opcode_load_ab(&mut self) -> ProgramCounter {
        self.register_af.set_left(self.register_bc.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_ac(&mut self) -> ProgramCounter {
        self.register_af.set_left(self.register_bc.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_ad(&mut self) -> ProgramCounter {
        self.register_af.set_left(self.register_de.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_ae(&mut self) -> ProgramCounter {
        self.register_af.set_left(self.register_de.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_ah(&mut self) -> ProgramCounter {
        self.register_af.set_left(self.register_hl.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_al(&mut self) -> ProgramCounter {
        self.register_af.set_left(self.register_hl.get_right());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_ahl(&mut self) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        self.register_af.set_left(self.memory_bus.read_memory(addr));
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_aa(&mut self) -> ProgramCounter {
        self.register_af.set_left(self.register_af.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }



    
    fn opcode_nop(&mut self) -> ProgramCounter {
        self.ticks += 4;

        ProgramCounter::Next
    }



    fn opcode_jmp(&mut self, address: u16) -> ProgramCounter {  
        self.ticks += 12;

        ProgramCounter::Jump(address)
    }
}