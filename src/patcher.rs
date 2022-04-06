use crate::win32::{self as os, HookResult};
use crate::win_overlay::Overlay;

use crate::korean::{self, MappedChar};
use crate::history_type::HistoryType;
use crate::char_type::CharType;
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
    prev: Option<&'static MappedChar>,
    pub history_type: HistoryType,
    pub settings: &'static Settings,
    history: Vec<u8>,
    buffer: [u8; 4],
    pub mode: CharType,
}

impl Patcher {
    pub fn new(settings: &'static Settings) -> Self {
        Patcher {
            history_type: HistoryType::Null,
            history: Vec::with_capacity(5),
            mode: CharType::Chosung,
            buffer: [0; 4],
            prev: None,
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

            if IS_MC_FULLSCREEN.load(Ordering::SeqCst) {
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

    #[inline(always)]
    fn handler(&mut self, key: u8, shift: bool)  {
        match (&self.mode, shift) {
            (CharType::Chosung, false) => {
                let data = korean::KOR_MAP.get(&key).unwrap();

                if data.0 == CharType::Chosung {
                    self.prev = Some(data);
                    self.mode = CharType::Jungsung;
                    self.buffer[0] = data.5;

                    os::send_key(false, korean::utf16_jongsung(data.5));
                    
                    self.history_type = HistoryType::A;

                    self.history.clear();
                    self.history.push(data.5);
                } else {
                    if let Some(prev) = self.prev.take() {
                        if prev.3 & data.4 != 0 {
                            self.history_type = HistoryType::Bb;
    
                            self.history.push(data.5);

                            os::send_key(true, korean::utf16_jungsung(korean::assemble_jungsung(prev.5, data.5)));

                            return
                        }

                        self.handler(key, false);
                    } else {
                        self.history_type = HistoryType::B;
                        self.prev = Some(data);

                        self.history.clear();
                        self.history.push(data.5);

                        os::send_key(false, korean::utf16_jungsung(data.5));
                    }
                }
            }
            (CharType::Chosung, true) => {
                let data = korean::KOR_MAP_SHIFT.get(&key).unwrap();

                if !data.1 {
                    self.handler(key, false);

                    return
                }

                if data.0 == CharType::Chosung {
                    self.mode = CharType::Jungsung;
                    self.buffer[0] = data.2;

                    os::send_key(false, korean::utf16_jongsung(data.2));
                    
                    self.history_type = HistoryType::A;
                    
                    self.history.clear();
                    self.history.push(data.2);
                } else {
                    os::send_key(false, korean::utf16_jungsung(data.2));
                    
                    self.history_type = HistoryType::B;

                    self.history.clear();
                    self.history.push(data.2);
                }
            }
            (CharType::Jungsung, false) => {
                let data = korean::KOR_MAP.get(&key).unwrap();

                if data.0 == CharType::Chosung {
                    if let Some(prev) = self.prev.take() {
                        if prev.1 & data.2 != 0 {
                            let jongsung = korean::assemble_jongsung(prev.5, data.5);
                            self.buffer[0] = jongsung;

                            os::send_key(true, korean::utf16_jongsung(jongsung));

                            self.history_type = HistoryType::Cc;

                            self.history.push(data.5);

                            return
                        }
                    }

                    self.mode = CharType::Chosung;
                    self.handler(key, false);
                } else {
                    let mut bs = true;

                    if self.history_type == HistoryType::Cc {
                        self.buffer[0] = self.history[self.history.len() - 1];

                        bs = false;
                        os::send_key(true, korean::utf16_jongsung(self.history[self.history.len() - 2]));
                    }

                    self.mode = CharType::Jongsung;
                    self.prev = Some(data);
                    self.buffer[1] = data.5;

                    os::send_key(bs, korean::assemble(self.buffer[0], data.5, 0));

                    self.history_type = HistoryType::Ab;

                    self.history.push(data.5);
                }
            }
            (CharType::Jungsung, true) => {
                let data = korean::KOR_MAP_SHIFT.get(&key).unwrap();

                if !data.1 {
                    self.handler(key, false);

                    return
                }

                if data.0 == CharType::Chosung {
                    self.mode = CharType::Chosung;
                    self.prev = None;

                    self.handler(key, true);
                } else {
                    self.mode = CharType::Jongsung;
                    self.prev = None;
                    self.buffer[1] = data.2;

                    os::send_key(true, korean::assemble(self.buffer[0], data.2, 0));

                    self.history_type = HistoryType::Ab;

                    self.history.push(data.2);
                }
            }
            (CharType::Jongsung, false) => {
                let data = korean::KOR_MAP.get(&key).unwrap();

                if data.0 == CharType::Jungsung {
                    if let Some(prev) = self.prev.take() {
                        if prev.3 & data.4 != 0 {
                            let jungsung = korean::assemble_jungsung(prev.5, data.5);
                            self.buffer[1] = jungsung;

                            os::send_key(true, korean::assemble(self.buffer[0], self.buffer[1], 0));

                            self.history_type = HistoryType::Abb;

                            self.history.push(data.5);

                            return
                        }
                    }

                    self.mode = CharType::Chosung;
                    self.handler(key, false);
                } else {
                    self.mode = CharType::JongsungFinal;
                    self.prev = Some(data);
                    self.buffer[2] = data.5;

                    os::send_key(true, korean::assemble(self.buffer[0], self.buffer[1], data.5 + 1));

                    self.history_type = match self.history_type {
                        HistoryType::Ab => HistoryType::Abc,
                        HistoryType::Abb => HistoryType::Abbc,
                        _ => unreachable!()
                    };

                    self.history.push(data.5);
                }
            }
            (CharType::Jongsung, true) => {
                let data = korean::KOR_MAP_SHIFT.get(&key).unwrap();

                if !data.1 {
                    self.handler(key, false);

                    return
                }

                if data.0 == CharType::Jungsung {
                    self.mode = CharType::Chosung;
                    self.handler(key, true);
                } else {
                    self.mode = CharType::JongsungFinal;
                    self.prev = None;
                    self.buffer[2] = data.2;

                    os::send_key(true, korean::assemble(self.buffer[0], self.buffer[1], data.2 + 1));

                    self.history_type = match self.history_type {
                        HistoryType::Ab => HistoryType::Abc,
                        HistoryType::Abb => HistoryType::Abbc,
                        _ => unreachable!()
                    };

                    self.history.push(data.2);
                }
            }
            (CharType::JongsungFinal, false) => {
                let data = korean::KOR_MAP.get(&key).unwrap();

                if data.0 == CharType::Chosung {
                    if let Some(prev) = self.prev.take() {
                        if prev.1 & data.2 != 0 {
                            let jongsung = korean::assemble_jongsung(prev.5, data.5);
                            self.buffer[2] = jongsung;
                            self.buffer[3] = prev.5;
                            self.prev = Some(data);

                            os::send_key(true, korean::assemble(self.buffer[0], self.buffer[1], jongsung + 1));
                            
                            self.history_type = match self.history_type {
                                HistoryType::Abc => HistoryType::Abcc,
                                HistoryType::Abbc => HistoryType::Abbcc,
                                _ => unreachable!()
                            };

                            self.history.push(data.5);

                            return
                        }
                    }

                    self.mode = CharType::Chosung;
                    self.handler(key, false);
                } else {
                    if self.prev.is_some() && self.buffer[2] != self.prev.unwrap().5 {
                        os::send_key(true, korean::assemble(self.buffer[0], self.buffer[1], self.buffer[3] + 1));

                        self.buffer[0] = self.prev.unwrap().5;
                        self.mode = CharType::Jongsung;
                        self.prev = Some(data);
                        self.buffer[1] = data.5;
                        
                        os::send_key(false, korean::assemble(self.buffer[0], data.5, 0));

                        self.history_type = HistoryType::Ab;

                        self.history.clear();
                        self.history.push(self.buffer[0]);
                        self.history.push(data.5);
                    } else {
                        os::send_key(true, korean::assemble(self.buffer[0], self.buffer[1], 0));
                        
                        self.buffer[0] = self.buffer[2];
                        self.mode = CharType::Jongsung;
                        self.prev = Some(data);
                        self.buffer[1] = data.5;
    
                        os::send_key(false, korean::assemble(self.buffer[0], data.5, 0));

                        self.history_type = HistoryType::Ab;

                        self.history.clear();
                        self.history.push(self.buffer[0]);
                        self.history.push(data.5);
                    }
                }
            }
            (CharType::JongsungFinal, true) => {
                let data = korean::KOR_MAP_SHIFT.get(&key).unwrap();

                if !data.1 {
                    self.handler(key, false);

                    return
                }

                if data.0 == CharType::Chosung {
                    self.mode = CharType::Chosung;
                    self.handler(key, false);
                } else {
                    os::send_key(true, korean::assemble(self.buffer[0], self.buffer[1], 0));
                    
                    self.buffer[0] = self.buffer[2];
                    self.mode = CharType::Jongsung;
                    self.prev = None;
                    self.buffer[1] = data.2;

                    os::send_key(false, korean::assemble(self.buffer[0], data.2, 0));

                    self.history_type = HistoryType::Ab;

                    self.history.clear();
                    self.history.push(self.buffer[0]);
                    self.history.push(data.2);
                }
            }
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

            if !os::is_ingame() {
                return HookResult::Pass
            }
    
            if key == keys::F11 {
                IS_MC_FULLSCREEN.store(!IS_MC_FULLSCREEN.load(Ordering::SeqCst), Ordering::SeqCst);
                
                patcher.mode = CharType::Chosung;
                patcher.buffer = [0; 4];
    
                patcher.history_type = HistoryType::Null;
    
                if patcher.settings.show_overlay.load(Ordering::Relaxed) {
                    os::toggle_fullscreen_custom();

                    return HookResult::Block
                }
    
                return HookResult::Pass
            }
    
            if patcher.settings.block_kr_toggle_ingame.load(Ordering::Relaxed) && key == keys::KoreanToggle {
                if patcher.settings.korean_toggle_key.load(Ordering::Relaxed) == keys::KoreanToggle {
                    if !chat_opened!() {
                        return HookResult::Block
                    }
                } else {
                    return HookResult::Block
                }
            }
    
            if !chat_opened!() && (key == patcher.settings.chat_open_key.load(Ordering::Relaxed) || key == patcher.settings.cmd_open_key.load(Ordering::Relaxed)) {
                chat_opened!(true);
    
                HookResult::Pass
            } else if chat_opened!() {
                if key == patcher.settings.korean_toggle_key.load(Ordering::Relaxed) {
                    korean_mode!(!korean_mode!());
        
                    if korean_mode!() == false {
                        patcher.mode = CharType::Chosung;
                        patcher.buffer = [0; 4];
    
                        patcher.history_type = HistoryType::Null;
                    }
        
                    if patcher.settings.korean_toggle_key.load(Ordering::Relaxed) != keys::KoreanToggle {
                        return HookResult::Pass
                    } else {
                        return HookResult::Block
                    }
                }
                
                if os::ctrl_pressed() {
                    match key {
                        keys::V | keys::A => {
                            patcher.mode = CharType::Chosung;
                            patcher.buffer = [0; 4];
    
                            patcher.history_type = HistoryType::Null;
                        }
                        _ => ()
                    }
                    
                    return HookResult::Pass
                }
        
                if key == keys::Enter || key == keys::ESC {
                    patcher.mode = CharType::Chosung;
                    patcher.buffer = [0; 4];
                    chat_opened!(false);
                    
                    patcher.history_type = HistoryType::Null;
                    
                    if patcher.settings.auto_disable_korean.load(Ordering::Relaxed) {
                        korean_mode!(false);
                    }
                    
                    return HookResult::Pass
                } else if keys::is_arrow(key) {
                    patcher.mode = CharType::Chosung;
                    patcher.buffer = [0; 4];
                    
                    patcher.history_type = HistoryType::Null;
                    
                    return HookResult::Pass
                }
        
                if korean_mode!() {
                    if korean::is_mappable(key) {
                        patcher.handler(key, os::is_shift());
            
                        HookResult::Block
                    } else if key == keys::Backspace {
                        if patcher.history_type == HistoryType::Null {
                            HookResult::Pass
                        } else {
                            patcher.history.pop();
    
                            let mut ret = HookResult::Block;
    
                            patcher.history_type = match patcher.history_type {
                                HistoryType::Cc | HistoryType::Ab => {
                                    let id = patcher.history[0];
    
                                    os::send_key(true, korean::utf16_jongsung(id));
                                    
                                    patcher.mode = CharType::Jungsung;
                                    patcher.buffer[0] = id;
                                    
                                    patcher.prev = Some(korean::KOR_MAP.values().find(|data| data.0 == CharType::Chosung && data.5 == id).unwrap());
                                    
                                    HistoryType::A
                                }
                                HistoryType::Bb => {
                                    let id = patcher.history[0];
    
                                    os::send_key(true, korean::utf16_jongsung(id));
                                    
                                    patcher.mode = CharType::Chosung;
                                    patcher.buffer[0] = id;
    
                                    patcher.prev = Some(korean::KOR_MAP.values().find(|data| data.0 == CharType::Jungsung && data.5 == id).unwrap());
    
                                    HistoryType::B
                                }
                                HistoryType::Abb | HistoryType::Abc => {
                                    let id0 = patcher.history[0];
                                    let id1 = patcher.history[1];
    
                                    os::send_key(true, korean::assemble(id0, id1, 0));
                                    
                                    patcher.mode = CharType::Jongsung;
                                    patcher.buffer[0] = id0;
                                    patcher.buffer[1] = id1;
    
                                    patcher.prev = Some(korean::KOR_MAP.values().find(|data| data.0 == CharType::Jungsung && data.5 == id1).unwrap());
    
                                    HistoryType::Ab
                                }
                                HistoryType::Abbc => {
                                    let id0 = patcher.history[0];
                                    let id1 = korean::assemble_jungsung(patcher.history[1], patcher.history[2]);
    
                                    os::send_key(true, korean::assemble(id0, id1, 0));
                                    
                                    patcher.mode = CharType::Jongsung;
                                    patcher.buffer[0] = id0;
                                    patcher.buffer[1] = id1;
    
                                    patcher.prev = Some(korean::KOR_MAP.values().find(|data| data.0 == CharType::Jungsung && data.5 == id1).unwrap());
    
                                    HistoryType::Abb
                                }
                                HistoryType::Abcc => {
                                    os::send_key(true, korean::assemble(patcher.history[0], patcher.history[1], patcher.history[2] + 1));
                                    
                                    patcher.mode = CharType::JongsungFinal;
                                    patcher.buffer[2] = patcher.history[2];
    
                                    patcher.prev = Some(korean::KOR_MAP.values().find(|data| data.0 == CharType::Chosung && data.5 == patcher.history[2]).unwrap());
    
                                    HistoryType::Abc
                                }
                                HistoryType::Abbcc => {
                                    let id1 = korean::assemble_jungsung(patcher.history[1], patcher.history[2]);
    
                                    os::send_key(true, korean::assemble(patcher.history[0], id1, patcher.history[3] + 1));
                                    
                                    patcher.mode = CharType::JongsungFinal;
                                    patcher.prev = Some(korean::KOR_MAP.values().find(|data| data.0 == CharType::Chosung && data.5 == patcher.history[3]).unwrap());
                                   
                                    HistoryType::Abbc
                                }
                                _ => {
                                    ret = HookResult::Pass;
                                    HistoryType::Null
                                }
                            };
    
                            ret
                        }
                    } else {
                        if !keys::is_control(key) {
                            patcher.mode = CharType::Chosung;
                            patcher.buffer = [0; 4];
                            
                            patcher.history_type = HistoryType::Null;
                        }
        
                        HookResult::Pass
                    }
                } else {
                    HookResult::Pass
                }
            } else {
                HookResult::Pass
            }
        })
    }
}
