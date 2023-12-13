use crate::prelude::*;
use itertools::Itertools;

pub trait IteratorExt<Item> {
    fn duplicate_positions(self) -> Vec<usize>;
}

impl<Item, Iter: Iterator<Item = Item>> IteratorExt<Item> for Iter
where
    Item: PartialEq + Clone,
{
    fn duplicate_positions(self) -> Vec<usize> {
        let mut reflections = vec![];

        self.enumerate()
            .tuple_windows()
            .for_each(|((_, prev), (curr_idx, curr))| {
                if prev == curr {
                    reflections.push(curr_idx);
                }
            });

        reflections
    }
}
