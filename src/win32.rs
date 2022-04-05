use winapi::{
    shared::{
        minwindef::HINSTANCE, 
        windef::{HWND, HHOOK, RECT}
    },
    um::{winuser::{self as user}, processthreadsapi::GetCurrentThreadId},
    ctypes::c_int,
};
use std::{
    sync::atomic::{AtomicUsize, AtomicPtr, Ordering, AtomicU32},
    cell::RefCell, mem, ptr, rc::Rc,
};

thread_local! {
    static HANDLER: RefCell<Option<fn(u8) -> HookResult>> = RefCell::new(None);
}

use std::sync::Mutex;

use crate::Event;

lazy_static::lazy_static! {
    static ref HOOK: Mutex<Option<usize>> = Mutex::new(None);
}

static MC_HWND: AtomicUsize = AtomicUsize::new(0);

pub enum HookResult {
    Block,
    Pass
}

pub static LOOP_ID: AtomicU32 = AtomicU32::new(0);

pub(crate) fn msg_loop(cb: impl Fn(&mut Event) -> ()) {
    use user::*;

    unsafe {
        let mut msg: MSG = std::mem::zeroed();
        
        LOOP_ID.store(GetCurrentThreadId(), Ordering::SeqCst);
    
        while GetMessageA(&mut msg, 0 as _, 0, 0) > 0 {
            if msg.message == 0x400 {
                let ptr = &mut *(msg.lParam as *mut Event);

                cb(ptr);

                std::ptr::drop_in_place(ptr);
            }

            TranslateMessage(&mut msg);
            DispatchMessageA(&mut msg);
        }
    }
}

pub(crate) fn send_msg(ev: Event) {
    unsafe {
        let ev = Box::leak(Box::new(ev));

        user::PostThreadMessageA(LOOP_ID.load(Ordering::SeqCst), 0x400, 0, ev as *mut _ as isize);
    }
}

unsafe extern "system" fn hook_fn(code: i32, w: usize, l: isize) -> isize {
    let kbd = *(l as *const user::KBDLLHOOKSTRUCT);

    if code < 0 || kbd.flags & 0x10 != 0 {
        return user::CallNextHookEx(ptr::null_mut(), code, w, l)
    }

    match w as u32 {
        user::WM_KEYDOWN => HANDLER.with(move |handler| match handler.borrow().as_ref().unwrap()(kbd.vkCode as u8) {
            HookResult::Block => 1,
            HookResult::Pass => user::CallNextHookEx(ptr::null_mut(), code, w, l)
        }),
        _ => user::CallNextHookEx(ptr::null_mut(), code, w, l),
    }
}

pub fn set_hook(func: fn(u8) -> HookResult) {
    unsafe {
        HANDLER.with(move |handler| {
            *handler.borrow_mut() = Some(func);
        });

        *HOOK.lock().unwrap() = Some(user::SetWindowsHookExA(user::WH_KEYBOARD_LL, Some(hook_fn), 0 as HINSTANCE, 0) as usize);
    }
}

pub fn del_hook() {
    unsafe {
        let hook = HOOK.lock().unwrap().take().unwrap();
        
        user::UnhookWindowsHookEx(hook as HHOOK);
    }
}

static LAST_RECT: AtomicPtr<RECT> = AtomicPtr::new(0 as _);

pub fn toggle_fullscreen_custom() {
    use user::*;
    
    unsafe {
        if !is_ingame() {
            return
        }

        if LAST_RECT.load(Ordering::Relaxed) as usize == 0 {
            let rect = Box::leak(Box::new(mem::zeroed::<RECT>()));
            let ptr = rect as *mut RECT;
        
            LAST_RECT.store(ptr, Ordering::Relaxed);
        }

        let rect = LAST_RECT.load(Ordering::Relaxed);
        let hwnd = GetForegroundWindow();
        let dw_style = GetWindowLongW(hwnd, GWL_STYLE);

        if dw_style & WS_OVERLAPPEDWINDOW as i32 != 0 {
            GetWindowRect(hwnd, rect);

            let mut mi = MONITORINFO {
                cbSize: mem::size_of::<MONITORINFO>() as u32,
                rcMonitor: mem::zeroed(),
                rcWork: mem::zeroed(),
                dwFlags: 0,
            };

            if GetMonitorInfoW(MonitorFromWindow(hwnd, MONITOR_DEFAULTTOPRIMARY), &mut mi) != 0 {
                let w = mi.rcMonitor.right - mi.rcMonitor.left;
                let h = mi.rcMonitor.bottom - mi.rcMonitor.top;

                ClipCursor(&mi.rcMonitor);
                SetWindowLongW(hwnd, GWL_STYLE, dw_style & !WS_OVERLAPPEDWINDOW as i32);
                SetWindowLongW(hwnd, GWL_EXSTYLE, GetWindowLongW(hwnd, GWL_EXSTYLE) & !WS_EX_TOPMOST as i32);
                SetCursorPos(w / 2, h / 2);
                SetWindowPos(hwnd, HWND_TOP, mi.rcMonitor.left, mi.rcMonitor.top, w, h, SWP_NOOWNERZORDER | SWP_FRAMECHANGED);
                SetForegroundWindow(hwnd);
            }
        } else {
            let rect = &*rect;
            
            SetWindowLongW(hwnd, GWL_STYLE, dw_style | WS_OVERLAPPEDWINDOW as i32);
            SetWindowLongW(hwnd, GWL_EXSTYLE, GetWindowLongW(hwnd, GWL_EXSTYLE) & !WS_EX_TOPMOST as i32);
            SetWindowPos(hwnd, HWND_TOP, rect.left, rect.top, rect.right - rect.left, rect.bottom - rect.top, SWP_NOOWNERZORDER | SWP_FRAMECHANGED);
        }
    }
}

pub fn find_and_toggle_fullscreen_custom() {
    use user::*;

    unsafe {
        let hwnd = get_mc_hwnd();

        if hwnd.is_none() {
            return;
        }

        let hwnd = hwnd.unwrap_unchecked();

        if LAST_RECT.load(Ordering::Relaxed) as usize == 0 {
            let rect = Box::leak(Box::new(mem::zeroed::<RECT>()));
            let ptr = rect as *mut RECT;

            LAST_RECT.store(ptr, Ordering::Relaxed);
        }

        let rect = LAST_RECT.load(Ordering::Relaxed);
        let dw_style = GetWindowLongW(hwnd, GWL_STYLE);

        if dw_style & WS_OVERLAPPEDWINDOW as i32 != 0 {
            GetWindowRect(hwnd, rect);

            let mut mi = MONITORINFO {
                cbSize: mem::size_of::<MONITORINFO>() as u32,
                rcMonitor: mem::zeroed(),
                rcWork: mem::zeroed(),
                dwFlags: 0,
            };

            if GetMonitorInfoW(MonitorFromWindow(hwnd, MONITOR_DEFAULTTOPRIMARY), &mut mi) != 0 {
                let w = mi.rcMonitor.right - mi.rcMonitor.left;
                let h = mi.rcMonitor.bottom - mi.rcMonitor.top;

                ClipCursor(&mi.rcMonitor);
                SetWindowLongW(hwnd, GWL_STYLE, dw_style & !WS_OVERLAPPEDWINDOW as i32);
                SetWindowLongW(hwnd, GWL_EXSTYLE, GetWindowLongW(hwnd, GWL_EXSTYLE) & !WS_EX_TOPMOST as i32);
                SetCursorPos(w / 2, h / 2);
                SetWindowPos(hwnd, HWND_TOP, mi.rcMonitor.left, mi.rcMonitor.top, w, h, SWP_NOOWNERZORDER | SWP_FRAMECHANGED);
                SetForegroundWindow(hwnd);
            }
        } else {
            let rect = &*rect;

            SetWindowLongW(hwnd, GWL_STYLE, dw_style | WS_OVERLAPPEDWINDOW as i32);
            SetWindowLongW(hwnd, GWL_EXSTYLE, GetWindowLongW(hwnd, GWL_EXSTYLE) & !WS_EX_TOPMOST as i32);
            SetWindowPos(hwnd, HWND_TOP, rect.left, rect.top, rect.right - rect.left, rect.bottom - rect.top, SWP_NOOWNERZORDER | SWP_FRAMECHANGED);
        }
    }
}

pub fn send_key(backspace: bool, utf16: u16) {
    unsafe {
        if backspace {
            let scancode = user::MapVirtualKeyW(8, 0) as u16;

            keybd_event(&[
                (user::KEYEVENTF_SCANCODE, scancode),
                (user::KEYEVENTF_SCANCODE | user::KEYEVENTF_KEYUP, scancode),
                (user::KEYEVENTF_UNICODE, utf16),
                (user::KEYEVENTF_UNICODE | user::KEYEVENTF_KEYUP, utf16),
            ]);
        } else {
            keybd_event(&[
                (user::KEYEVENTF_UNICODE, utf16),
                (user::KEYEVENTF_UNICODE | user::KEYEVENTF_KEYUP, utf16)
            ]);
        }
    }
}

pub fn get_mc_hwnd() -> Option<HWND> {
    unsafe extern "system" fn enumerator(hwnd: HWND, lp: isize) -> i32 {
        if user::IsWindowVisible(hwnd) == 0 {
            return 1
        }

        let length = user::GetWindowTextLengthW(hwnd);

        if length == 0 {
            return 1
        }

        let mut buf = vec![0u16; length as usize];
        
        let textw = user::GetWindowTextW(hwnd, buf.as_mut_ptr(), length);

        if textw == 0 {
            return 1
        }

        let title = String::from_utf16(&buf[0..(textw as usize)]).unwrap();
        let result = is_title_mc(title);
        
        let mut buf = vec![0u16; 8];
        let classw = user::GetClassNameW(hwnd, buf.as_mut_ptr(), 8);

        if classw == 0 {
            return 1
        }

        let cname = String::from_utf16(&buf[0..(classw as usize)]).unwrap();
        let result = result && is_cname_mc(cname);

        if result {
            *(lp as *mut HWND) = hwnd;

            return 0
        } else {
            return 1
        }
    }

    let mut hwnd: HWND = ptr::null_mut();

    unsafe {
        user::EnumWindows(Some(enumerator), &mut hwnd as *mut HWND as isize);
    }

    if hwnd as usize == 0 {
        None
    } else {
        Some(hwnd)
    }
}

pub fn is_ingame() -> bool {
    unsafe {
        let hwnd = user::GetForegroundWindow();

        if hwnd as usize == 0 {
            return false
        }
        
        let length = user::GetWindowTextLengthW(hwnd);

        if length == 0 {
            return false
        }

        let mut buf = vec![0u16; length as usize];
        
        let textw = user::GetWindowTextW(hwnd, buf.as_mut_ptr(), length);

        if textw == 0 {
            return false
        }

        let title = String::from_utf16(&buf[0..(textw as usize)]).unwrap();
        let result = is_title_mc(title);
        
        let mut buf = vec![0u16; 8];
        let classw = user::GetClassNameW(hwnd, buf.as_mut_ptr(), 8);

        if classw == 0 {
            return false
        }

        let cname = String::from_utf16(&buf[0..(classw as usize)]).unwrap();
        let result = result && is_cname_mc(cname);

        if result {
            MC_HWND.store(hwnd as usize, Ordering::Relaxed);
        }

        result
    }
}

pub fn is_shift() -> bool {
    unsafe {
        (user::GetAsyncKeyState(160) >> 15) != 0 || (user::GetAsyncKeyState(161) >> 15) != 0
    }
}

pub fn ctrl_pressed() -> bool {
    unsafe {
        (user::GetAsyncKeyState(0xA2) >> 15) != 0 || (user::GetAsyncKeyState(0xA3) >> 15) != 0
    }
}

unsafe fn keybd_event(inputs: &[(u32, u16)]) {
    let input = inputs.iter().map(|(flags, scan)| user::INPUT {
        type_: user::INPUT_KEYBOARD,
        u: mem::transmute_copy(&user::KEYBDINPUT {
            wVk: 0,
            wScan: scan.clone(),
            dwFlags: flags.clone(),
            time: 0,
            dwExtraInfo: 0,
        }),
    }).collect::<Vec<_>>();
    let input = input.as_slice();
    let ptr = input.as_ptr();

    user::SendInput(inputs.len() as u32, ptr as user::LPINPUT, mem::size_of::<user::INPUT>() as c_int);
}

pub unsafe fn keybd_event_vk_scan(inputs: &[(u32, u16, u16)]) {
    let input = inputs.iter().map(|(flags, vk, scan)| user::INPUT {
        type_: user::INPUT_KEYBOARD,
        u: mem::transmute_copy(&user::KEYBDINPUT {
            wVk: vk.clone(),
            wScan: scan.clone(),
            dwFlags: flags.clone(),
            time: 0,
            dwExtraInfo: 0,
        }),
    }).collect::<Vec<_>>();
    let input = input.as_slice();
    let ptr = input.as_ptr();

    user::SendInput(inputs.len() as u32, ptr as user::LPINPUT, mem::size_of::<user::INPUT>() as c_int);
}

pub fn is_title_mc(title: String) -> bool {
    (title.contains("Lunar Client") || title.contains("Minecraft")) && (!title.contains("Minecraft Updater") && !title.contains("Minecraft Launcher"))
}

pub fn is_cname_mc(cname: String) -> bool {
    cname.contains("LWJGL") || cname.contains("GLFW30")
}