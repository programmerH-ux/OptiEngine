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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schedulers::scheduler::Scheduler;

    #[test]
    fn test_exponential_lr_decay() {
        let mut scheduler = ExponentialLR::new(0.1, 0.9);

        scheduler.step();
        assert!((scheduler.current_lr() - 0.09).abs() < 1e-10);

        scheduler.step();
        assert!((scheduler.current_lr() - 0.081).abs() < 1e-10);
    }
}
