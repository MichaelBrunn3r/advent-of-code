use std::fmt::{Debug, Formatter};

const BOUNDS: [f64; 2] = [200000000000000.0, 400000000000000.0];
static mut HAIL_STONES: ConstVec<Hail, 300> = unsafe { std::mem::zeroed() };

pub fn part_1(input: &str) -> usize {
    let mut data = input.as_ptr();
    unsafe {
        HAIL_STONES.clear();

        while data.read() != b'\n' {
            let mut pos = [0.0, 0.0, 0.0];
            for comp in &mut pos {
                while data.read().is_ascii_digit() {
                    *comp = *comp * 10.0 + (data.read() - b'0') as f64;
                    data = data.add(1);
                }

                data = data.add(", ".len());
            }

            let mut vel = [0.0, 0.0, 0.0];
            for comp in &mut vel {
                data = data.add(" ".len());
                let sign = if data.read() == b'-' {
                    data = data.add(1);
                    -1.0
                } else {
                    1.0
                };

                while data.read().is_ascii_digit() {
                    *comp = *comp * 10.0 + (data.read() - b'0') as f64;
                    data = data.add(1);
                }
                *comp *= sign;
                data = data.add(",".len());
            }

            let slope = vel[1] / vel[0];
            HAIL_STONES.push(Hail {
                pos: [pos[0], pos[1]],
                vel: [vel[0], vel[1]],
                y_intercept: pos[1] - slope * pos[0],
                slope,
            });
        }

        let mut interactions = 0;
        for i in 0..HAIL_STONES.len as usize {
            let a = &HAIL_STONES.data[i];
            for j in i + 1..HAIL_STONES.len as usize {
                let b = &HAIL_STONES.data[j];

                let x = (a.y_intercept - b.y_intercept) / (b.slope - a.slope);
                let y = a.slope * x + a.y_intercept;
                let t_a = (x - a.pos[0]) / a.vel[0];
                let t_b = (x - b.pos[0]) / b.vel[0];

                if x < BOUNDS[0]
                    || x > BOUNDS[1]
                    || y < BOUNDS[0]
                    || y > BOUNDS[1]
                    || t_a < 0.0
                    || t_b < 0.0
                {
                    continue;
                }

                interactions += 1;
            }
        }

        interactions
    }
}

pub fn part_2(input: &str) -> usize {
    0
}

#[derive(Debug)]
struct Hail {
    pos: [f64; 2],
    vel: [f64; 2],
    y_intercept: f64,
    slope: f64,
}

struct ConstVec<T, const C: usize> {
    data: [T; C],
    len: u16,
}

impl<T, const C: usize> ConstVec<T, C> {
    fn push(&mut self, value: T) {
        self.data[self.len as usize] = value;
        self.len += 1;
    }

    fn clear(&mut self) {
        self.len = 0;
    }
}

impl<T: Debug, const C: usize> Debug for ConstVec<T, C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.data[..self.len as usize].iter())
            .finish()
    }
}
