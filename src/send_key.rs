use std::{mem, thread};
use winapi::ctypes::c_int;
use winapi::um::winuser::{
    MapVirtualKeyW, SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP,
    KEYEVENTF_SCANCODE, KEYEVENTF_UNICODE, LPINPUT,
};

unsafe fn keybd_event(flags: u32, scan: u16) {
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: mem::transmute_copy(&KEYBDINPUT {
            wVk: 0,
            wScan: scan,
            dwFlags: flags,
            time: 0,
            dwExtraInfo: 0,
        }),
    };

    SendInput(1, &mut input as LPINPUT, mem::size_of::<INPUT>() as c_int);
}

pub fn send_key(key: &str) {
    let key = String::from(key);
    thread::spawn(move || {
        let mut buffer = [0; 2];
        for c in key.as_str().chars() {
            let result = c.encode_utf16(&mut buffer);
            unsafe {
                if result.len() == 1 {
                    keybd_event(KEYEVENTF_UNICODE, result[0]);
                    keybd_event(KEYEVENTF_UNICODE | KEYEVENTF_KEYUP, result[0]);
                } else {
                    for utf16 in result {
                        keybd_event(KEYEVENTF_UNICODE, utf16.clone());
                    }
                }
            }
        }
    });
}

pub unsafe fn send_back() {
    let scancode = MapVirtualKeyW(8, 0) as u16;

    keybd_event(KEYEVENTF_SCANCODE, scancode);
    keybd_event(KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP, scancode);
}
