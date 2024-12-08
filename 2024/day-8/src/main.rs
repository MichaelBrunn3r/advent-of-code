use aoc_2024_8::*;

static mut NODE_LOCATIONS: NodeLocations = unsafe{std::mem::zeroed()};

fn main() {
    parse(&aoc::read_input_to_string(), unsafe{&mut NODE_LOCATIONS});
    println!("Part 1&2: {:?}", p(unsafe{&NODE_LOCATIONS}));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p() {
        parse(&aoc::read_input_to_string(), unsafe{&mut NODE_LOCATIONS});
        assert_eq!(p(unsafe{&NODE_LOCATIONS}), (293, 934));
    }
}