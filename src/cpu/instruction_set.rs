pub mod instruction;
use instruction::addressing_mode::AddressingMode;
use instruction::Instruction;
use lazy_static::lazy_static;
use std::collections::HashMap;

const INSTRUCTION_SET_SIZE: usize = 26;

lazy_static! {
    pub static ref INSTRUCTION_SET: [Instruction; INSTRUCTION_SET_SIZE] = [
        // AND - Logical AND
        Instruction::new(0x29, "AND", AddressingMode::Immediate, 2),
        Instruction::new(0x25, "AND", AddressingMode::ZeroPage, 2),
        Instruction::new(0x35, "AND", AddressingMode::ZeroPageX, 2),
        Instruction::new(0x2D, "AND", AddressingMode::Absolute, 3),
        Instruction::new(0x3D, "AND", AddressingMode::AbsoluteX, 3),
        Instruction::new(0x39, "AND", AddressingMode::AbsoluteY, 3),
        Instruction::new(0x21, "AND", AddressingMode::IndirectX, 2),
        Instruction::new(0x31, "AND", AddressingMode::IndirectY, 2),

        // ASL - Arithmetic Shift Left
        Instruction::new(0x0A, "ASL", AddressingMode::Accumulator, 1),
        Instruction::new(0x06, "ASL", AddressingMode::ZeroPage, 2),
        Instruction::new(0x16, "ASL", AddressingMode::ZeroPageX, 2),
        Instruction::new(0x0E, "ASL", AddressingMode::Absolute, 3),
        Instruction::new(0x1E, "ASL", AddressingMode::AbsoluteX, 3),

        // BCC - Branch if Carry Clear
        Instruction::new(0x90, "BCC", AddressingMode::Relative, 2),

        // BCS - Branch if Carry Set
        Instruction::new(0xB0, "BCS", AddressingMode::Relative, 2),

        // BEQ - Branch if Equal
        Instruction::new(0xF0, "BEQ", AddressingMode::Relative, 2),

        // BRK - (Break) Force Interrupt
        Instruction::new(0x00, "BRK", AddressingMode::Implicit, 1),

        // BIT - Bit Test
        Instruction::new(0x24, "BIT", AddressingMode::ZeroPage, 2),
        Instruction::new(0x2C, "BIT", AddressingMode::Absolute, 3),

        // BMI - Branch if Minus
        Instruction::new(0x30, "BMI", AddressingMode::Relative, 2),

        // INX - Increment X Register
        Instruction::new(0xE8, "INX", AddressingMode::Implicit, 1),

        // LDA - Load Accumulator
        Instruction::new(0xA9, "LDA", AddressingMode::Immediate, 2),

        // LDX - Load X Register
        Instruction::new(0xA2, "LDX", AddressingMode::Immediate, 2),

        // LDY - Load Y Register
        Instruction::new(0xA0, "LDY", AddressingMode::Immediate, 2),

        // TAX - Transfer Accumulator to X Register
        Instruction::new(0xAA, "TAX", AddressingMode::Implicit, 1),

        // STA - Store Accumulator
        Instruction::new(0x85, "STA", AddressingMode::ZeroPage, 2),
    ];

    pub static ref INSTRUCTION_MAP: HashMap<u8, &'static Instruction> = {
        let mut map = HashMap::new();

        let mut i = 0;
        while i < INSTRUCTION_SET_SIZE {
            let instruction = &INSTRUCTION_SET[i];
            map.insert(instruction.opcode, instruction);
            i += 1;
        }

        map
    };
}
