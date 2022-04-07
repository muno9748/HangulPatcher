use sejong::{Buffer as BufferInner};

struct UnsafeBuffer(pub Vec<u32>); // mem::size_of::<u32>() == mem::size_of::<Syllable>()

#[inline(always)]
fn map_byte(b: u8, shift: bool) -> u8 {
    if shift {
        match b {
            0x65 | 0x6f | 0x70 | 0x71 | 0x72 | 0x74 | 0x77 => b,
            _ => b + 32
        }
    } else {
        b + 32
    }
}

pub struct Buffer {
    buf: BufferInner
}

impl Buffer {
    pub fn new() -> Self {
        Buffer { buf: BufferInner::with_capacity(2) }
    }

    pub fn push(&mut self, byte: u8, shift: bool) {
        self.buf.put(map_byte(byte, shift));
    }

    pub fn pop(&mut self) {
        self.buf.pop();
    }

    pub fn dequeue_syllable(&mut self) {
        unsafe {
            let buf = &mut *(&mut self.buf as *mut BufferInner as *mut UnsafeBuffer);

            buf.0.remove(0);
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            let buf = &mut *(&mut self.buf as *mut BufferInner as *mut UnsafeBuffer);

            buf.0.clear(); // this has better performance than self.buf.out();
        }
    }

    pub fn assemble(&self) -> (u16, Option<u16>) {
        let utf16 = self.buf.to_string();
        println!("{}", utf16);
        
        let mut utf16 = utf16.encode_utf16();
        println!("{:?}", utf16);

        (utf16.next().unwrap(), utf16.next())
    }

    pub fn strlen(&self) -> usize {
        unsafe {
            let buf = &*(&self.buf as *const BufferInner as *const UnsafeBuffer);

            buf.0.len()
        }
    }
}