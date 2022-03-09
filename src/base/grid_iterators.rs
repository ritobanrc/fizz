use crate::base::Range;
use crate::math::*;

/// An iterator over an N-dimensional range. Iterates over the all of the integer coordinates in
/// the `Range` (`max` boundary is not included).
pub struct RangeIterator {
    index: IV,
    range: Range<IV>,
    done: bool,
}

impl RangeIterator {
    pub fn new(range: Range<IV>) -> Self {
        Self {
            index: range.min,
            range,
            done: false,
        }
    }
}

impl Iterator for RangeIterator {
    type Item = IV;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self.index;

        let mut i = DIM - 1;
        loop {
            self.index[i] += 1;
            if self.index[i] >= self.range.max[i] {
                self.index[i] = self.range.min[i];
                if i != 0 {
                    i -= 1;
                } else {
                    self.done = true;
                    break;
                }
            } else {
                break;
            }
        }

        Some(result)
    }
}
