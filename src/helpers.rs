pub fn convert_to_12bit_signed(value: i32) -> u16 {
    let value = match value < 0 {
        true => !value + 1,
        false => value,
    };

    (value << 3) as u16
}

pub fn convert_from_12bit_signed(value: u16) -> i32 {
    let value = match value & 0x8000 > 0 {
        true => !value + 1,
        false => value,
    };

    (value >> 3) as i32
}
