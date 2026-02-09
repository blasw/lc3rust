use super::processor::{ExecutionResult, Processor};
use super::syscalls::System;

const MEMORY_SIZE: usize = u16::MAX as usize + 1;

pub struct VM {
    memory: [u16; MEMORY_SIZE],
    processor: Processor,
    system: System,
}

impl VM {
    pub fn new() -> VM {
        VM {
            memory: [0; MEMORY_SIZE],
            processor: Processor::new(),
            system: System::new(),
        }
    }

    pub fn write_memory(&mut self, addr: u16, value: u16) {
        self.memory[addr as usize] = value;
    }

    pub fn read_memory(&self, addr: u16) -> u16 {
        self.memory[addr as usize]
    }

    pub fn execute(&mut self) {
        while (self.processor.registers.pc as usize) < MEMORY_SIZE {
            let pc = self.processor.registers.pc;
            let instruction = self.memory[pc as usize];
            self.processor.registers.pc += 1;

            match self.processor.execute(instruction, &mut self.memory) {
                ExecutionResult::Continue => {}
                ExecutionResult::Trap(trap_vector) => {
                    self.system.handle_trap(trap_vector, &mut self.processor.registers, &mut self.memory);
                }
            }
        }
    }
}
