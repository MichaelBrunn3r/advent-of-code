use std::ops::Range;

use aoc::prelude::RangeExt;

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
        if !range.overlaps(&self.from) {
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
