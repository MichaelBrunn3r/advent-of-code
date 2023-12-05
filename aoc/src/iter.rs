use crate::prelude::*;

pub trait IteratorExt<Item> {
    fn progress(self, options: ProgressOptions) -> Box<dyn Iterator<Item = Item>>;
}

impl<Item, Iter: Iterator<Item = Item> + 'static> IteratorExt<Item> for Iter {
    fn progress(self, options: ProgressOptions) -> Box<dyn Iterator<Item = Item>> {
        let mut i = 0;
        let mut prev_percentage = 0.0;
        Box::new(self.into_iter().inspect(move |_| {
            i += 1;

            let percentage = i as f32 / options.max as f32;
            if !(percentage - prev_percentage > 0.001) {
                return;
            }
            prev_percentage = percentage;

            let percentage_points = (percentage * 100.0).floor() as usize;
            print!(
                "\r{}{} {:.1}%",
                options.full_char.repeat(percentage_points),
                options.empty_char.repeat(100 - percentage_points),
                percentage * 100.0
            );
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }))
    }
}

pub struct ProgressOptions {
    max: usize,
    empty_char: char,
    full_char: char,
}

impl From<usize> for ProgressOptions {
    fn from(max: usize) -> Self {
        Self::new(max)
    }
}

impl ProgressOptions {
    pub fn new(max: usize) -> Self {
        Self {
            max,
            empty_char: '▁',
            full_char: '█',
        }
    }
}
