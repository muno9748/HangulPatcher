#![allow(non_upper_case_globals)]

pub const ESC: u8 = 27;
pub const F11: u8 = 0x7A;
pub const Slash: u8 = 0xBF;
pub const Enter: u8 = 0x0D;
pub const Backspace: u8 = 0x08;
pub const KoreanToggle: u8 = 0x15;

pub const V: u8 = 0x56;
pub const A: u8 = 0x41;
pub const T: u8 = 0x54;

pub fn is_control(key: u8) -> bool {
    (0x10 <= key && key <= 0x14) ||
    (0x21 <= key && key <= 0x24) ||
    (0x2C <= key && key <= 0x2F) ||
    (0x5B <= key && key <= 0x5D) ||
    (0x70 <= key && key <= 0x87) ||
    (0xA0 <= key && key <= 0xA5) ||
    key == 0x29 ||
    key == 0x2A ||
    key == 0x90 ||
    key == 0x91
}

pub fn is_arrow(key: u8) -> bool {
    0x25 <= key && key <= 0x28
}