pub mod addressing_mode;
use addressing_mode::AddressingMode;

pub struct Instruction {
    pub opcode: u8,
    pub mnemonic: &'static str,
    pub mode: AddressingMode,
    pub length: u8, /* in bytes */
}

impl Instruction {
    pub const fn new(opcode: u8, mnemonic: &'static str, mode: AddressingMode, length: u8) -> Self {
        Instruction {
            opcode,
            mnemonic,
            mode,
            length,
        }
    }
}
