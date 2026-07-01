use super::scheduler::Scheduler;

pub struct StepLR{
    lr: f64,
    gamma: f64,
    step_size: usize,
    current_step: usize,
}

impl StepLR {
    pub fn new(lr: f64, step_size: usize, gamma: f64) -> Self {
        Self {
            lr,
            gamma,
            step_size,
            current_step: 0,
        }
    }
}

impl Scheduler for StepLR {
    fn step(&mut self) {
        self.current_step += 1;

        if self.current_step % self.step_size == 0 {
            self.lr *= self.gamma;
        }
    }

    fn current_lr(&self) -> f64 {
        self.lr
    }
}
