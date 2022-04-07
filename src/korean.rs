use crate::char_type::CharType;
use phf::{phf_map, Map};

// Map<VirtualKeyCode, (CharType, JongsungMap, JongsungID, MoumMap, MoumID, KoreanCode)>
pub type MappedChar = (CharType, u8, u8, u8, u8, u8);

pub static KOR_MAP: Map<u8, MappedChar> = phf_map! {
    65u8 => (CharType::Chosung,  0, 4,  0, 0, 16), // a => ㅁ
    66u8 => (CharType::Jungsung, 0, 0,  0, 0, 17), // b => ㅠ
    67u8 => (CharType::Chosung,  0, 0,  0, 0, 25), // c => ㅊ
    68u8 => (CharType::Chosung,  0, 0,  0, 0, 22), // d => ㅇ
    69u8 => (CharType::Chosung,  0, 0,  0, 0, 6),  // e => ㄷ
    70u8 => (CharType::Chosung,  4, 0,  0, 0, 8),  // f => ㄹ
    71u8 => (CharType::Chosung,  0, 6,  0, 0, 29), // g => ㅎ
    72u8 => (CharType::Jungsung, 0, 0,  1, 0, 8),  // h => ㅗ
    73u8 => (CharType::Jungsung, 0, 0,  0, 0, 2),  // i => ㅑ
    74u8 => (CharType::Jungsung, 0, 0,  0, 2, 4),  // j => ㅓ
    75u8 => (CharType::Jungsung, 0, 0,  0, 1, 0),  // k => ㅏ
    76u8 => (CharType::Jungsung, 0, 0,  0, 7, 20), // l => ㅣ
    77u8 => (CharType::Jungsung, 0, 0,  4, 0, 18), // m => ㅡ
    78u8 => (CharType::Jungsung, 0, 0,  2, 0, 13), // n => ㅜ
    79u8 => (CharType::Jungsung, 0, 0,  0, 1, 1),  // o => ㅐ
    80u8 => (CharType::Jungsung, 0, 0,  0, 2, 5),  // p => ㅔ
    81u8 => (CharType::Chosung,  8, 4,  0, 0, 17), // q => ㅂ
    82u8 => (CharType::Chosung,  1, 4,  0, 0, 0),  // r => ㄱ
    83u8 => (CharType::Chosung,  2, 0,  0, 0, 3),  // s => ㄴ
    84u8 => (CharType::Chosung,  0, 13, 0, 0, 20), // t => ㅅ
    85u8 => (CharType::Jungsung, 0, 0,  0, 0, 6),  // u => ㅕ
    86u8 => (CharType::Chosung,  0, 4,  0, 0, 28), // v => ㅍ
    87u8 => (CharType::Chosung,  0, 2,  0, 0, 23), // w => ㅈ
    88u8 => (CharType::Chosung,  0, 4,  0, 0, 27), // x => ㅌ
    89u8 => (CharType::Jungsung, 0, 0,  0, 0, 12), // y => ㅛ
    90u8 => (CharType::Chosung,  0, 0,  0, 0, 26), // z => ㅋ
};

// Map<VirtualKeyCode, (CharType, ShiftApplied, KoreanCode)>
pub static KOR_MAP_SHIFT: Map<u8, (CharType, bool, u8)> = phf_map! {
    65u8 => (CharType::Chosung,  false, 16), // a => ㅁ
    66u8 => (CharType::Jungsung, false, 17), // b => ㅠ
    67u8 => (CharType::Chosung,  false, 25), // c => ㅊ
    68u8 => (CharType::Chosung,  false, 22), // d => ㅇ
    69u8 => (CharType::Chosung,  true,  7),  // e => ㄸ
    70u8 => (CharType::Chosung,  false, 8),  // f => ㄹ
    71u8 => (CharType::Chosung,  false, 29), // g => ㅎ
    72u8 => (CharType::Jungsung, false, 8),  // h => ㅗ
    73u8 => (CharType::Jungsung, false, 2),  // i => ㅑ
    74u8 => (CharType::Jungsung, false, 4),  // j => ㅓ
    75u8 => (CharType::Jungsung, false, 0),  // k => ㅏ
    76u8 => (CharType::Jungsung, false, 20), // l => ㅣ
    77u8 => (CharType::Jungsung, false, 18), // m => ㅡ
    78u8 => (CharType::Jungsung, false, 13), // n => ㅜ
    79u8 => (CharType::Jungsung, true,  3),  // o => ㅒ
    80u8 => (CharType::Jungsung, true,  7),  // p => ㅖ
    81u8 => (CharType::Chosung,  true,  18), // q => ㅃ
    82u8 => (CharType::Chosung,  true,  1),  // r => ㄲ
    83u8 => (CharType::Chosung,  false, 3),  // s => ㄴ
    84u8 => (CharType::Chosung,  true,  21), // t => ㅆ
    85u8 => (CharType::Jungsung, false, 6),  // u => ㅕ
    86u8 => (CharType::Chosung,  false, 28), // v => ㅍ
    87u8 => (CharType::Chosung,  true,  24), // w => ㅉ
    88u8 => (CharType::Chosung,  false, 27), // x => ㅌ
    89u8 => (CharType::Jungsung, false, 12), // y => ㅛ
    90u8 => (CharType::Chosung,  false, 26), // z => ㅋ
};

#[inline(always)]
pub fn assemble(chosung: u8, jungsung: u8, jongsung: u8) -> u16 {
    0xAC00 + (map_char_to_chosung(chosung) as u16 * 588) + (jungsung as u16 * 28) + map_char_to_jongsung(jongsung) as u16
}

#[inline(always)]
pub fn is_mappable(key: u8) -> bool {
    64 < key && key < 91
}

#[inline(always)]
pub fn utf16_jungsung(jungsung: u8) -> u16 {
    jungsung as u16 + 12623
}

#[inline(always)]
pub fn utf16_jongsung(jongsung: u8) -> u16 {
    jongsung as u16 + 12593
}

pub fn map_char_to_chosung(c: u8) -> u8 {
    match c {
        0 => 0,
        1 => 1,
        3 => 2,
        6 => 3,
        7 => 4,
        8 => 5,
        16 => 6,
        17 => 7,
        18 => 8,
        20 => 9,
        21 => 10,
        22 => 11,
        23 => 12,
        24 => 13,
        25 => 14,
        26 => 15,
        27 => 16,
        28 => 17,
        29 => 18,
        _ => unreachable!()
    }
}

pub fn map_char_to_jongsung(c: u8) -> u8 {
    match c {
        x if x <= 7 => x,
        x if x <= 18 => x - 1,
        x if x <= 24 => x - 2,
        x if x <= 30 => x - 3,
        _ => unreachable!()
    }
}

pub fn assemble_jungsung(a: u8, b: u8) -> u8 {
    match a {
        8 if b == 0 => 9,
        8 if b == 1 => 10,
        8 => 11,

        13 if b == 4 => 14,
        13 if b == 5 => 15,
        13 => 16,

        18 => 19,

        _ => unreachable!()
    }
}

pub fn assemble_jongsung(a: u8, b: u8) -> u8 {
    match a {
        0 => 2,

        3 if b == 23 => 4,
        3 => 5,

        8 if b == 0 => 9,
        8 if b == 16 => 10,
        8 if b == 17 => 11,
        8 if b == 20 => 12,
        8 if b == 27 => 13,
        8 if b == 28 => 14,
        8 => 15,
        
        17 => 19,

        _ => unreachable!()
    }
}