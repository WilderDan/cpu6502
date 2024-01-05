pub mod instruction;
use instruction::addressing_mode::AddressingMode;
use instruction::Instruction;
use lazy_static::lazy_static;
use std::collections::HashMap;

const INSTRUCTION_SET_SIZE: usize = 15;

lazy_static! {
    pub static ref INSTRUCTION_SET: [Instruction; INSTRUCTION_SET_SIZE] = [
        // AND
        Instruction::new(0x29, "AND", AddressingMode::Immediate, 2),
        Instruction::new(0x25, "AND", AddressingMode::ZeroPage, 2),
        Instruction::new(0x35, "AND", AddressingMode::ZeroPageX, 2),
        Instruction::new(0x2D, "AND", AddressingMode::Absolute, 3),
        Instruction::new(0x3D, "AND", AddressingMode::AbsoluteX, 3),
        Instruction::new(0x39, "AND", AddressingMode::AbsoluteY, 3),
        Instruction::new(0x21, "AND", AddressingMode::IndirectX, 2),
        Instruction::new(0x31, "AND", AddressingMode::IndirectY, 2),

        // BRK
        Instruction::new(0x00, "BRK", AddressingMode::Implicit, 1),

        // INX
        Instruction::new(0xE8, "INX", AddressingMode::Implicit, 1),

        // LDA
        Instruction::new(0xA9, "LDA", AddressingMode::Immediate, 2),

        // LDX
        Instruction::new(0xA2, "LDX", AddressingMode::Immediate, 2),

        // LDY
        Instruction::new(0xA0, "LDY", AddressingMode::Immediate, 2),

        // TAX
        Instruction::new(0xAA, "TAX", AddressingMode::Implicit, 1),

        // STA
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
