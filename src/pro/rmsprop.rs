use pyo3::prelude::*;
use crate::license::ensure_pro;

#[pyclass]
pub struct RMSProp {
    lr: f64,
    decay: f64,
    t: u32,
    avg_sq: f64,
}

#[pymethods]
impl RMSProp {
    #[new]
    pub fn new(lr: f64, decay: f64) -> Self {
        ensure_pro();
        Self { lr, decay, t: 0, avg_sq: 0.0 }
    }

    pub fn step(&mut self, value: f64) -> f64 {
        ensure_pro();
        self.t += 1;
        let grad = value;
        self.avg_sq = self.decay * self.avg_sq + (1.0 - self.decay) * grad * grad;
        value - self.lr * grad / (self.avg_sq.sqrt() + 1e-8)
    }
}
