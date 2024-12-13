use aoc_2024_13::*;

static mut MACHINES: Machines = unsafe{std::mem::zeroed()};

fn main() {
    unsafe{
        parse(&aoc::read_input_to_string(), &mut MACHINES);
        println!("Part 1: {}", p1(&MACHINES));
        println!("Part 2: {}", p2(&MACHINES));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let mut machines: Machines = unsafe{std::mem::zeroed()};
        parse(&aoc::read_input_to_string(), &mut machines);
        assert_eq!(p1(&machines), 29187);
    }

    #[test]
    fn test_p2() {
        let mut machines: Machines = unsafe{std::mem::zeroed()};
        parse(&aoc::read_input_to_string(), &mut machines);
        assert_eq!(p2(&machines), 99968222587852);
    }
}