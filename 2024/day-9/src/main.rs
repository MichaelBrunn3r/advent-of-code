use aoc_2024_9::*;

fn main() {
    let input = aoc::read_input_to_string();
    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(&aoc::read_input_to_string()), 6288599492129);
        assert_eq!(p1(&"111\n"), 1);
        assert_eq!(p1(&"211\n"), 2);
        assert_eq!(p1(&"101\n"), 1);
        assert_eq!(p1(&"2333133121414131402\n"), 1928);
        assert_eq!(p1(&"633761367\n"), 741);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&aoc::read_input_to_string()), 6321896265143);
        // assert_eq!(p2(&"2333133121414131402\n"), 2858);
        // assert_eq!(p2(&"633761367\n"), 771);
    }
}
