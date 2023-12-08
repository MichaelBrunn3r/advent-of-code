use gcd::Gcd;

pub trait UnsignedExt {
    fn lcm(self, other: Self) -> Self;
}

impl UnsignedExt for usize {
    fn lcm(self, other: Self) -> Self {
        self * other / self.gcd(other)
    }
}
