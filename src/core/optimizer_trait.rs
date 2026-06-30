pub trait OptimizerStep {
    fn step(&mut self, value: f64) -> f64;
}
