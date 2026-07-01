pub trait Scheduler {
    fn step(&mut self);
    fn current_lr(&self) -> f64;
}
