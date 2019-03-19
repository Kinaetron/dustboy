use std::num::Wrapping;

pub struct CPU {
    registers: [Wrapping<u16>; 4],
    stack_pointer:  Wrapping<u16>,
    program_counter: Wrapping<u16>,
}

impl CPU {
    pub fn new() -> Self {
        let cpu = CPU {
            registers: [Wrapping(0); 4],
            stack_pointer:  Wrapping(0),
            program_counter: Wrapping(0),
        };
        cpu
    }
}