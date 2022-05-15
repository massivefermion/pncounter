pub struct GCounter {
    id: usize,
    values: Vec<usize>,
}

impl GCounter {
    pub fn new(id: usize, total: usize) -> GCounter {
        GCounter {
            id,
            values: vec![0; total],
        }
    }

    pub fn query(&self) -> usize {
        self.values.iter().sum()
    }

    pub fn add(&mut self, additive: usize) {
        self.values[self.id] += additive;
    }

    pub fn merge(&mut self, other: GCounter) {
        let zipped: Vec<(&usize, &usize)> = self.values.iter().zip(other.values.iter()).collect();
        self.values = zipped
            .iter()
            .map(|(a, b)| {
                if a > b {
                    return a;
                }
                return b;
            })
            .map(|v| **v)
            .collect();
    }
}
