use pyo3::prelude::*;
use crate::license::ensure_pro;
use crate::core::step_optimizer::StepOptimizer;

#[pyclass]
pub struct Adam {
    lr: f64,
}

#[pymethods]
impl Adam {
    #[new]
    pub fn new(lr: f64) -> Self {
        ensure_pro();
        Self { lr }
    }

    pub fn step(&self, value: f64) -> f64 {
        value - self.lr
    }
}

impl StepOptimizer for Adam {
    fn step(&self, value: f64) -> f64 {
        value - self.lr
    }
}
