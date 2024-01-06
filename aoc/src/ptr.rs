pub trait U8PtrExt {
    unsafe fn parse_ascii_digits(&mut self, num_digits: usize) -> usize;
}

impl U8PtrExt for *const u8 {
    unsafe fn parse_ascii_digits(&mut self, num_digits: usize) -> usize {
        let mut num = (self.read() - b'0') as usize;
        *self = self.add(1);

        for _ in 1..num_digits {
            num *= 10;
            num += (self.read() - b'0') as usize;
            *self = self.add(1);
        }
        num
    }
}
