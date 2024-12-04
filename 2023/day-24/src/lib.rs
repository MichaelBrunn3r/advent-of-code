use aoc::{prelude::*, ConstVec, Vec3};

const BOUNDS: [f64; 2] = [200000000000000.0, 400000000000000.0];
static mut HAIL_STONES: ConstVec<Hail, 300> = unsafe { std::mem::zeroed() };

pub fn parse(input: &str) -> &[Hail] {
    let mut data = input.as_ptr();
    unsafe {
        HAIL_STONES.clear();
        while data.read() != b'\n' {
            HAIL_STONES.push(parse_hailstone(&mut data));
        }
        HAIL_STONES.as_ref()
    }
}

pub fn p1(hailstones: &[Hail]) -> usize {
    let mut interactions = 0;
    for i in 0..hailstones.len() {
        let a = &hailstones[i];
        for b in &hailstones[i + 1..hailstones.len()] {
            let x = (a.y_intercept - b.y_intercept) / (b.slope - a.slope);
            let y = a.slope * x + a.y_intercept;
            let t_a = (x - a.pos.x) / a.vel.x;
            let t_b = (x - b.pos.x) / b.vel.x;

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

pub fn p2(hailstones: &[Hail]) -> usize {
    let p2 = hailstones[1].pos.sub(&hailstones[0].pos);
    let p3 = hailstones[2].pos.sub(&hailstones[0].pos);
    let v2 = hailstones[1].vel.sub(&hailstones[0].vel);
    let v3 = hailstones[2].vel.sub(&hailstones[0].vel);

    println!("p2: {:?}", p2);

    0
}

#[derive(Debug)]
pub struct Hail {
    pos: Vec3<f64>,
    vel: Vec3<f64>,
    y_intercept: f64,
    slope: f64,
}

fn parse_hailstone(data: &mut *const u8) -> Hail {
    unsafe {
        let pos: [f64; 3] = data.parse_n_uints::<f64, 3, 14>(", ".len());
        *data = data.add(" @ ".len());

        let vel: [f64; 3] = data.parse_n_ints::<f64, 3, 1>(", ".len());
        *data = data.add("\n".len());

        let slope = vel[1] / vel[0];
        Hail {
            pos: Vec3::new(pos[0], pos[1], pos[2]),
            vel: Vec3::new(vel[0], vel[1], vel[2]),
            y_intercept: pos[1] - slope * pos[0],
            slope,
        }
    }
}
