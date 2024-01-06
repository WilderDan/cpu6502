// Reference: https://www.nesdev.org/obelisk-6502-guide/addressing.html#REL

#[derive(Debug)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX, /* Indexed Indirect */
    IndirectY, /* Indirect Indexed */
}
