use aoc::prelude::*;
use std::ops::Range;

const NUM_SEEDS: usize = 20;

pub fn part_1(input: &str) -> usize {
    let mut sections = input.split("\n\n");
    sections.next().unwrap();

    unsafe {
        let mut data = input.as_ptr();
        data = data.add("seeds: ".len());

        let seeds = parse_seeds(&mut data);

        let map_sections = parse_map_sections(&mut data);

        let mut min = usize::MAX;
        for seed in seeds.iter() {
            let mut mapped = *seed;
            for mappings in map_sections.iter() {
                let default = RangeToRangeMap::identity(mapped);
                let mapping = mappings
                    .iter()
                    .take_while(|map| mapped >= map.from.start)
                    .find(|map| map.from.contains(&mapped))
                    .unwrap_or(&default);

                mapped = mapping.map_value(mapped);
            }
            if mapped < min {
                min = mapped;
            }
        }

        min
    }
}

const NUM_SEED_RANGES: usize = 10;

pub fn part_2(input: &str) -> usize {
    let mut sections = input.split("\n\n");
    sections.next().unwrap();

    unsafe {
        let mut data = input.as_ptr();
        data = data.add("seeds: ".len());

        let mut seed_ranges = parse_seed_ranges(&mut data);

        let map_sections = parse_map_sections(&mut data);

        for map_section in map_sections.iter() {
            seed_ranges = seed_ranges
                .into_iter()
                .map(|mut seed_range| {
                    let maps: Vec<&RangeToRangeMap> = map_section
                        .iter()
                        .filter(|m| seed_range.intersects(&m.from))
                        .collect();

                    if maps.is_empty() {
                        return vec![seed_range];
                    }

                    let mut mapped = vec![];
                    for map in maps.into_iter() {
                        let (left_overhang, mapped_range, right_overhang) = map.map(seed_range);
                        mapped.push(mapped_range);

                        match left_overhang {
                            // Maps are sorted -> We won't find a mapping for the left overhang in this section
                            Some(range) => mapped.push(range),
                            _ => {}
                        }

                        if right_overhang.is_none() {
                            break;
                        }

                        // Maps are sorted -> right overhang may be mapped by subsequent maps
                        seed_range = right_overhang.unwrap();
                    }

                    mapped
                })
                .flatten()
                .collect();
        }

        seed_ranges.iter().map(|r| r.start).min().unwrap()
    }
}

pub fn parse_seeds(data: &mut *const u8) -> [usize; NUM_SEEDS] {
    unsafe {
        let mut seeds = [0; NUM_SEEDS];
        for i in 0..NUM_SEEDS {
            seeds[i] = data.parse_ascii_digits(get_num_seed_digits(data));
            *data = data.add(1);
        }

        seeds
    }
}

fn parse_seed_ranges(data: &mut *const u8) -> Vec<Range<usize>> {
    unsafe {
        let mut seed_ranges = vec![];
        for _ in 0..NUM_SEED_RANGES {
            let mut seed_range = [0; 2];
            for i in 0..2 {
                seed_range[i] = data.parse_ascii_digits(get_num_seed_digits(data));
                *data = data.add(1);
            }

            seed_ranges.push(seed_range[0]..seed_range[0] + seed_range[1]);
        }

        seed_ranges
    }
}

unsafe fn get_num_seed_digits(data: &*const u8) -> usize {
    // #digits: {8:5, 9:7, 10:8}
    if !data.add(10).read().is_ascii_digit() && data.add(9).read().is_ascii_digit() {
        10
    } else if !data.add(9).read().is_ascii_digit() {
        9
    } else {
        8
    }
}

pub unsafe fn parse_map_sections(data: &mut *const u8) -> Vec<Vec<RangeToRangeMap>> {
    let mut sections = vec![];

    while data.read() == b'\n' {
        *data = data.add(1);
        sections.push(parse_map_section(data));
    }

    sections
}

unsafe fn parse_map_section(data: &mut *const u8) -> Vec<RangeToRangeMap> {
    while data.read() != b'\n' {
        *data = data.add(1);
    }
    *data = data.add(1);

    let mut section = Vec::with_capacity(40);

    loop {
        if !data.read().is_ascii_digit() {
            break;
        }

        let mut parts = [0; 3];
        for i in 0..3 {
            let mut num = 0;
            while data.read().is_ascii_digit() {
                num *= 10;
                num += (data.read() - b'0') as usize;
                *data = data.add(1);
            }
            *data = data.add(1);

            parts[i] = num;
        }

        section.push(RangeToRangeMap {
            from: parts[1]..parts[1] + parts[2],
            to: parts[0]..parts[0] + parts[2],
        });
    }

    section.sort_unstable_by_key(|map| map.from.start);
    section
}

#[derive(Debug, PartialEq, Clone)]
pub struct RangeToRangeMap {
    pub from: Range<usize>,
    pub to: Range<usize>,
}

impl From<(Range<usize>, Range<usize>)> for RangeToRangeMap {
    fn from((from, to): (Range<usize>, Range<usize>)) -> Self {
        Self { from, to }
    }
}

impl RangeToRangeMap {
    pub fn identity(val: usize) -> Self {
        Self {
            from: val..val,
            to: val..val,
        }
    }

    pub fn len(&self) -> usize {
        self.from.len()
    }

    pub fn map_value(&self, val: usize) -> usize {
        self.to.start + (val - self.from.start)
    }

    pub fn map(
        &self,
        mut range: Range<usize>,
    ) -> (Option<Range<usize>>, Range<usize>, Option<Range<usize>>) {
        if !range.intersects(&self.from) {
            return (None, range, None);
        }

        let left_overhang = if range.start < self.from.start {
            let len = self.from.start - range.start;
            Some(range.start..range.start + len)
        } else {
            None
        };

        let right_overhang = if range.end > self.from.end {
            let len = range.end - self.from.end;
            Some(range.end - len..range.end)
        } else {
            None
        };

        range.start = self.to.start + (range.start.max(self.from.start) - self.from.start);
        range.end = self.to.start + (range.end.min(self.from.end) - self.from.start);

        (left_overhang, range, right_overhang)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map() {
        // range:    1 2 3
        // map.from: 1 2 3
        // map.to:   4 5 6
        let map = RangeToRangeMap::from((1..4, 4..7));
        assert_eq!(map.map(1..4), (None, 4..7, None));

        // range:      2 3 4 5
        // map.from: 1 2 3
        // map.to:   4 5 6
        let map = RangeToRangeMap::from((1..4, 4..7));
        assert_eq!(map.map(2..6), (None, 5..7, Some(4..6)));
    }

    #[test]
    fn test_map_example_0() {
        // seed-to-soil
        assert_map_seed(79..93, (50..98, 52..100), (None, 81..95, None));

        // soil-to-fertilizer -> No mapping exists
        // fertilizer-to-water -> No mapping exists

        // water-to-light
        assert_map_seed(81..95, (25..95, 18..88), (None, 74..88, None));

        // light-to-temperature
        assert_map_seed(74..88, (64..77, 68..81), (None, 78..81, Some(77..88)));
        assert_map_seed(77..88, (77..100, 45..68), (None, 45..56, None)); // handle right overhang

        // temperature-to-humidity -> Only mapping for 45..56 exists
        assert_map_seed(45..56, (0..69, 1..70), (None, 46..57, None));

        // humidity-to-location
        assert_map_seed(78..81, (56..93, 60..97), (None, 82..85, None));
        assert_map_seed(46..57, (56..93, 60..97), (Some(46..56), 60..61, None));
    }

    #[test]
    fn test_map_input() {
        let seed = 3169137700..3440855309;
        let map = RangeToRangeMap {
            from: 3113014199..3363187305,
            to: 2330985014..2581158120,
        };
        assert_eq!(
            map.map(seed),
            (None, 2387108515..2581158120, Some(3363187305..3440855309))
        );
    }

    fn assert_map_seed(
        seed: Range<usize>,
        (map_from, map_to): (Range<usize>, Range<usize>),
        expected: (Option<Range<usize>>, Range<usize>, Option<Range<usize>>),
    ) {
        let map = RangeToRangeMap {
            from: map_from,
            to: map_to,
        };
        assert_eq!(map.map(seed), expected);
    }
}
