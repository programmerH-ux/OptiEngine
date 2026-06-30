pub trait StepOptimizer {
    fn step(&self, value: f64) -> f64;
}
