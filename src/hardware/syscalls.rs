use std::io::{self, Write};
use super::registers::Registers;

/// the kernel itself
pub struct System;

impl System {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_trap(&mut self, trap_vector: u8, registers: &mut Registers, memory: &mut [u16]) {
        match trap_vector {
            0x20 => self.getc(registers),
            0x21 => self.out(registers),
            0x22 => self.puts(registers, memory),
            0x23 => self.in_char(registers),
            0x24 => self.putsp(registers, memory),
            0x25 => self.halt(),
            _ => {
                println!("Unknown TRAP vector: {:#04x}", trap_vector);
                std::process::exit(1);
            }
        }
    }

    ///prints string starting from address stored in r0
    fn puts(&self, registers: &Registers, memory: &[u16]) {
        let mut address = registers.r0 as usize;
        
        let mut stdout = io::stdout();
        while let Some(&word) = memory.get(address) {
            if word == 0 {
                break;
            }
            let c = (word & 0xFF) as u8;
            write!(stdout, "{}", c as char).unwrap();
            address += 1;
        }
        stdout.flush().unwrap();
    }

    fn getc(&self, registers: &mut Registers) {
        // TODO: Implement GETC (Read char, no echo)
    }

    fn out(&self, registers: &Registers) {
        // TODO: Implement OUT (Write char from R0)
    }

    fn in_char(&self, registers: &mut Registers) {
        // TODO: Implement IN (Print prompt, read char, echo)
    }

    fn putsp(&self, registers: &Registers, memory: &[u16]) {
        // TODO: Implement PUTSP (Packed string)
    }

    fn halt(&self) {
        println!("HALT");
        std::process::exit(0);
    }
}
