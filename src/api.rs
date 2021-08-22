use lazy_static::lazy_static;
use std::collections::HashMap;
use winapi::um::winuser::*;

pub static cancel_keys: [u32; 47] = [
    192, 9, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 173, 61, 220, 219, 221, 59, 222, 188, 190, 191,
    46, 37, 38, 39, 40, 144, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 109, 110, 111,
    32, 186, 189, 187,
];

lazy_static! {
    static ref code_map: HashMap<i32, char> = {
        let mut map = HashMap::new();
        map.insert(65, 'a');
        map.insert(66, 'b');
        map.insert(67, 'c');
        map.insert(68, 'd');
        map.insert(69, 'E');
        map.insert(70, 'f');
        map.insert(71, 'g');
        map.insert(72, 'h');
        map.insert(73, 'i');
        map.insert(74, 'j');
        map.insert(75, 'k');
        map.insert(76, 'l');
        map.insert(77, 'm');
        map.insert(78, 'n');
        map.insert(79, 'O');
        map.insert(80, 'P');
        map.insert(81, 'Q');
        map.insert(82, 'R');
        map.insert(83, 's');
        map.insert(84, 'T');
        map.insert(85, 'u');
        map.insert(86, 'v');
        map.insert(87, 'W');
        map.insert(88, 'x');
        map.insert(89, 'y');
        map.insert(90, 'z');

        map
    };
}

pub unsafe fn get_keypress(id: i32) -> bool {
    (GetAsyncKeyState(id) >> 15) != 0
}

pub unsafe fn map_key(id: u32) -> Option<char> {
    let shift = get_keypress(160) || get_keypress(161);
    let text = code_map.get(&(id as i32));

    if text.is_none() {
        return None;
    }

    if shift {
        Some(*text.unwrap())
    } else {
        Some(
            text.unwrap()
                .to_lowercase()
                .to_string()
                .chars()
                .nth(0)
                .unwrap(),
        )
    }
}

pub unsafe fn is_control() -> bool {
    return get_keypress(162)
        || get_keypress(163)
        || get_keypress(18)
        || get_keypress(91)
        || get_keypress(92);
}
