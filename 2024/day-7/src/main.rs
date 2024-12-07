use aoc_2024_7::*;

fn main() {
    let input = aoc::read_input_to_string();
    let lines = parse(&input);
    println!("Part 1&2: {:?}", p(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let lines = parse(&aoc::read_input_to_string());
        assert_eq!(p(&lines), (663613490587, 110365987435001));
    }
}