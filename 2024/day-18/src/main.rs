use aoc_2024_18::*;

fn main() {
    let bytes = parse(&aoc::read_input_to_string(), unsafe { &mut GRID });
    println!("Part 1: {}", p1(unsafe { &GRID }));
    println!("Part 2: {:?}", p2(&bytes, unsafe { &mut GRID }));
}

#[cfg(test)]
mod tests {
    use aoc::xy;

    use super::*;

    #[test]
    fn test_p1() {
        parse(&aoc::read_input_to_string(), unsafe { &mut GRID });
        assert_eq!(p1(unsafe { &GRID }), 276);
    }

    #[test]
    fn test_p2() {
        let bytes = parse(&aoc::read_input_to_string(), unsafe { &mut GRID });
        assert_eq!(p2(&bytes, unsafe { &mut GRID }), xy(60usize, 37usize));
    }
}
