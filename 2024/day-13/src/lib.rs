use aoc::{prelude::*, XY};
use itertools::Itertools;

const NUM_MACHINES: usize = 320;
pub type Machines = [(XY<i64, i64>, XY<i64, i64>, XY<i64, i64>); NUM_MACHINES];

pub fn p(input: &str) -> (usize, usize) {
    let mut tokens = 0;
    let mut tokens_10B = 0;

    let mut crs = input.as_ptr();
    for _ in 0..NUM_MACHINES {
        let [a,b,p] = parse_machine(&mut crs);

        let t =  min_tokens(a, b, p);
        tokens += t;
        if t == 0 {
            tokens_10B += min_tokens(a,b , p + 10000000000000);
        }
    }

    (tokens, tokens_10B)
}

fn min_tokens(a: XY<i64, i64>, b: XY<i64, i64>, p: XY<i64, i64>) -> usize {
    let det = a.x * b.y - b.x * a.y;
    let det_a = p.x * b.y - b.x * p.y;
    let det_b = a.x * p.y - p.x * a.y;

    if det_a % det != 0 || det_b % det != 0 {
        0
    } else {
        let presses_a = det_a / det;
        let presses_b = det_b / det;
        (3 * presses_a + presses_b) as usize
    }
}

fn parse_machine(crs: &mut *const u8) -> [XY<i64,i64>; 3] {
    crs.skip("Button A: X+".len());
    let ax = crs.parse_uint::<i64, 2>();
    let ay = crs.skip(4).parse_uint::<i64, 2>();
    crs.skip("\nButton B: X+".len());

    let bx = crs.parse_uint::<i64, 2>();
    let by = crs.skip(4).parse_uint::<i64, 2>();
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

    [xy(ax, ay),xy(bx, by),xy(prize_x, prize_y)]
}