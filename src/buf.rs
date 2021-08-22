#![allow(dead_code)]
use sejong::Buffer;

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum FinalConsonant {
    None,
    G,
    KK,
    GS,
    N,
    NJ,
    NH,
    D,
    L,
    LG,
    LM,
    LB,
    LS,
    LT,
    LP,
    LH,
    M,
    B,
    BS,
    S,
    SS,
    NG,
    J,
    CH,
    K,
    T,
    P,
    H,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum InitialConsonant {
    G,
    KK,
    N,
    D,
    TT,
    R,
    M,
    B,
    PP,
    S,
    SS,
    NG,
    J,
    JJ,
    CH,
    K,
    T,
    P,
    H,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum MedialVowel {
    A,
    AE,
    YA,
    YAE,
    EO,
    E,
    YEO,
    YE,
    O,
    WA,
    WAE,
    OE,
    YO,
    U,
    WO,
    WE,
    WI,
    YU,
    EU,
    YI,
    I,
}

#[derive(Clone)]
pub struct XBuffer(pub Vec<Syllable>);

#[derive(Clone, Copy, Debug)]
pub enum Syllable {
    Initial(InitialConsonant),
    Medial(InitialConsonant, MedialVowel),
    Final(InitialConsonant, MedialVowel, FinalConsonant),
    VowelOnly(MedialVowel),
}

pub unsafe fn convert(buf: *mut Buffer) -> *mut XBuffer {
    std::mem::transmute::<*mut Buffer, *mut XBuffer>(buf)
}

pub unsafe fn remove_first(buf: *mut Buffer) {
    let xbuf = convert(buf);

    (*xbuf).0.remove(0);
}

pub unsafe fn get_length(buf: *mut Buffer) -> usize {
    let xbuf = convert(buf);

    (*xbuf).0.len()
}
