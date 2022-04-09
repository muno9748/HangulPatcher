use crate::buffer::Buffer;
use crate::win32::{self as os, HookResult};
use crate::win_overlay::Overlay;

use crate::settings::Settings;
use crate::keys;

use lazy_static::lazy_static;
use std::sync::atomic::Ordering;
use std::{
    sync::{Mutex, atomic::AtomicBool},
    cell::RefCell,
};

thread_local! {
    pub static INSTANCE: RefCell<Option<Patcher>> = RefCell::new(None);
}

macro_rules! chat_opened {
    () => {
        crate::patcher::CHAT_OPENED.load(std::sync::atomic::Ordering::Relaxed)
    };
    ($v: expr) => {
        crate::patcher::CHAT_OPENED.store($v, std::sync::atomic::Ordering::Relaxed)
    };
}

macro_rules! korean_mode {
    () => {
        crate::patcher::KOREAN_MODE.load(std::sync::atomic::Ordering::Relaxed)
    };
    ($v: expr) => {
        crate::patcher::KOREAN_MODE.store($v, std::sync::atomic::Ordering::Relaxed)
    };
}

pub(crate) use chat_opened;
pub(crate) use korean_mode;

pub static CHAT_OPENED: AtomicBool = AtomicBool::new(false);
pub static KOREAN_MODE: AtomicBool = AtomicBool::new(false);
pub static IS_MC_FULLSCREEN: AtomicBool = AtomicBool::new(false);

static mut OVERLAY: usize = 0;

pub struct Patcher {
    pub settings: &'static Settings,
    pub buf: Buffer
}

impl Patcher {
    pub fn new(settings: &'static Settings) -> Self {
        Patcher {
            buf: Buffer::new(),
            settings,
        }
    }

    pub fn run_overlay() {
        std::thread::spawn(move || unsafe {
            let mut overlay = Overlay::new();

            OVERLAY = &mut overlay as *mut _ as usize;

            overlay.run();
        });
    }

    pub fn stop_overlay() {
        unsafe {
            if OVERLAY == 0 {
                return
            }

            let overlay = &mut *(OVERLAY as *mut Overlay);

            OVERLAY = 0;

            overlay.stop();

            if IS_MC_FULLSCREEN.load(Ordering::Relaxed) {
                os::find_and_toggle_fullscreen_custom();
            }
        }
    }

    pub fn set_instance(self) {
        INSTANCE.with(|inst| {
            *inst.borrow_mut() = Some(self);
        });
    }

    pub fn start() {
        os::set_hook(Patcher::hook);
    }

    pub fn stop() {
        os::del_hook();
    }

    fn send(&mut self, backspace: bool)  {
        os::send_key(backspace, self.buf.assemble().0);
    }

    fn send_all(&mut self)  {
        let assemble = self.buf.assemble();

        os::send_key(true, assemble.0);
        os::send_key(false, unsafe { self.buf.assemble().1.unwrap_unchecked() });
    }
    
    fn handler(&mut self, key: u8) -> HookResult {
        if !os::is_ingame() {
            return HookResult::Pass
        }

        if key == keys::F11 {
            IS_MC_FULLSCREEN.store(!IS_MC_FULLSCREEN.load(Ordering::Relaxed), Ordering::Relaxed);
            
            self.buf.clear();

            if self.settings.show_overlay.load(Ordering::Relaxed) {
                os::toggle_fullscreen_custom();

                return HookResult::Block
            }

            return HookResult::Pass
        }

        if self.settings.block_kr_toggle_ingame.load(Ordering::Relaxed) && key == keys::KoreanToggle {
            if self.settings.korean_toggle_key.load(Ordering::Relaxed) == keys::KoreanToggle {
                if !chat_opened!() {
                    return HookResult::Block
                }
            } else {
                return HookResult::Block
            }
        }

        if !chat_opened!() && (key == self.settings.chat_open_key.load(Ordering::Relaxed) || key == self.settings.cmd_open_key.load(Ordering::Relaxed)) {
            chat_opened!(true);

            HookResult::Pass
        } else if chat_opened!() {
            if key == self.settings.korean_toggle_key.load(Ordering::Relaxed) {
                korean_mode!(!korean_mode!());
    
                if korean_mode!() == false {
                    self.buf.clear();
                }
    
                if self.settings.korean_toggle_key.load(Ordering::Relaxed) != keys::KoreanToggle {
                    return HookResult::Pass
                } else {
                    return HookResult::Block
                }
            }
            
            if os::ctrl_pressed() {
                match key {
                    keys::V | keys::A => self.buf.clear(),
                    _ => ()
                }
                
                return HookResult::Pass
            }
    
            if key == keys::Enter || key == keys::ESC {
                chat_opened!(false);

                self.buf.clear();
                
                if self.settings.auto_disable_korean.load(Ordering::Relaxed) {
                    korean_mode!(false);
                }
                
                return HookResult::Pass
            } else if keys::is_arrow(key) {
                self.buf.clear();
                
                return HookResult::Pass
            }
    
            if korean_mode!() {
                if keys::is_kor_mappable(key) {
                    let strlen = self.buf.strlen();

                    self.buf.push(key, os::is_shift());
                    
                    if self.buf.strlen() == 2 {
                        self.send_all();
                        self.buf.dequeue_syllable();
                    } else {
                        self.send(strlen != 0);
                    }
        
                    HookResult::Block
                } else if key == keys::Backspace {
                    if self.buf.strlen() == 0 {
                        HookResult::Pass
                    } else {
                        self.buf.pop();

                        if self.buf.strlen() == 0 {
                            HookResult::Pass
                        } else {
                            self.send(true);
                            HookResult::Block
                        }
                    }
                } else {
                    if !keys::is_control(key) {
                        self.buf.clear();
                    }
    
                    HookResult::Pass
                }
            } else {
                HookResult::Pass
            }
        } else {
            HookResult::Pass
        }
    }

    fn hook(key: u8) -> HookResult {
        lazy_static! {
            static ref MUTEX: Mutex<()> = Mutex::new(());
        }

        INSTANCE.with(|patcher| {
            let _lock = MUTEX.lock().unwrap();
        
            let mut patcher = patcher.borrow_mut();
            let patcher = patcher.as_mut().unwrap();

            patcher.handler(key)
        })
    }
}
