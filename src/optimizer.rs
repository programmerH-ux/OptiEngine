pub struct Optimizer {
    iterations: u32,
}

impl Optimizer {
    pub fn new(iterations: u32) -> Self {
        Self { iterations }
    }

    pub fn optimize(&self, mut value: f64) -> f64 {
        for _ in 0..self.iterations {
            value -= 0.01 * value;
        }
        value
    }
}
