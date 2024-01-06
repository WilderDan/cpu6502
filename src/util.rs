pub fn get_bit_at(input: u8, pos: u8) -> bool {
    if pos < 8 {
        input & (1 << pos) != 0
    } else {
        panic!("[get_bit_at]: Too high of bit postion specified")
    }
}

pub fn get_address_from_offset(addr: u16, offset: u8) -> u16 {
    let value = offset & 0b0111_1111;

    if get_bit_at(offset, 7) {
        addr - value as u16
    } else {
        addr + value as u16
    }
}
