use pyo3::prelude::*;
use crate::license::ensure_pro;

#[pyclass]
pub struct Momentum {
    lr: f64,
    momentum: f64,
    velocity: f64,
}

#[pymethods]
impl Momentum {
     #[new]
     pub fn new(lr: f64, momentum: f64) ->
    PyResult<Self> {
         ensure_pro()?;

         Ok(Self {
             lr,
             momentum,
             velocity: 0.0,
         })
    }
     pub fn step(&mut self, value: f64) ->
    PyResult<f64> {
         ensure_pro()?;

         let grad = value;

         self.velocity = self.momentum * self.velocity - self.lr * grad;

         Ok(value + self.velocity)
    }
}
