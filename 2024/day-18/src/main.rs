use aoc_2024_18::*;

fn main() {
    let bytes = parse(&aoc::read_input_to_string(), unsafe { &mut GRID });
    println!("Part 1&2: {:?}", p(&bytes, unsafe { &mut GRID }));
}

#[cfg(test)]
mod tests {
    use aoc::xy;
    use super::*;

    #[test]
    fn test_p2() {
        let bytes = parse(&aoc::read_input_to_string(), unsafe { &mut GRID });
        assert_eq!(p(&bytes, unsafe { &mut GRID }), (276, xy(60usize, 37usize)));
    }
}
