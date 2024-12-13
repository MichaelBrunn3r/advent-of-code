use aoc::{prelude::*, XY};
use itertools::Itertools;

const NUM_MACHINES: usize = 320;
pub type Machines = [(XY<i64, i64>, XY<i64, i64>, XY<i64, i64>); NUM_MACHINES];

pub fn parse(input: &str, machines: &mut Machines) {
    let mut crs = input.as_ptr();
    for i in 0..NUM_MACHINES {
        crs.skip("Button A: X+".len());
        let a = xy(
            crs.parse_uint::<i64, 2>(),
            crs.skip(4).parse_uint::<i64, 2>(),
        );
        crs.skip("\nButton B: X+".len());

        let b = xy(
            crs.parse_uint::<i64, 2>(),
            crs.skip(4).parse_uint::<i64, 2>(),
        );
        crs.skip("\nPrize: X=".len());

        // abs. freq. digits(X): 3->12, 4->123, 5->95
        let digits_x = if unsafe { *crs.add(4) } == b',' {
            4
        } else if unsafe { *crs.add(5) } == b',' {
            5
        } else {
            3
        };
        let prize_x = crs.parse_uint_n_digits::<i64>(digits_x);
        crs.skip(", Y=".len());

        // abs. freq. digits(Y): 3->8, 4->213, 5->94
        let digits_y = if unsafe { *crs.add(5) } == b'\n' && unsafe { *crs.add(6) } == b'\n' {
            5
        } else if unsafe { *crs.add(4) } == b'\n' && unsafe { *crs.add(5) } == b'\n' {
            4
        } else {
            3
        };
        let prize_y = crs.parse_uint_n_digits::<i64>(digits_y);
        crs.skip(2);
        machines[i] = (a, b, xy(prize_x, prize_y));
    }
}

pub fn p1(machines: &Machines) -> usize {
    machines
        .iter()
        .map(|(a, b, prize)| {
            let PyBx_PxBy = prize.y * b.x - prize.x * b.y;
            let BxAy_AxBy = b.x * a.y - a.x * b.y;
            let presses_a = PyBx_PxBy / BxAy_AxBy;

            if PyBx_PxBy % BxAy_AxBy == 0 {
                let pressed_b = (prize.y - presses_a * a.y) / b.y;
                (3 * presses_a + pressed_b) as usize
            } else {
                0
            }
        })
        .sum()
}

pub fn p2(machines: &Machines) -> usize {
    machines
        .iter()
        .map(|(a, b, prize)| {
            let prize = xy(prize.x + 10000000000000, prize.y + 10000000000000);
            let PyBx_PxBy = prize.y * b.x - prize.x * b.y;
            let BxAy_AxBy = b.x * a.y - a.x * b.y;
            let presses_a = PyBx_PxBy / BxAy_AxBy;

            if PyBx_PxBy % BxAy_AxBy != 0 {
                return 0;
            }

            let presses_b = (prize.y - presses_a * a.y) / b.y;
            if (prize.y - presses_a * a.y) % b.y == 0 {
                (3 * presses_a + presses_b) as usize
            } else {
                0
            }
        })
        .sum()
}
