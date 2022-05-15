use crate::gcounter::GCounter;

pub struct PNCounter {
    positive: GCounter,
    negative: GCounter,
}

impl PNCounter {
    pub fn new(id: usize, total: usize) -> PNCounter {
        PNCounter {
            positive: GCounter::new(id, total),
            negative: GCounter::new(id, total),
        }
    }

    pub fn query(&self) -> isize {
        (self.positive.query() as isize) - (self.negative.query() as isize)
    }

    pub fn add(&mut self, additive: usize) {
        self.positive.add(additive);
    }

    pub fn subtract(&mut self, subtractive: usize) {
        self.negative.add(subtractive);
    }

    pub fn merge(&mut self, other: PNCounter) {
        self.positive.merge(other.positive);
        self.negative.merge(other.negative);
    }
}
