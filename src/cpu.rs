mod instruction_set;
mod status_flag;
use instruction_set::instruction::addressing_mode::AddressingMode;
use instruction_set::INSTRUCTION_MAP;
use status_flag::StatusFlag;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.mem_read(pos);
        let hi = self.mem_read(pos + 1);
        u16::from_le_bytes([lo, hi])
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run()
    }

    fn set_status_bit(&mut self, flag: StatusFlag) {
        self.status |= flag as u8;
    }

    fn unset_status_bit(&mut self, flag: StatusFlag) {
        self.status &= !(flag as u8);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.set_status_bit(StatusFlag::Zero)
        } else {
            self.unset_status_bit(StatusFlag::Zero)
        }

        if result & 0b1000_0000 != 0 {
            self.set_status_bit(StatusFlag::Negative)
        } else {
            self.unset_status_bit(StatusFlag::Negative)
        }
    }

    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,

            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,

            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),

            AddressingMode::ZeroPageX => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            }

            AddressingMode::ZeroPageY => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            }

            AddressingMode::AbsoluteX => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            }

            AddressingMode::AbsoluteY => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            }

            AddressingMode::IndirectX => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }

            AddressingMode::IndirectY => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }

            AddressingMode::Implicit | AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    fn get_operand_value(&mut self, mode: &AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode);
        self.mem_read(addr)
    }

    fn and(&mut self, mode: &AddressingMode) {
        let operand = self.get_operand_value(mode);

        self.register_a = self.register_a & operand;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn lda(&mut self, mode: &AddressingMode) {
        self.register_a = self.get_operand_value(mode);
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        self.register_x = self.get_operand_value(mode);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        self.register_y = self.get_operand_value(mode);
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        self.mem_write(address, self.register_a);
    }

    pub fn run(&mut self) {
        loop {
            // Fetch
            let opcode = self.mem_read(self.program_counter);
            self.program_counter += 1;

            // Decode
            let instruction = INSTRUCTION_MAP
                .get(&opcode)
                .expect(&format!("OpCode {:x} is not recognized", opcode));

            // Execute
            match opcode {
                // AND
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    self.and(&instruction.mode)
                }

                // LDA
                0xA9 => self.lda(&instruction.mode),

                // LDX
                0xA2 => self.ldx(&instruction.mode),

                // LDY
                0xA0 => self.ldy(&instruction.mode),

                // STA
                0x85 => self.sta(&instruction.mode),

                // Implicit addressing opcodes
                0xAA => self.tax(),
                0xE8 => self.inx(),
                0x00 => return,

                _ => todo!(),
            }

            self.program_counter += (instruction.length - 1) as u16;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x10, 0xaa, 0x00]);
        assert_eq!(cpu.register_x, 0x10);
    }

    #[test]
    fn test_0xe8_inx_increments_base() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_0xe8_inx_increments_wraps() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0xFF, 0xAA, 0xE8, 0x00]);
        assert_eq!(cpu.register_x, 0b0000_0000);
        assert!(cpu.status & (StatusFlag::Zero as u8) == StatusFlag::Zero as u8);
    }

    #[test]
    fn test_0xe8_inx_set_negative_flag() {
        let mut cpu = CPU::new();
        cpu.register_x = 0b0111_1111;
        cpu.load_and_run(vec![0xA9, 0b0111_1111, 0xAA, 0xE8, 0xe8, 0x00]);
        assert!(cpu.status & (StatusFlag::Negative as u8) == StatusFlag::Negative as u8);
    }

    #[test]
    fn test_0xe8_inx_no_negative_flag() {
        let mut cpu = CPU::new();
        cpu.register_x = 0b0111_1110;
        cpu.load_and_run(vec![0xe8, 0x00]);
        assert!(cpu.status & (StatusFlag::Negative as u8) == 0);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    #[should_panic]
    fn test_unknown_opcode() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0x02, 0x00]);
    }

    // AND

    // Immediate
    #[test]
    fn test_0x29_and() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b1010_1101, 0x29, 0b1111_1110, 0x00]);
        assert_eq!(cpu.register_a, 0b1010_1100);
        assert_eq!(
            cpu.status & (StatusFlag::Negative as u8),
            StatusFlag::Negative as u8
        );
        assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
    }

    // Zero Page
    #[test]
    fn test_0x25_and() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9,
            0xFF,
            0x85,
            0x10,
            0xa9,
            0b1111_0111,
            0x25,
            0x10,
            0x00,
        ]);
        assert_eq!(cpu.register_a, 0b1111_0111);
        assert_eq!(
            cpu.status & (StatusFlag::Negative as u8),
            StatusFlag::Negative as u8
        );
        assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
    }

    // Zero Page X
    #[test]
    fn test_0x35_and() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9,
            0xFF,
            0x85,
            0x10,
            0xa9,
            0b1111_0111,
            0x35,
            0x10,
            0x00,
        ]);
        assert_eq!(cpu.register_a, 0b1111_0111);
        assert_eq!(
            cpu.status & (StatusFlag::Negative as u8),
            StatusFlag::Negative as u8
        );
        assert_eq!(cpu.status & (StatusFlag::Zero as u8), 0);
    }

    // Absolute
    #[test]
    fn test_0x2d_and() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x1234, 0b1111_1111);
        cpu.load_and_run(vec![0xa9, 0b1010_1010, 0x2d, 0x34, 0x12, 0x00]);
        assert_eq!(cpu.register_a, 0b1010_1010);
    }

    // Absolute X
    #[test]
    fn test_0x3d_and() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x4321, 0b1111_1111);
        // Load A with 42. Load X with 1. Address 0x4320 + 1 (see above). AND with A (42)
        cpu.load_and_run(vec![0xa9, 42, 0xa2, 1, 0x3d, 0x20, 0x43, 0x00]);
        assert_eq!(cpu.register_a, 42);
    }

    // Absoulute Y
    #[test]
    fn test_0x39_and() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x4321, 0b0000_1000);
        // Load A with bits. Load Y with 3. Address 0x431E + Y (see above). AND that with A's bits
        cpu.load_and_run(vec![0xa9, 0b0000_1111, 0xa0, 3, 0x39, 0x1E, 0x43, 0x00]);
        assert_eq!(cpu.register_a, 8);
    }

    // Indexed Indirect (Indirect X)
    #[test]
    fn test_0x21_and() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0x14, 0x1234);
        cpu.mem_write(0x1234, 0b0010_0010);
        cpu.load_and_run(vec![0xa9, 0b0000_1111, 0xa2, 4, 0x21, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 2);
    }

    // Indirect Indexed (Indirect Y)
    #[test]
    fn test_0x31_and() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0x10, 0x1234);
        cpu.mem_write(0x1238, 0b0010_0010);
        cpu.load_and_run(vec![0xa9, 0b0000_1111, 0xa0, 4, 0x31, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 2);
    }

    // LDX
    #[test]
    fn test_0xa2_ldx() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0xee, 0x00]);
        assert_eq!(cpu.register_x, 0xee);
    }

    // LDY
    #[test]
    fn test_0xa0_ldy() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x12, 0x00]);
        assert_eq!(cpu.register_y, 0x12);
    }

    // STA
    #[test]
    fn test_0x85_sta() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xFF, 0x85, 0x10, 0x00]);
        assert_eq!(cpu.mem_read(0x10 as u16), 0xFF);
    }
}
