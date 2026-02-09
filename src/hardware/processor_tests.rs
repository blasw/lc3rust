use super::processor::{ExecutionResult, Processor};

fn memory() -> [u16; 65536] {
    [0; 65536]
}

#[test]
fn add_register_mode() {
    let mut processor = Processor::new();
    let mut mem = memory();
    processor.registers.update(1, 10);
    processor.registers.update(2, 5);

    let instr = 0b0001_000_001_0_00_010; // ADD R0, R1, R2
    let result = processor.execute(instr, &mut mem);

    assert!(matches!(result, ExecutionResult::Continue));
    assert_eq!(processor.registers.get(0), 15);
    assert_eq!(processor.registers.cond, 1); // POS
}

#[test]
fn add_immediate_mode() {
    let mut processor = Processor::new();
    let mut mem = memory();
    processor.registers.update(1, 10);

    let instr = 0b0001_000_001_1_11110; // ADD R0, R1, -2
    processor.execute(instr, &mut mem);

    assert_eq!(processor.registers.get(0), 8);
}

#[test]
fn and_register_mode() {
    let mut processor = Processor::new();
    let mut mem = memory();
    processor.registers.update(1, 0b1010);
    processor.registers.update(2, 0b1100);

    let instr = 0b0101_000_001_0_00_010; // AND R0, R1, R2
    processor.execute(instr, &mut mem);

    assert_eq!(processor.registers.get(0), 0b1000);
}

#[test]
fn and_immediate_mode() {
    let mut processor = Processor::new();
    let mut mem = memory();
    processor.registers.update(1, 0b1111);

    let instr = 0b0101_000_001_1_00101; // AND R0, R1, #5
    processor.execute(instr, &mut mem);

    assert_eq!(processor.registers.get(0), 0b0101);
}

#[test]
fn not_register_mode_and_negative_flag() {
    let mut processor = Processor::new();
    let mut mem = memory();
    processor.registers.update(1, 0x0001);

    let instr = 0b1001_000_001_111111; // NOT R0, R1
    processor.execute(instr, &mut mem);

    assert_eq!(processor.registers.get(0), 0xFFFE);
    assert_eq!(processor.registers.cond, 4); // NEG
}

#[test]
fn br_takes_branch_when_condition_matches() {
    let mut processor = Processor::new();
    let mut mem = memory();
    processor.registers.cond = 2; // ZRO
    let initial_pc = processor.registers.pc;

    let instr = 0b0000_010_000001010; // BRz +10
    processor.execute(instr, &mut mem);

    assert_eq!(processor.registers.pc, initial_pc.wrapping_add(10));
}

#[test]
fn br_does_not_branch_when_condition_does_not_match() {
    let mut processor = Processor::new();
    let mut mem = memory();
    processor.registers.cond = 1; // POS
    let initial_pc = processor.registers.pc;

    let instr = 0b0000_010_000001010; // BRz +10
    processor.execute(instr, &mut mem);

    assert_eq!(processor.registers.pc, initial_pc);
}

#[test]
fn br_with_clear_condition_register_is_noop() {
    let mut processor = Processor::new();
    let mut mem = memory();
    processor.registers.cond = 0;
    let initial_pc = processor.registers.pc;

    let instr = 0b0000_111_000000001; // BRnzp +1
    processor.execute(instr, &mut mem);

    assert_eq!(processor.registers.pc, initial_pc);
}

#[test]
fn jmp_sets_pc_to_base_register() {
    let mut processor = Processor::new();
    let mut mem = memory();
    processor.registers.update(3, 0x3456);

    let instr = 0b1100_000_011_000000; // JMP R3
    processor.execute(instr, &mut mem);

    assert_eq!(processor.registers.pc, 0x3456);
}

#[test]
fn jsrr_saves_return_address_and_jumps_to_base_register() {
    let mut processor = Processor::new();
    let mut mem = memory();
    processor.registers.pc = 0x3005;
    processor.registers.update(2, 0x4000);

    let instr = 0b0100_0_00_010_000000; // JSRR R2
    processor.execute(instr, &mut mem);

    assert_eq!(processor.registers.get(7), 0x3005);
    assert_eq!(processor.registers.pc, 0x4000);
}

#[test]
fn jsr_saves_return_address_and_jumps_by_offset() {
    let mut processor = Processor::new();
    let mut mem = memory();
    processor.registers.pc = 0x3005;

    let instr = 0b0100_1_00000000011; // JSR +3
    processor.execute(instr, &mut mem);

    assert_eq!(processor.registers.get(7), 0x3005);
    assert_eq!(processor.registers.pc, 0x3008);
}

#[test]
fn ld_loads_from_pc_relative_address_and_sets_flags() {
    let mut processor = Processor::new();
    let mut mem = memory();
    processor.registers.pc = 0x3000;
    mem[0x3002] = 0xFFFF;

    let instr = 0b0010_000_000000010; // LD R0, +2
    processor.execute(instr, &mut mem);

    assert_eq!(processor.registers.get(0), 0xFFFF);
    assert_eq!(processor.registers.cond, 4); // NEG
}

#[test]
fn ldi_loads_indirect_value() {
    let mut processor = Processor::new();
    let mut mem = memory();
    processor.registers.pc = 0x3000;
    mem[0x3001] = 0x4000;
    mem[0x4000] = 0x1234;

    let instr = 0b1010_000_000000001; // LDI R0, +1
    processor.execute(instr, &mut mem);

    assert_eq!(processor.registers.get(0), 0x1234);
}

#[test]
fn ldr_loads_base_plus_offset() {
    let mut processor = Processor::new();
    let mut mem = memory();
    processor.registers.update(1, 0x4100);
    mem[0x4102] = 0xBEEF;

    let instr = 0b0110_000_001_000010; // LDR R0, R1, +2
    processor.execute(instr, &mut mem);

    assert_eq!(processor.registers.get(0), 0xBEEF);
}

#[test]
fn lea_writes_effective_address() {
    let mut processor = Processor::new();
    let mut mem = memory();
    processor.registers.pc = 0x3000;

    let instr = 0b1110_000_000000111; // LEA R0, +7
    processor.execute(instr, &mut mem);

    assert_eq!(processor.registers.get(0), 0x3007);
}

#[test]
fn st_writes_to_pc_relative_address() {
    let mut processor = Processor::new();
    let mut mem = memory();
    processor.registers.pc = 0x3000;
    processor.registers.update(3, 0xCAFE);

    let instr = 0b0011_011_000000010; // ST R3, +2
    processor.execute(instr, &mut mem);

    assert_eq!(mem[0x3002], 0xCAFE);
}

#[test]
fn sti_writes_indirectly() {
    let mut processor = Processor::new();
    let mut mem = memory();
    processor.registers.pc = 0x3000;
    processor.registers.update(4, 0xA0A0);
    mem[0x3001] = 0x4200;

    let instr = 0b1011_100_000000001; // STI R4, +1
    processor.execute(instr, &mut mem);

    assert_eq!(mem[0x4200], 0xA0A0);
}

#[test]
fn str_writes_base_plus_offset() {
    let mut processor = Processor::new();
    let mut mem = memory();
    processor.registers.update(5, 0xABCD);
    processor.registers.update(2, 0x4400);

    let instr = 0b0111_101_010_000001; // STR R5, R2, +1
    processor.execute(instr, &mut mem);

    assert_eq!(mem[0x4401], 0xABCD);
}

#[test]
fn trap_vectors_are_returned_to_system_layer() {
    let mut processor = Processor::new();
    let mut mem = memory();
    let vectors = [0x20u16, 0x21u16, 0x22u16, 0x23u16, 0x24u16, 0x25u16];

    for vector in vectors {
        let instr = 0xF000 | vector; // TRAP x??
        let result = processor.execute(instr, &mut mem);
        assert!(matches!(result, ExecutionResult::Trap(v) if v == vector as u8));
    }
}

#[test]
fn rti_and_res_do_not_panic_and_continue() {
    let mut processor = Processor::new();
    let mut mem = memory();

    let rti = 0x8000;
    let res = 0xD000;

    assert!(matches!(
        processor.execute(rti, &mut mem),
        ExecutionResult::Continue
    ));
    assert!(matches!(
        processor.execute(res, &mut mem),
        ExecutionResult::Continue
    ));
}
