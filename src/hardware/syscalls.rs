use super::registers::Registers;
use std::io::{self, Read, Write};

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
        let mut char_buf = [0u8];
        io::stdin().read_exact(&mut char_buf).unwrap();
        registers.r0 = char_buf[0] as u16;
    }

    fn out(&self, registers: &Registers) {
        let char = (registers.r0 & 0xFF) as u8;
        print!("{}", char as char);
        io::stdout().flush().unwrap();
    }

    fn in_char(&self, registers: &mut Registers) {
        let mut stdout = io::stdout();
        write!(stdout, "Enter character: ").unwrap();
        stdout.flush().unwrap();
        let mut char_buf = [0u8];
        io::stdin().read_exact(&mut char_buf).unwrap();
        write!(stdout, "{}", char_buf[0] as char).unwrap();
        stdout.flush().unwrap();
        registers.r0 = char_buf[0] as u16;
    }

    fn putsp(&self, registers: &Registers, memory: &[u16]) {
        let start_addr = registers.r0 as usize;
        let mut stdout = io::stdout();
        for i in start_addr..memory.len() {
            let low_byte: u8 = (memory[i] & 0xFF) as u8;
            let high_byte: u8 = ((memory[i] >> 8) & 0xFF) as u8;
            write!(stdout, "{}", low_byte as char).unwrap();
            if high_byte != 0 {
                write!(stdout, "{}", high_byte as char).unwrap();
                continue;
            }

            break;
        }

        stdout.flush().unwrap();
    }

    fn halt(&self) {
        std::process::exit(0);
    }
}
