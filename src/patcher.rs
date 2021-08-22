use crate::api;
use crate::buf;
use crate::send_key::*;

use lazy_static::lazy_static;
use sejong::Buffer;
use std::mem::MaybeUninit;
use std::ptr;
use std::sync::Mutex;
use winapi::{
    shared::{minwindef::*, windef::*},
    um::winuser::*,
};

lazy_static! {
    static ref hangul: Mutex<Buffer> = Mutex::new(Buffer::default());
}

static mut erase: bool = false;
static mut skips: u16 = 0;
static mut lang_mode: u8 = 0u8;
static mut chat_open: u32 = 0u32;
static mut block_hangul: bool = false;
static mut keep_hangul: bool = false;
static mut is_hangul: bool = false;
static mut is_chat: bool = false;
static mut hook: Option<*mut HHOOK__> = None;

unsafe fn clear_buffer() {
    let mut buf = hangul.lock().unwrap();
    buf.out();

    erase = false;
}

pub unsafe fn wait() {
    let mut msg: MSG = MaybeUninit::zeroed().assume_init();
    GetMessageW(&mut msg, 0 as HWND, 0, 0);
}

pub unsafe fn start_hook() {
    hook = Some(SetWindowsHookExA(
        WH_KEYBOARD_LL,
        Some(handler),
        0 as HINSTANCE,
        0,
    ));
}

pub unsafe fn stop_hook() {
    if hook.is_some() {
        UnhookWindowsHookEx(hook.unwrap());

        hook = None;
        is_chat = false;
        is_hangul = false;
    }
}

pub unsafe fn set_langmode(mode: u8) {
    lang_mode = mode;
}

pub unsafe fn set_chat_open(vk_code: u32) {
    chat_open = vk_code;
}

pub unsafe fn set_hangul_block(block: bool) {
    block_hangul = block;
}

pub unsafe fn set_keep_hangul(keep: bool) {
    keep_hangul = keep;
}

unsafe fn toggle_mode(mode: u8) {
    clear_buffer();

    if mode == lang_mode {
        is_hangul = !is_hangul;
    }
}

pub unsafe extern "system" fn handler(code: i32, w_param: usize, l_param: isize) -> isize {
    let next = || CallNextHookEx(ptr::null_mut(), code, w_param, l_param);

    if w_param as u32 == WM_KEYDOWN {
        let l_param = l_param as *const KBDLLHOOKSTRUCT;
        let vk_code = (*l_param).vkCode;

        if api::is_control() {
            clear_buffer();

            return next();
        }

        if vk_code == 27 || vk_code == 13 {
            if is_chat {
                std::thread::spawn(move || unsafe {
                    clear_buffer();

                    is_chat = false;

                    if !keep_hangul {
                        is_hangul = false;
                    }
                });
            }

            return next();
        }

        if vk_code == chat_open {
            if !is_chat {
                is_chat = true;
                return next();
            }
        }

        if vk_code == 21 {
            if is_chat {
                toggle_mode(0);
                return 1;
            } else if block_hangul {
                return 1;
            }

            return next();
        } else if vk_code == 162 {
            if is_chat {
                toggle_mode(1);
            }

            return next();
        }

        if vk_code == 8 {
            std::thread::spawn(move || unsafe {
                if !is_hangul {
                    return;
                }

                if skips != 0 {
                    skips -= 1;
                    return;
                }

                let mut buf = hangul.lock().unwrap();

                buf.pop();
                skips += 1;
                send_back();
                send_key(buf.to_string().as_str());

                if buf::get_length(&mut *buf as *mut Buffer) == 0 {
                    erase = false;
                }
            });

            return next();
        }

        if api::cancel_keys.contains(&vk_code) {
            clear_buffer();

            return next();
        }

        let mapped_key = api::map_key(vk_code);

        if mapped_key.is_none() {
            return next();
        }

        if !is_hangul || !is_chat {
            return next();
        }

        std::thread::spawn(move || unsafe {
            let key = mapped_key.unwrap();
            let mut buf = hangul.lock().unwrap();
            buf.put(key);
            let text = buf.to_string();
            let text = text.as_str();
            if erase {
                skips += 1;
                send_back();
            }
            send_key(text);
            if text.len() == 6 {
                buf::remove_first(&mut *buf as *mut Buffer);
            }
            erase = true;
        });

        return 1;
    }

    next()
}
