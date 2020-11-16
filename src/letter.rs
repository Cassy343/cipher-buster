pub const fn c2i(character: u8) -> usize {
    (character - ('a' as u8)) as usize
}

pub const fn i2c(index: usize) -> u8 {
    index as u8 + ('a' as u8)
}

pub const fn shift(character: u8, amount: usize) -> u8 {
    i2c((c2i(character) + amount) % 26)
}

pub const fn is_letter(character: u8) -> bool {
    character >= 'a' as u8 && character <= 'z' as u8
}