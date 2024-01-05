pub fn get_bit_at(input: &u8, n: u8) -> bool {
    if n < 8 {
        input & (1 << n) != 0
    } else {
        panic!("[get_bit_at]: Too high of bit postion specified")
    }
}
