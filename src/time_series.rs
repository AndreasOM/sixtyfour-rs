use std::collections::VecDeque;

#[derive(Debug)]
pub struct TimeSeries {
    count: usize,
    values: VecDeque<f32>,
}

impl Default for TimeSeries {
    fn default() -> Self {
        Self::new(1000)
    }
}
impl TimeSeries {
    pub fn new(count: usize) -> Self {
        Self {
            count,
            values: VecDeque::with_capacity(count),
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }
    pub fn values(&self) -> impl Iterator<Item = &f32> {
        self.values.iter()
    }

    pub fn push(&mut self, value: f32) {
        while self.values.len() >= self.count {
            if let Some(_old_value) = self.values.pop_front() {}
        }

        self.values.push_back(value);
    }

    pub fn avg(&self, count: usize) -> f32 {
        let count = count.min(self.values.len());
        let start = self.values.len() - count;
        let mut total = 0.0;
        for v in self.values.range(start..) {
            total += v;
        }

        let r = total / count as f32;

        r
    }

    pub fn max(&self, count: usize) -> f32 {
        let count = count.min(self.values.len());
        let start = self.values.len() - count;
        let mut r = f32::MIN;
        for v in self.values.range(start..) {
            r = r.max(*v);
        }
        r
    }
    pub fn min(&self, count: usize) -> f32 {
        let count = count.min(self.values.len());
        let start = self.values.len() - count;
        let mut r = f32::MAX;
        for v in self.values.range(start..) {
            r = r.min(*v);
        }
        r
    }
}
