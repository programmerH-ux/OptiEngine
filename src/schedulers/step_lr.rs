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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schedulers::scheduler::Scheduler;

    #[test]
    fn test_step_lr_decay() {
       let mut scheduler = StepLR::new(0.1,2,0.5);

       assert_eq!(scheduler.current_lr(), 0.1);

       scheduler.step();
       assert_eq!(scheduler.current_lr(), 0.1);

       scheduler.step();
       assert!((scheduler.current_lr() - 0.05).abs() < 1e-10);

       scheduler.step();
       assert!((scheduler.current_lr() - 0.05).abs() < 1e-10);

       scheduler.step();
       assert!((scheduler.current_lr() - 0.025).abs() < 1e-10);
      }
}
