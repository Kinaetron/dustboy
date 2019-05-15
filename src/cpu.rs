use std::u8;
use crate::memory::*;

const Z_FLAG : u8 = 7;
const N_FLAG : u8 = 6;
const H_FLAG : u8 = 5;
const C_FLAG : u8 = 4;

const Z_FLAG_BIT : u8 = 0x80;
const N_FLAG_BIT : u8 = 0x40;
const H_FLAG_BIT : u8 = 0x20;
const C_FLAG_BIT : u8 = 0x10;

fn set_bit(value: u8, offset: u8) -> u8 {
    let ret_val = (1 << offset) | value;
    ret_val
}

fn clear_bit(value: u8, offset: u8) -> u8 {
    let ret_val = !(1 << offset) & value;
    ret_val
}

fn get_bit(value: u8, offset: u8, bit_value: u8) -> bool {
    let ret_val = (value & bit_value) >> offset;
    ret_val != 0
}

fn set_bool_bit(value: u8, offset: u8, data: bool) -> u8 {
    let mut ret_val = 0;

    if data == true {
        ret_val =  (1 << offset) | value;
    }
    else {
        ret_val = !(1 << offset) & value;
    }
    ret_val
}

enum ProgramCounter {
    Next,
    Skip,
    Skip2,
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
        self.value = (value as u16) << 8 | (self.value & 0x00FF);
    }

    pub fn get_left(&self) -> u8 {
        (self.value >> 8) as u8
    }

    pub fn set_right(&mut self, value: u8) {
        self.value = (self.value & 0xFF00) | (value as u16);
    }

    pub fn get_right(&self) -> u8 {
        self.value as u8
    }

    pub fn set_bit_right(&mut self, offset: u8) {
         self.set_right(set_bit(self.get_right(), offset));
    }

    pub fn clear_bit_right(&mut self, offset: u8) {
         self.set_right(clear_bit(self.get_right(), offset));
    }

    pub fn get_bit_right(&self, offset: u8, bit_value: u8) -> bool {
        get_bit(self.get_right(), offset, bit_value)
    }

    pub fn set_bool_bit_right(&mut self, offset: u8, data: bool) {
        self.set_right(set_bool_bit(self.get_right(), offset, data));
    }

    pub fn set_bit_left(&mut self, offset: u8) {
        self.set_left(set_bit(self.get_left(), offset));
    }

    pub fn clear_bit_left(&mut self, offset: u8) {
          self.set_left(clear_bit(self.get_left(), offset));
    }

    pub fn get_bit_left(&self, offset: u8, bit_value: u8) -> bool {
        get_bit(self.get_left(), offset, bit_value)
    }

    pub fn set_bool_bit_left(&mut self, offset: u8, data: bool) {
        self.set_left(set_bool_bit(self.get_left(), offset, data));
    }

    pub fn inc(&mut self) -> u16 {
        self.value += 1;

        self.value
    }

    pub fn dec(&mut self) -> u16 {
        self.value -= 1;

        self.value
    }

    pub fn inc_left(&mut self) -> u8 {
        self.set_left(self.get_left() + 1);

        self.get_left()
    }

    pub fn dec_left(&mut self) -> u8 {
        self.set_left(self.get_left() - 1);

        self.get_left()
    }

    pub fn inc_right(&mut self) -> u8 {
         self.set_right(self.get_right() + 1);

         self.get_right()
    }

    pub fn dec_right(&mut self) -> u8 {
         self.set_right(self.get_right() - 1);

         self.get_right()
    }
}

pub struct CPU {
    ticks: u32,
    register_af: Register,
    register_bc: Register,
    register_de: Register,
    register_hl: Register,
    program_counter: u16,
}

impl CPU {
    pub fn new() -> CPU {
        let mut cpu = CPU {
            ticks: 0,
            register_af: Register::new(),
            register_bc: Register::new(),
            register_de: Register::new(),
            register_hl: Register::new(),
            program_counter: 0,
        };
        cpu
    }

    pub fn get_ticks(&mut self) -> u32 {
        let ticks = self.ticks; 
        self.ticks = 0;

        ticks
    }

    pub fn set_program_counter(&mut self, pc: u16) {
        self.program_counter = pc
    }

    pub fn execute_opcode(&mut self,  memory_bus: &mut Memory) {
        let opcode = self.fetch_opcode(memory_bus);

        println!("Program Counter {:X}", self.program_counter);
        println!("Opcode {:X}", opcode);

        let n = memory_bus.read_memory((self.program_counter + 1)as usize);

        let nn = (memory_bus.read_memory((self.program_counter + 2) as usize) as u16) << 8 | 
                 (memory_bus.read_memory((self.program_counter + 1) as usize) as u16);

           let pc_change = match opcode {
            0x00 => self.opcode_nop(),
            0x04 => self.opcode_inc_b(),
            0x05 => self.opcode_dec_b(),
            0x06 => self.opcode_load_bn(n),
            0x0C => self.opcode_inc_c(),
            0x0D => self.opcode_dec_c(),
            0x0E => self.opcode_load_cn(n),
            0x11 => self.opcode_load_de_16(nn),
            0x12 => self.opcode_load_de_a(memory_bus),
            0x13 => self.opcode_dec_de(),
            0x17 => self.opcode_roate_a_left(),
            0x18 => self.opcode_jr(n as i8),
            0x1C => self.opcode_inc_e(),
            0x1A => self.opcode_load_a_de(memory_bus),
            0x1E => self.opcode_load_e_n(n),
            0x20 => self.opcode_jmp_nz(n as i8),
            0x21 => self.opcode_load_hl_16(nn),
            0x22 => self.opcode_load_a_hl_inc(memory_bus),
            0x23 => self.opcode_inc_hl(),
            0x28 => self.opcode_jr_nz(n as i8),
            0x2A => self.opcode_load_hl_a_inc(memory_bus),
            0x2E => self.opcode_load_l_n(n),
            0x31 => self.opcode_load_sp_16(nn, memory_bus),
            0x32 => self.opcode_load_hl_a_dec(memory_bus),
            0x3D => self.opcode_dec_a(),
            0x3E => self.opcode_load_n_a(n),
            0x40 => self.opcode_load_bb(),
            0x41 => self.opcode_load_bc(),
            0x42 => self.opcode_load_bd(),
            0x43 => self.opcode_load_be(),
            0x44 => self.opcode_load_bh(),
            0x45 => self.opcode_load_bl(),
            0x46 => self.opcode_load_bhl(memory_bus),
            0x47 => self.opcode_load_ba(),
            0x48 => self.opcode_load_cb(),
            0x49 => self.opcode_load_cc(),
            0x4A => self.opcode_load_cd(),
            0x4B => self.opcode_load_ce(),
            0x4C => self.opcode_load_ch(),
            0x4D => self.opcode_load_cl(),
            0x4E => self.opcode_load_chl(memory_bus),
            0x4F => self.opcode_load_ca(),
            0x50 => self.opcode_load_db(),
            0x51 => self.opcode_load_dc(),
            0x52 => self.opcode_load_dd(),
            0x53 => self.opcode_load_de(),
            0x54 => self.opcode_load_dh(),
            0x55 => self.opcode_load_dl(),
            0x56 => self.opcode_load_dhl(memory_bus),
            0x57 => self.opcode_load_da(),
            0x58 => self.opcode_load_eb(),
            0x59 => self.opcode_load_ec(),
            0x5A => self.opcode_load_ed(),
            0x5B => self.opcode_load_ee(),
            0x5C => self.opcode_load_eh(),
            0x5D => self.opcode_load_el(),
            0x5E => self.opcode_load_ehl(memory_bus),
            0x5F => self.opcode_load_ea(),
            0x60 => self.opcode_load_hb(),
            0x61 => self.opcode_load_hc(),
            0x62 => self.opcode_load_hd(),
            0x63 => self.opcode_load_he(),
            0x64 => self.opcode_load_hh(),
            0x65 => self.opcode_load_hl(),
            0x66 => self.opcode_load_hhl(memory_bus),
            0x67 => self.opcode_load_ha(),
            0x68 => self.opcode_load_lb(),
            0x69 => self.opcode_load_lc(),
            0x6A => self.opcode_load_ld(),
            0x6B => self.opcode_load_le(),
            0x6C => self.opcode_load_lh(),
            0x6D => self.opcode_load_ll(),
            0x6E => self.opcode_load_lhl(memory_bus),
            0x6F => self.opcode_load_la(),
            0x70 => self.opcode_load_hlb(memory_bus),
            0x71 => self.opcode_load_hlc(memory_bus),
            0x72 => self.opcode_load_hld(memory_bus),
            0x73 => self.opcode_load_hle(memory_bus),
            0x74 => self.opcode_load_hlh(memory_bus),
            0x75 => self.opcode_load_hll(memory_bus),
            0x77 => self.opcode_load_hla(memory_bus),
            0x78 => self.opcode_load_ab(),
            0x79 => self.opcode_load_ac(),
            0x7A => self.opcode_load_ad(),
            0x7B => self.opcode_load_ae(),
            0x7C => self.opcode_load_ah(),
            0x7D => self.opcode_load_al(),
            0x7E => self.opcode_load_ahl(memory_bus),
            0x7F => self.opcode_load_aa(),
            0xAF => self.opcode_xor_aa(),
            0xC0 => self.opcode_rtn_nz(memory_bus),
            0xC1 => self.opcode_pop_bc(memory_bus),
            0xC3 => self.opcode_jmp(nn),
            0xC5 => self.opcode_push_bc(memory_bus),
            0xC9 => self.opcode_return(memory_bus),
            0xCB => self.cb_opcodes(memory_bus),
            0xCD => self.opcode_call(nn, memory_bus),
            0xE0 => self.opcode_load_a_ff00_plus_n(n, memory_bus),
            0xEA => self.opcode_load_a_nn(nn, memory_bus),
            0xE2 => self.opcode_load_a_ff00_plus_c(memory_bus),
            0xF0 => self.opcode_load_a_mem_ff00_plus_n(n, memory_bus),
            0xFE => self.opcode_com_a_n(n),

            _ => panic!("Opcode {:X} isn't implemented", opcode)
        };

         match pc_change {
            ProgramCounter::Next => self.program_counter += 1,
            ProgramCounter::Skip => self.program_counter += 2,
            ProgramCounter::Skip2 => self.program_counter += 3,
            ProgramCounter::Jump(address) => self.program_counter = address
        }
    }


    fn pc_inc_next(&mut self) -> u16 {
        self.program_counter += 1;
        self.program_counter
    }

     fn pc_inc_skip_1(&mut self) -> u16 {
        self.program_counter += 2;
        self.program_counter
    }

    fn pc_inc_skip_2(&mut self) -> u16 {
        self.program_counter += 3;
        self.program_counter
    }

    fn check_half_carry(&self, a: u8, b: u8) -> bool {
        return (((a & 0xF) + (b & 0xF)) & 0x10) == 0x10;
    }

    fn fetch_opcode(&mut self, memory_bus: &mut Memory) -> u8 {
        memory_bus.read_memory(self.program_counter as usize)
    }

    fn opcode_nop(&mut self) -> ProgramCounter {
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_inc_b(&mut self) -> ProgramCounter {
        let value = self.register_bc.get_left();
        let result = self.register_bc.inc_left();

        self.register_af.set_bool_bit_right(Z_FLAG, result == 0);

        self.register_af.clear_bit_right(N_FLAG);

        let half_bool = self.check_half_carry(value, result);
        self.register_af.set_bool_bit_right(H_FLAG, half_bool);

        ProgramCounter::Next
    }

    fn opcode_dec_b(&mut self) -> ProgramCounter {
        let value = self.register_bc.get_left();
        let result = self.register_bc.dec_left();

        self.register_af.set_bool_bit_right(Z_FLAG, result == 0);

        self.register_af.set_bit_right(N_FLAG);

        let half_bool = self.check_half_carry(value, result);
        self.register_af.set_bool_bit_right(H_FLAG, half_bool);

        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_bn(&mut self, value: u8) -> ProgramCounter {
        self.register_bc.set_left(value);
        self.ticks += 8;

        ProgramCounter::Skip
    }

    fn opcode_inc_c(&mut self) -> ProgramCounter {
        let value = self.register_bc.get_right();
        let result = self.register_bc.inc_right();

        let z_bool = self.register_bc.get_right() == 0;
        self.register_af.set_bool_bit_right(Z_FLAG, z_bool);

        self.register_af.clear_bit_right(N_FLAG);

        let half_bool = self.check_half_carry(value, result);
        self.register_af.set_bool_bit_right(H_FLAG, half_bool);

        self.ticks += 4;
        ProgramCounter::Next
    }

     fn opcode_dec_c(&mut self) -> ProgramCounter {
        let value = self.register_bc.get_right();
        let result = self.register_bc.dec_right();

        self.register_af.set_bool_bit_right(Z_FLAG, result == 0);

        self.register_af.set_bit_right(N_FLAG);

        let half_bool = self.check_half_carry(value, result);
        self.register_af.set_bool_bit_right(H_FLAG, half_bool);

        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_cn(&mut self, value: u8) -> ProgramCounter {
        self.register_bc.set_right(value);
        self.ticks += 8;

        ProgramCounter::Skip
    }

    fn opcode_load_de_16(&mut self, value: u16) -> ProgramCounter {
        self.register_de.set(value);
        self.ticks += 12;

        ProgramCounter::Skip2
    }

    fn opcode_load_de_a(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        memory_bus.write_memory(self.register_de.get() as usize, 
                                     self.register_af.get_left());
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_dec_de(&mut self) -> ProgramCounter {
        self.register_de.dec();
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_roate_a_left(&mut self) -> ProgramCounter {
        let value = self.register_af.get_left().rotate_left(1);
        self.register_af.set_left(value);

        let reg_bool = self.register_af.get_bit_left(7, 0x80);
        self.register_af.set_bool_bit_right(C_FLAG, reg_bool);

        self.register_af.clear_bit_right(Z_FLAG);
        self.register_af.clear_bit_right(N_FLAG);
        self.register_af.clear_bit_right(H_FLAG);

        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_jr(&mut self, value: i8) -> ProgramCounter {
        self.ticks += 12;
         let jump_value = self.program_counter.wrapping_add(2)
                                              .wrapping_add(value as u16);
        ProgramCounter::Jump(jump_value)
    }

    fn opcode_inc_e(&mut self) -> ProgramCounter {
        self.register_de.inc_right();
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_a_de(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        let value = memory_bus.read_memory(self.register_de.get() as usize);
        self.register_af.set_left(value);
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_e_n(&mut self, value:u8) -> ProgramCounter {
        self.register_de.set_right(value);
        self.ticks += 8;

        ProgramCounter::Skip
    }

    fn opcode_pop_bc(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        self.register_bc.set(memory_bus.pop_16());
        self.ticks += 12;

        ProgramCounter::Next
    }

    fn opcode_jmp_nz(&mut self, value: i8)  -> ProgramCounter {
        self.ticks += 8;

        if self.register_af.get_bit_right(Z_FLAG, Z_FLAG_BIT) == false {
            let jump_value = self.program_counter.wrapping_add(2)
                                                 .wrapping_add(value as u16);


            return ProgramCounter::Jump(jump_value);
        }
        ProgramCounter::Skip
    }

    fn opcode_load_hl_16(&mut self, value: u16) -> ProgramCounter {
        self.register_hl.set(value);
        self.ticks += 12;

        ProgramCounter::Skip2
    }

    fn opcode_inc_hl(&mut self) -> ProgramCounter {
        self.register_hl.inc();
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_jr_nz(&mut self, value: i8) -> ProgramCounter {
        self.ticks += 8;

        if self.register_af.get_bit_right(Z_FLAG, Z_FLAG_BIT) {
                    let jump_value = self.program_counter.wrapping_add(2)
                                                          .wrapping_add(value as u16);

            return ProgramCounter::Jump(jump_value);                                    
        }

        ProgramCounter::Skip
    }

    fn opcode_load_a_hl_inc(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        memory_bus.write_memory(self.register_hl.get() as usize,
                                     self.register_af.get_left());

        self.register_hl.inc();
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_hl_a_inc(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        let value = memory_bus.read_memory((self.register_hl.get()) as usize);
        self.register_af.set_left(value);
        self.register_hl.inc_left();
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_l_n(&mut self, value:u8) -> ProgramCounter {
        self.register_hl.set_right(value);
        self.ticks += 8;

        ProgramCounter::Skip
    }

    fn opcode_load_sp_16(&mut self, value:u16, memory_bus: &mut Memory) -> ProgramCounter {
        memory_bus.set_stack_pointer(value);
        self.ticks += 12;

        ProgramCounter::Skip2
    }

    fn opcode_load_hl_a_dec(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        memory_bus.write_memory(self.register_hl.get() as usize, 
                                     self.register_af.get_left());
        self.register_hl.dec();
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_dec_a(&mut self) -> ProgramCounter {
        let value = self.register_af.get_left();
        let result = self.register_af.dec_left();

        self.register_af.set_bool_bit_right(Z_FLAG, result == 0);

        self.register_af.set_bit_right(N_FLAG);

        let half_bool = self.check_half_carry(value, result);
        self.register_af.set_bool_bit_right(H_FLAG, half_bool);

        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_load_n_a(&mut self, value: u8) -> ProgramCounter {
        self.register_af.set_left(value);
        self.ticks += 8;

        ProgramCounter::Skip
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

    fn opcode_load_bhl(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        self.register_bc.set_left(memory_bus.read_memory(addr));
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

    fn opcode_load_chl(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        self.register_bc.set_right(memory_bus.read_memory(addr));
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

    fn opcode_load_dhl(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        self.register_de.set_left(memory_bus.read_memory(addr));
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

    fn opcode_load_ehl(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        self.register_de.set_right(memory_bus.read_memory(addr));
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

    fn opcode_load_hhl(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        self.register_hl.set_left(memory_bus.read_memory(addr));
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

    fn opcode_load_lhl(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        self.register_hl.set_right(memory_bus.read_memory(addr));
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_la(&mut self) -> ProgramCounter {
        self.register_hl.set_right(self.register_af.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }



    fn opcode_load_hlb(&mut self, memory_bus: &mut Memory) -> ProgramCounter {

        let addr = self.register_hl.get() as usize;

        memory_bus.write_memory(addr, self.register_bc.get_left());
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_hlc(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;
        
        memory_bus.write_memory(addr, self.register_bc.get_right());
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_hld(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        memory_bus.write_memory(addr, self.register_de.get_left());
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_hle(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        memory_bus.write_memory(addr, self.register_de.get_right());
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_hlh(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        memory_bus.write_memory(addr, self.register_hl.get_left());
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_hll(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        memory_bus.write_memory(addr, self.register_hl.get_right());
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_hla(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        memory_bus.write_memory(addr, self.register_af.get_left());
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

    fn opcode_load_ahl(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        let addr = self.register_hl.get() as usize;

        self.register_af.set_left(memory_bus.read_memory(addr));
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_load_aa(&mut self) -> ProgramCounter {
        self.register_af.set_left(self.register_af.get_left());
        self.ticks += 4;

        ProgramCounter::Next
    }

    fn opcode_xor_aa(&mut self) -> ProgramCounter {
        let value = self.register_af.get_left() ^ self.register_af.get_left();
        self.register_af.set_left(value);

        let z_bool = self.register_af.get_left() == 0;
        self.register_af.set_bool_bit_right(Z_FLAG, z_bool);

        self.register_af.clear_bit_right(N_FLAG);
        self.register_af.clear_bit_right(H_FLAG);
        self.register_af.clear_bit_right(C_FLAG);
        
        self.ticks += 4;

        ProgramCounter::Next
    }


    fn opcode_rtn_nz(&mut self, memory_bus: &mut Memory) -> ProgramCounter {

        if self.register_af.get_bit_right(Z_FLAG, Z_FLAG_BIT) == false {
            self.ticks += 8;
            return ProgramCounter::Jump(memory_bus.pop_16());
        }

        ProgramCounter::Next
    }


    fn opcode_jmp(&mut self, address: u16) -> ProgramCounter {  
        self.ticks += 12;

        ProgramCounter::Jump(address)
    }

    fn opcode_load_a_ff00_plus_n(&mut self, value: u8, memory_bus: &mut Memory) -> ProgramCounter {
        let addr = (0xFF00 as u16) | (value as u16);
        let value = memory_bus.read_memory(addr as usize);

        self.register_af.set_left(value);
        self.ticks += 8;

        ProgramCounter::Skip
    }

    fn opcode_load_a_nn(&mut self, value: u16, memory_bus: &mut Memory) -> ProgramCounter {
        memory_bus.write_memory(value as usize, self.register_af.get_left());
        self.ticks += 16;

        ProgramCounter::Skip2
    }

    fn opcode_push_bc(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        memory_bus.push_16(self.register_bc.get());    
        self.ticks += 16;

        ProgramCounter::Next
    }

    fn opcode_return(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        let value = memory_bus.pop_16();
        self.ticks += 16;

        ProgramCounter::Jump(value)
    }


    fn opcode_load_a_ff00_plus_c(&mut self, memory_bus: &mut Memory) -> ProgramCounter {
        let value = (0xFF00 as u16) | (self.register_bc.get_right() as u16);
        memory_bus.write_memory(value as usize, self.register_af.get_left());
        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_com_a_n(&mut self, value: u8) -> ProgramCounter {
        let result = self.register_af.get_left().overflowing_sub(value);

        self.register_af.set_bool_bit_right(Z_FLAG, result.0 == 0);
        self.register_af.set_bit_right(N_FLAG);

        let half_bool = self.check_half_carry(value, result.0);
        self.register_af.set_bool_bit_right(H_FLAG, half_bool);

        let reg_bool = self.register_af.get_bit_right(7, 0x80);
        self.register_af.set_bool_bit_right(C_FLAG, reg_bool);

        self.ticks += 8;

        ProgramCounter::Skip
    }

    fn opcode_load_a_mem_ff00_plus_n(&mut self, value:u8, memory_bus: &mut Memory) -> ProgramCounter {
        let addr = (0xFF00 + value as u16) as usize;
        let result = memory_bus.read_memory(addr);

        self.register_de.set_right(result);
        self.ticks += 12; 

        ProgramCounter::Skip
    }


    fn cb_opcodes(&mut self, memory_bus: &mut Memory) -> ProgramCounter {

        self.pc_inc_next();
        let opcode = self.fetch_opcode(memory_bus);

        let pc_change = match opcode {
                0x7C => self.opcode_h_bit7(),
                0x11 => self.opcode_rotate_c_left_cb(),

            _ => panic!("CB Opcode {:X} isn't implemented", opcode)
        };
        
        pc_change   
    }

    fn opcode_h_bit7(&mut self) -> ProgramCounter {
        let reg_bool = !self.register_hl.get_bit_left(7, 0x80);
        self.register_af.set_bool_bit_right(Z_FLAG, reg_bool);

        self.register_af.clear_bit_right(N_FLAG);
        self.register_af.set_bit_right(H_FLAG);

        self.ticks += 8;
        ProgramCounter::Next
    }

    fn opcode_rotate_c_left_cb(&mut self) -> ProgramCounter {
        let value = self.register_bc.get_right().rotate_left(1);
        self.register_bc.set_right(value);

        let z_bool = self.register_bc.get_right() == 0;
        self.register_af.set_bool_bit_right(Z_FLAG, z_bool);

        self.register_af.clear_bit_right(N_FLAG);
        self.register_af.clear_bit_right(H_FLAG);

        let reg_bool = self.register_bc.get_bit_right(7, 0x80);
        self.register_af.set_bool_bit_right(C_FLAG, reg_bool);

        self.ticks += 8;

        ProgramCounter::Next
    }

    fn opcode_call(&mut self, value: u16, memory_bus: &mut Memory) -> ProgramCounter {
        let pc_count = self.pc_inc_skip_2();
        
        memory_bus.push_16(pc_count);
        self.ticks += 12;

        ProgramCounter::Jump(value)
    }
}