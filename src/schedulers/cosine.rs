use std::f64::consts::PI;
use super::scheduler::Scheduler;

pub struct CosineAnnealingLR {
    initial_lr: f64,
    lr: f64,
    t_max: usize,
    current_step: usize,
}

impl CosineAnnealingLR {
     pub fn new(initial_lr: f64, t_max: usize) -> Self{
         Self {
             initial_lr,
             lr: initial_lr,
             t_max,
             current_step: 0,
         }
     }
}

impl Scheduler for CosineAnnealingLR {
     fn step(&mut self) {
         self.current_step += 1;

         self.lr = self.initial_lr * 0.5 * (1.0 + (PI * self.current_step as f64 / self.t_max as f64).cos());
     }

     fn current_lr(&self) -> f64 {
         self.lr
     }
}
