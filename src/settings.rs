use std::sync::atomic::{AtomicBool, AtomicU8};

use crate::keys;

pub struct Settings {
    pub block_kr_toggle_ingame: AtomicBool,
    pub auto_disable_korean: AtomicBool,
    pub korean_toggle_key: AtomicU8,
    pub show_overlay: AtomicBool,
    pub chat_open_key: AtomicU8,
    pub cmd_open_key: AtomicU8,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            korean_toggle_key: AtomicU8::new(keys::KoreanToggle),
            block_kr_toggle_ingame: AtomicBool::new(true),
            auto_disable_korean: AtomicBool::new(false),
            cmd_open_key: AtomicU8::new(keys::Slash),
            chat_open_key: AtomicU8::new(keys::T),
            show_overlay: AtomicBool::new(true),
        }
    }
}