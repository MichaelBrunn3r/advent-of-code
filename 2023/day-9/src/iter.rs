use itertools::Itertools;

pub struct SeriesValuesIterator<'a> {
    input: &'a [u8],
    pos: usize,
}

impl SeriesValuesIterator<'_> {
    pub fn new(input: &[u8]) -> SeriesValuesIterator {
        SeriesValuesIterator { input, pos: 0 }
    }
}

impl<'a> Iterator for SeriesValuesIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return None;
        }

        let sign = match self.input[self.pos] {
            b'-' => {
                self.pos += 1;
                -1
            }
            b'0'..=b'9' => 1,
            _ => {
                return None;
            }
        };

        let mut val = 0;
        while self.pos < self.input.len() {
            let c = self.input[self.pos];
            match c {
                b'0'..=b'9' => {
                    self.pos += 1;
                    val = val * 10 + (c - b'0') as i32;
                }
                _ => {
                    self.pos += 1;
                    break;
                }
            }
        }

        return Some(val * sign);
    }
}

pub struct SeriesValuesIteratorReverse<'a> {
    input: &'a [u8],
    pos: i32,
}

impl SeriesValuesIteratorReverse<'_> {
    pub fn new(input: &[u8]) -> SeriesValuesIteratorReverse {
        SeriesValuesIteratorReverse {
            input,
            pos: (input.len() - 1) as i32,
        }
    }
}

impl Iterator for SeriesValuesIteratorReverse<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < 0 {
            return None;
        }

        let first = self.input[self.pos as usize];
        let mut val = match first {
            b'0'..=b'9' => (first - b'0') as i32,
            _ => {
                return None;
            }
        };
        self.pos -= 1;

        let mut sign = 1;
        let mut multiplier = 10;
        while self.pos >= 0 {
            let c = self.input[self.pos as usize];
            match c {
                b'0'..=b'9' => {
                    self.pos -= 1;
                    val += multiplier * (c - b'0') as i32;
                    multiplier *= 10;
                }
                b'-' => {
                    self.pos -= 1;
                    sign = -1;
                }
                _ => {
                    self.pos -= 1;
                    break;
                }
            }
        }

        return Some(val * sign);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_series_values_iterator() {
        assert_eq!(
            SeriesValuesIterator::new("0 1 2 3 4".as_bytes()).collect_vec(),
            vec![0, 1, 2, 3, 4]
        );
        assert_eq!(
            SeriesValuesIterator::new("-1 -4 -13 -35 -77 -144 -237 -351 -473 -580 -637 -595 -389 64 867 2145 4047 6748 10451 15389 21827".as_bytes()).collect_vec(),
            vec![-1, -4, -13,-35,-77,-144,-237,-351,-473,-580,-637,-595,-389,64,867,2145,4047,6748,10451,15389,21827]
        );
    }

    #[test]
    fn test_series_values_iterator_rev() {
        assert_eq!(
            SeriesValuesIteratorReverse::new("0 1 2 3 4".as_bytes()).collect_vec(),
            vec![4, 3, 2, 1, 0]
        );
        assert_eq!(
            SeriesValuesIteratorReverse::new("3 2 1 0 -1 -2 -3".as_bytes()).collect_vec(),
            vec![-3, -2, -1, 0, 1, 2, 3]
        );
        assert_eq!(
            SeriesValuesIteratorReverse::new("-144".as_bytes()).collect_vec(),
            vec![-144]
        );
    }
}
