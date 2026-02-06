use super::registers::Registers;
use crate::utils::sign_extend;

pub enum OpCode {
    BR = 0, // branch
    ADD,    // add
    LD,     // load
    ST,     // store
    JSR,    // jump register
    AND,    // bitwise and
    LDR,    // load register
    STR,    // store register
    RTI,    // unused
    NOT,    // bitwise not
    LDI,    // load indirect
    STI,    // store indirect
    JMP,    // jump
    RES,    // reserved (unused)
    LEA,    // load effective address
    TRAP,   // execute trap
}

impl OpCode {
    pub fn get_op_code(val: &u16) -> Option<OpCode> {
        match val >> 12 {
            0 => Some(OpCode::BR),
            1 => Some(OpCode::ADD),
            2 => Some(OpCode::LD),
            3 => Some(OpCode::ST),
            4 => Some(OpCode::JSR),
            5 => Some(OpCode::AND),
            6 => Some(OpCode::LDR),
            7 => Some(OpCode::STR),
            8 => Some(OpCode::RTI),
            9 => Some(OpCode::NOT),
            10 => Some(OpCode::LDI),
            11 => Some(OpCode::STI),
            12 => Some(OpCode::JMP),
            13 => Some(OpCode::RES),
            14 => Some(OpCode::LEA),
            15 => Some(OpCode::TRAP),
            _ => None,
        }
    }
}

pub enum ExecutionResult {
    Continue,
    // need kernel help
    Trap(u8),
}

pub(super) struct Processor {
    pub registers: Registers,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
        }
    }

    pub fn execute(&mut self, instr: u16, memory: &mut [u16]) -> ExecutionResult {
        let op = match OpCode::get_op_code(&instr) {
            Some(op) => op,
            None => return ExecutionResult::Continue, //handle invalid opcode?
        };

        match op {
            OpCode::ADD => self.add(instr),
            OpCode::AND => self.and(instr),
            OpCode::NOT => self.not(instr),
            OpCode::BR => self.br(instr),
            OpCode::JMP => self.jmp(instr),
            OpCode::JSR => self.jsr(instr),
            OpCode::LD => self.ld(instr, memory),
            OpCode::LDI => self.ldi(instr, memory),
            OpCode::LDR => self.ldr(instr, memory),
            OpCode::LEA => self.lea(instr),
            OpCode::ST => self.st(instr, memory),
            OpCode::STI => self.sti(instr, memory),
            OpCode::STR => self.str(instr, memory),
            OpCode::TRAP => return self.trap(instr), // pass to OS
            _ => {}
        }
        ExecutionResult::Continue
    }

    fn add(&mut self, instr: u16) {
        let dr = (instr >> 9) & 0x7;
        let sr1 = (instr >> 6) & 0x7;
        let mode = (instr >> 5) & 0x1;
        let val1 = self.registers.get(sr1);
        let val2: u16;
        match mode {
            0 => {
                let sr2 = instr & 0x7;
                val2 = self.registers.get(sr2);
            }
            1 => val2 = sign_extend(instr & 0x1F, 5),
            _ => unreachable!(),
        }

        self.registers.update(dr, val1.wrapping_add(val2));
        self.registers.update_r_cond_register(dr);
    }

    fn and(&mut self, instr: u16) {
        let dr = (instr >> 9) & 0x7;
        let sr1 = (instr >> 6) & 0x7;
        let mode = (instr >> 5) & 0x1;
        let val1 = self.registers.get(sr1);
        let val2 = match mode {
            0 => {
                let sr2 = instr & 0x7;
                self.registers.get(sr2)
            }
            1 => sign_extend(instr & 0x1F, 5),
            _ => unreachable!(),
        };
        self.registers.update(dr, val1 & val2);
        self.registers.update_r_cond_register(dr);
    }

    fn not(&mut self, instr: u16) {
        let dr = (instr >> 9) & 0x7;
        let sr = (instr >> 6) & 0x7;
        let val = !self.registers.get(sr);
        self.registers.update(dr, val);
        self.registers.update_r_cond_register(dr);
    }

    fn br(&mut self, instr: u16) {
        let offset = sign_extend(instr & 0x1FF, 9);
        let cond = self.registers.cond;
        match cond {
            1 => {
                if (instr >> 9) & 0x1 == 1 {
                    self.registers.pc = self.registers.pc.wrapping_add(offset);
                }
            }
            2 => {
                if (instr >> 10) & 0x1 == 1 {
                    self.registers.pc = self.registers.pc.wrapping_add(offset);
                }
            }
            4 => {
                if (instr >> 11) & 0x1 == 1 {
                    self.registers.pc = self.registers.pc.wrapping_add(offset);
                }
            }
            _ => unreachable!(),
        }
    }

    fn jmp(&mut self, instr: u16) {
        let target_reg = (instr >> 6) & 0x7;
        let target_addr = self.registers.get(target_reg);
        self.registers.pc = target_addr;
    }

    fn jsr(&mut self, instr: u16) {
        let mode = (instr >> 11) & 0x1;
        self.registers.update(7, self.registers.get(8));
        match mode {
            0 => {
                let base_register = (instr >> 6) & 0x7;
                let offset = sign_extend(self.registers.get(base_register), 6);
                self.registers
                    .update(8, self.registers.pc.wrapping_sub(offset));
            }
            1 => {}
            _ => unreachable!(),
        }
    }

    fn ld(&mut self, instr: u16, memory: &[u16]) {}

    fn ldi(&mut self, instr: u16, memory: &[u16]) {}

    fn ldr(&mut self, instr: u16, memory: &[u16]) {}

    fn lea(&mut self, instr: u16) {
        let dr = (instr >> 9) & 0x7;
        let pc_offset = sign_extend(instr & 0x1FF, 9);
        let val = self.registers.pc.wrapping_add(pc_offset);
        self.registers.update(dr, val);
        self.registers.update_r_cond_register(dr);
    }

    fn st(&mut self, instr: u16, memory: &mut [u16]) {}

    fn sti(&mut self, instr: u16, memory: &mut [u16]) {}

    fn str(&mut self, instr: u16, _memory: &mut [u16]) {}

    fn trap(&mut self, instr: u16) -> ExecutionResult {
        let trap_vector = (instr & 0xFF) as u8;
        ExecutionResult::Trap(trap_vector)
    }
}
