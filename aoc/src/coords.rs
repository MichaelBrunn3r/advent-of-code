#[derive(Debug)]
pub struct Vec3<T: std::ops::Sub<Output = T> + Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: std::ops::Sub<Output = T> + Copy> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
