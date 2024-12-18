use aoc_2024_17::*;

static mut OUTPUT_BUFFER: [u8; PROGRAM_LEN + 1] = [b','; PROGRAM_LEN + 1];

fn main() {
    let (a, prog) = parse(&aoc::read_input_to_string());
    println!("Part 1: {}", p1(a, &prog, unsafe{&mut OUTPUT_BUFFER}));
    println!("Part 2: {}", p2(&prog));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let (a, prog) = parse(&aoc::read_input_to_string());
        let mut out = [b','; PROGRAM_LEN + 1];
        assert_eq!(p1(a, &prog, &mut out), "2,1,0,4,6,2,4,2,0");
    }

    #[test]
    fn test_p2() {
        let (_, prog) = parse(&aoc::read_input_to_string());
        assert_eq!(p2(&prog), 109685330781408);
    }
}