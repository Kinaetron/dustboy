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

pub struct CPU {
    pub ticks: u32,
    pub memory: [u8; 0x10000],
    register_AF: Register,
    register_BC: Register,
    register_DE: Register,
    register_HL: Register,
    stack_pointer:  u16,
    program_counter: u16,
}

impl CPU {
    pub fn new(memory: [u8; 0x10000]) -> CPU {
        let mut cpu = CPU {
            memory,
            ticks: 0,
            register_AF: Register::new(),
            register_BC: Register::new(),
            register_DE: Register::new(),
            register_HL: Register::new(),
            stack_pointer: 0xFFFE,
            program_counter: 0x100,
        };
        cpu.register_AF.set(0x01B0);
        cpu.register_BC.set(0x0013);
        cpu.register_DE.set(0x00D8);
        cpu.register_HL.set(0x014D);
        cpu
    }

    pub fn opcode_execute(&mut self) {
        let opcode = self.opcode_fetch();
        self.program_counter_inc();

        match opcode {
            0x40 => self.opcode_load_BB(),
            0x41 => self.opcode_load_BC(),
            0x42 => self.opcode_load_BD(),
            0x43 => self.opcode_load_BE(),
            0x44 => self.opcode_load_BH(),
            0x45 => self.opcode_load_BL(),
            0x46 => self.opcode_load_BHL(),
            0x47 => self.opcode_load_BA(),
            0x48 => self.opcode_load_CB(),
            0x49 => self.opcode_load_CC(),
            0x4A => self.opcode_load_CD(),
            0x4B => self.opcode_load_CE(),
            0x4C => self.opcode_load_CH(),
            0x4D => self.opcode_load_CL(),
            0x4E => self.opcode_load_CHL(),
            0x4F => self.opcode_load_CA(),
            0x50 => self.opcode_load_DB(),
            0x51 => self.opcode_load_DC(),
            0x52 => self.opcode_load_DD(),
            0x53 => self.opcode_load_DE(),
            0x54 => self.opcode_load_DH(),
            0x55 => self.opcode_load_DL(),
            0x56 => self.opcode_load_DHL(),
            0x57 => self.opcode_load_DA(),
            0x58 => self.opcode_load_EB(),
            0x59 => self.opcode_load_EC(),
            0x5A => self.opcode_load_ED(),
            0x5B => self.opcode_load_EE(),
            0x5C => self.opcode_load_EH(),
            0x5D => self.opcode_load_EL(),
            0x5E => self.opcode_load_EHL(),
            0x5F => self.opcode_load_EA(),
            0x60 => self.opcode_load_HB(),
            0x61 => self.opcode_load_HC(),
            0x62 => self.opcode_load_HD(),
            0x63 => self.opcode_load_HE(),
            0x64 => self.opcode_load_HH(),
            0x65 => self.opcode_load_HL(),
            0x66 => self.opcode_load_HHL(),
            0x67 => self.opcode_load_HA(),
            0x68 => self.opcode_load_LB(),
            0x69 => self.opcode_load_LC(),
            0x6A => self.opcode_load_LD(),
            0x6B => self.opcode_load_LE(),
            0x6C => self.opcode_load_LH(),
            0x6D => self.opcode_load_LL(),
            0x6E => self.opcode_load_LHL(),
            0x6F => self.opcode_load_LA(),
            0x70 => self.opcode_load_HLB(),
            0x71 => self.opcode_load_HLC(),
            0x72 => self.opcode_load_HLD(),
            0x73 => self.opcode_load_HLE(),
            0x74 => self.opcode_load_HLH(),
            0x75 => self.opcode_load_HLL(),
            0x77 => self.opcode_load_HLA(),
            0x78 => self.opcode_load_AB(),
            0x79 => self.opcode_load_AC(),
            0x7A => self.opcode_load_AD(),
            0x7B => self.opcode_load_AE(),
            0x7C => self.opcode_load_AH(),
            0x7D => self.opcode_load_AL(),
            0x7E => self.opcode_load_AHL(),
            0x7F => self.opcode_load_AA(),

            _ => self.program_counter_inc()
        }
    }

    fn program_counter_inc(&mut self) {
        self.program_counter += 1;
    }

    fn opcode_fetch(&mut self) -> u8 {
        self.memory[self.program_counter as usize]
    }

    fn opcode_load_BB(&mut self) {
        self.register_BC.set_left(self.register_BC.get_left());
        self.ticks += 4;
    }

    fn opcode_load_BC(&mut self) {
        self.register_BC.set_left(self.register_BC.get_right());
        self.ticks += 4;
    }

    fn opcode_load_BD(&mut self) {
        self.register_BC.set_left(self.register_DE.get_left());
        self.ticks += 4;
    }

    fn opcode_load_BE(&mut self) {
        self.register_BC.set_left(self.register_DE.get_right());
        self.ticks += 4;
    }

    fn opcode_load_BH(&mut self) {
        self.register_BC.set_left(self.register_HL.get_left());
        self.ticks += 4;
    }

    fn opcode_load_BL(&mut self) {
        self.register_BC.set_left(self.register_HL.get_right());
        self.ticks += 4;
    }

    fn opcode_load_BHL(&mut self) {
        self.register_BC.set_left(self.memory[self.register_HL.get() as usize]);
        self.ticks += 8;
    }

    fn opcode_load_BA(&mut self) {
        self.register_BC.set_left(self.register_AF.get_left());
        self.ticks += 4;
    }



    fn opcode_load_CB(&mut self) {
        self.register_BC.set_right(self.register_BC.get_left());
        self.ticks += 4;
    }

    fn opcode_load_CC(&mut self) {
        self.register_BC.set_right(self.register_BC.get_right());
        self.ticks += 4;
    }

    fn opcode_load_CD(&mut self) {
        self.register_BC.set_right(self.register_DE.get_left());
        self.ticks += 4;
    }

    fn opcode_load_CE(&mut self) {
        self.register_BC.set_right(self.register_DE.get_right());
        self.ticks += 4;
    }

    fn opcode_load_CH(&mut self) {
        self.register_BC.set_right(self.register_HL.get_left());
        self.ticks += 4;
    }

    fn opcode_load_CL(&mut self) {
        self.register_BC.set_right(self.register_HL.get_right());
        self.ticks += 4;
    }

    fn opcode_load_CHL(&mut self) {
        self.register_BC.set_right(self.memory[self.register_HL.get() as usize]);
        self.ticks += 8;
    }

    fn opcode_load_CA(&mut self) {
        self.register_BC.set_right(self.register_AF.get_left());
        self.ticks += 4;
    }



    fn opcode_load_DB(&mut self) {
        self.register_DE.set_left(self.register_BC.get_left());
        self.ticks += 4;
    }

    fn opcode_load_DC(&mut self) {
        self.register_DE.set_left(self.register_BC.get_right());
        self.ticks += 4;
    }

    fn opcode_load_DD(&mut self) {
        self.register_DE.set_left(self.register_DE.get_left());
        self.ticks += 4;
    }

    fn opcode_load_DE(&mut self) {
        self.register_DE.set_left(self.register_DE.get_right());
        self.ticks += 4;
    }

    fn opcode_load_DH(&mut self) {
        self.register_DE.set_left(self.register_HL.get_left());
        self.ticks += 4;
    }

    fn opcode_load_DL(&mut self) {
        self.register_DE.set_left(self.register_HL.get_right());
        self.ticks += 4;
    }

    fn opcode_load_DHL(&mut self) {
        self.register_DE.set_left(self.memory[self.register_HL.get() as usize]);
        self.ticks += 8;
    }

    fn opcode_load_DA(&mut self) {
        self.register_DE.set_left(self.register_AF.get_left());
        self.ticks += 4;
    }



    fn opcode_load_EB(&mut self) {
        self.register_DE.set_right(self.register_BC.get_left());
        self.ticks += 4;
    }

    fn opcode_load_EC(&mut self) {
        self.register_DE.set_right(self.register_BC.get_right());
        self.ticks += 4;
    }

    fn opcode_load_ED(&mut self) {
        self.register_DE.set_right(self.register_DE.get_left());
        self.ticks += 4;
    }

    fn opcode_load_EE(&mut self) {
        self.register_DE.set_right(self.register_DE.get_right());
        self.ticks += 4;
    }

    fn opcode_load_EH(&mut self) {
        self.register_DE.set_right(self.register_HL.get_left());
        self.ticks += 4;
    }

    fn opcode_load_EL(&mut self) {
        self.register_DE.set_right(self.register_HL.get_right());
        self.ticks += 4;
    }

    fn opcode_load_EHL(&mut self) {
        self.register_DE.set_right(self.memory[self.register_HL.get() as usize]);
        self.ticks += 8;
    }

    fn opcode_load_EA(&mut self) {
        self.register_DE.set_right(self.register_AF.get_left());
        self.ticks += 4;
    }



    fn opcode_load_HB(&mut self) {
        self.register_HL.set_left(self.register_BC.get_left());
        self.ticks += 4;
    }

    fn opcode_load_HC(&mut self) {
        self.register_HL.set_left(self.register_BC.get_right());
        self.ticks += 4;
    }

    fn opcode_load_HD(&mut self) {
        self.register_HL.set_left(self.register_DE.get_left());
        self.ticks += 4;
    }

    fn opcode_load_HE(&mut self) {
        self.register_HL.set_left(self.register_DE.get_right());
        self.ticks += 4;
    }

    fn opcode_load_HH(&mut self) {
        self.register_HL.set_left(self.register_HL.get_left());
        self.ticks += 4;
    }

    fn opcode_load_HL(&mut self) {
        self.register_HL.set_left(self.register_HL.get_right());
        self.ticks += 4;
    }

    fn opcode_load_HHL(&mut self) {
        self.register_HL.set_left(self.memory[self.register_HL.get() as usize]);
        self.ticks += 8;
    }

    fn opcode_load_HA(&mut self) {
        self.register_HL.set_left(self.register_AF.get_left());
        self.ticks += 4;
    }



    fn opcode_load_LB(&mut self) {
        self.register_HL.set_right(self.register_BC.get_left());
        self.ticks += 4;
    }

    fn opcode_load_LC(&mut self) {
        self.register_HL.set_right(self.register_BC.get_right());
        self.ticks += 4;
    }

    fn opcode_load_LD(&mut self) {
        self.register_HL.set_right(self.register_DE.get_left());
        self.ticks += 4;
    }

    fn opcode_load_LE(&mut self) {
        self.register_HL.set_right(self.register_DE.get_right());
        self.ticks += 4;
    }

    fn opcode_load_LH(&mut self) {
        self.register_HL.set_right(self.register_HL.get_left());
        self.ticks += 4;
    }

    fn opcode_load_LL(&mut self) {
        self.register_HL.set_right(self.register_HL.get_right());
        self.ticks += 4;
    }

    fn opcode_load_LHL(&mut self) {
        self.register_HL.set_right(self.memory[self.register_HL.get() as usize]);
        self.ticks += 8;
    }

    fn opcode_load_LA(&mut self) {
        self.register_HL.set_right(self.register_AF.get_left());
        self.ticks += 4;
    }



    fn opcode_load_HLB(&mut self) {
        self.memory[self.register_HL.get() as usize] = self.register_BC.get_left();
        self.ticks += 8;
    }

    fn opcode_load_HLC(&mut self) {
        self.memory[self.register_HL.get() as usize] = self.register_BC.get_right();
        self.ticks += 8;
    }

    fn opcode_load_HLD(&mut self) {
        self.memory[self.register_HL.get() as usize] = self.register_DE.get_left();
        self.ticks += 8;
    }

    fn opcode_load_HLE(&mut self) {
        self.memory[self.register_HL.get() as usize] = self.register_DE.get_right();
        self.ticks += 8;
    }

    fn opcode_load_HLH(&mut self) {
        self.memory[self.register_HL.get() as usize] = self.register_HL.get_left();
        self.ticks += 8;
    }

    fn opcode_load_HLL(&mut self) {
        self.memory[self.register_HL.get() as usize] = self.register_HL.get_right();
        self.ticks += 8;
    }

    fn opcode_load_HLA(&mut self) {
        self.memory[self.register_HL.get() as usize] = self.register_AF.get_left();
        self.ticks += 8;
    }



    fn opcode_load_AB(&mut self) {
        self.register_AF.set_left(self.register_BC.get_left());
        self.ticks += 4;
    }

    fn opcode_load_AC(&mut self) {
        self.register_AF.set_left(self.register_BC.get_right());
        self.ticks += 4;
    }

    fn opcode_load_AD(&mut self) {
        self.register_AF.set_left(self.register_DE.get_left());
        self.ticks += 4;
    }

    fn opcode_load_AE(&mut self) {
        self.register_AF.set_left(self.register_DE.get_right());
        self.ticks += 4;
    }

    fn opcode_load_AH(&mut self) {
        self.register_AF.set_left(self.register_HL.get_left());
        self.ticks += 4;
    }

    fn opcode_load_AL(&mut self) {
        self.register_AF.set_left(self.register_HL.get_right());
        self.ticks += 4;
    }

    fn opcode_load_AHL(&mut self) {
        self.register_AF.set_left(self.memory[self.register_HL.get() as usize]);
        self.ticks += 8;
    }

    fn opcode_load_AA(&mut self) {
        self.register_AF.set_left(self.register_AF.get_left());
        self.ticks += 4;
    }
}