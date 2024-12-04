use std::fmt::{Debug, Formatter};

static mut GRAPH: [ConstVec<u16, 8>; 1 << 16] = unsafe { std::mem::zeroed() };

pub fn p1(input: &str) -> usize {
    let mut data = input.as_ptr();
    unsafe {
        GRAPH = std::mem::zeroed();

        while data.read() != b'\0' {
            let from = encode(data) as usize;
            data = data.add("abc:".len());

            let to = &mut GRAPH[from];
            let mut idx = 0;
            while data.read() == b' ' {
                data = data.add(" ".len());
                to.insert_as_last(idx, encode(data));
                data = data.add("abc".len());
                idx += 1;
            }
            data = data.add("\n".len());

            // println!("{:?} -> {:?}", decode(from as u16).as_str_unchecked(), to);
        }
    }
    0
}

pub fn p2(input: &str) -> usize {
    0
}

fn encode(mut label: *const u8) -> u16 {
    unsafe {
        let mut n = 0;
        for _ in 0..3 {
            n <<= 5;
            n |= (label.read() - b'a') as u16;
            label = label.add(1);
        }
        n
    }
}

fn decode(mut label: u16) -> [u8; 3] {
    let mut s = [b'?'; 3];
    s[2] = (label & 0b11111) as u8 + b'a';
    for i in (0..2).rev() {
        label >>= 5;
        s[i] = (label & 0b11111) as u8 + b'a';
    }
    s
}

struct ConstVec<T, const C: usize> {
    data: [T; C],
    len: u8,
}

impl<T, const C: usize> ConstVec<T, C> {
    const fn new() -> Self {
        Self {
            data: unsafe { std::mem::zeroed() },
            len: 0,
        }
    }

    fn push(&mut self, value: T) {
        self.data[self.len as usize] = value;
        self.len += 1;
    }

    fn clear(&mut self) {
        self.len = 0;
    }

    fn insert_as_last(&mut self, idx: u8, value: T) {
        self.data[idx as usize] = value;
        self.len = idx + 1;
    }
}

impl Debug for ConstVec<u16, 8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.data[..self.len as usize].iter())
            .finish()
    }
}
