use pyo3::prelude::*;
use crate::core::step_optimizer::StepOptimizer;

#[pyclass]
pub struct GradientDescent {
    lr: f64,
}

#[pymethods]
impl GradientDescent {
    #[new]
    pub fn new(lr: f64) -> Self {
        Self { lr }
    }

    pub fn step(&self, value: f64) -> f64 {
        value - self.lr
    }
}

impl StepOptimizer for GradientDescent {
    fn step(&self, value: f64) -> f64 {
        value - self.lr
    }
}
