use super::scheduler::Scheduler;

pub struct ExponentialLR {
    lr: f64,
    gamma: f64,
}

impl ExponentialLR {
    pub fn new(lr: f64, gamma: f64) -> 
       Self {
              Self { lr, gamma }
           }
}

impl Scheduler for ExponentialLR {
    fn step(&mut self) {
        self.lr *= self.gamma;
    }

    fn current_lr(&self) -> f64 {
        self.lr
    }
}
