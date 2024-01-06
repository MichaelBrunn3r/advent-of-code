use aoc::U8PtrExt;

// distance = hold_time * (time - hold_time)
// -> distance_to_beat < hold_time * (time - hold_time)

// d = h * (t - h) (h=1.7)
// d/h = t-h
// d/h + h = t
// d + x^2 = th
// h^2 - th + d = 0

pub fn part_1(input: &str) -> usize {
    let mut data = input.as_ptr();
    unsafe {
        data = data.add("Time:   ".len());

        let mut times = [0; 4];
        times.iter_mut().take(4).for_each(|time| {
            data = data.add("     ".len());
            *time = data.parse_ascii_digits(2);
        });

        data = data.add("\nDistance".len());

        (0..4)
            .map(|_| {
                data = data.add("   ".len());
                let num_digits = get_num_distance_digits(&mut data);
                data.parse_ascii_digits(num_digits)
            })
            .zip(times.iter())
            .map(|(distance, time)| {
                let (max_hold_time, min_hold_time) =
                    quadratic_formula(1.0, -(*time as f64), distance as f64);

                (
                    (max_hold_time).floor() as usize,
                    (min_hold_time).ceil() as usize,
                )
            })
            .map(|(max_hold_time, min_hold_time)| max_hold_time - min_hold_time + 1)
            .reduce(|acc, x| acc * x)
            .unwrap()
    }
}

pub fn part_2(input: &str) -> usize {
    let mut data = input.as_ptr();
    unsafe {
        data = data.add("Time:   ".len());

        let mut time = 0usize;
        for _ in 0..4 {
            data = data.add("     ".len());
            for _ in 0..2 {
                time *= 10;
                time += (data.read() - b'0') as usize;
                data = data.add(1);
            }
        }

        data = data.add("\nDistance".len());

        let mut distance = 0usize;
        for _ in 0..4 {
            data = data.add("   ".len());
            let num_digits = get_num_distance_digits(&mut data);
            for _ in 0..num_digits {
                distance *= 10;
                distance += (data.read() - b'0') as usize;
                data = data.add(1);
            }
        }

        let (max_hold_time, min_hold_time) =
            quadratic_formula(1.0, -(time as f64), distance as f64);

        ((max_hold_time).floor() as usize) - ((min_hold_time).ceil() as usize) + 1
    }
}

fn quadratic_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
    let root = (b.powf(2.0) - 4.0 * a * c).sqrt();
    let x1 = (-b + root) / 2.0 * a;
    let x2 = (-b - root) / 2.0 * a;

    (x1, x2)
}

unsafe fn get_num_distance_digits(data: &mut *const u8) -> usize {
    if data.read() != b' ' {
        4
    } else {
        *data = data.add(1);
        3
    }
}
