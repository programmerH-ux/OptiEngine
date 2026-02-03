use pyo3::prelude::*;
pub mod optimizer;

#[pyfunction]
fn gradient_descent(
    value: f64,
    iterations: u32,
    learning_rate: f64,
) -> f64 {
    let opt = optimizer::Optimizer::new(iterations, learning_rate, 0.0);
    opt.gradient_descent(value)
}

#[pyfunction]
fn momentum_optimize(
    value: f64,
    iterations: u32,
    learning_rate: f64,
    momentum: f64,
) -> f64 {
    let opt = optimizer::Optimizer::new(iterations, learning_rate, momentum);
    opt.momentum_optimize(value)
}

#[pymodule]
fn opticore(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gradient_descent, m)?)?;
    m.add_function(wrap_pyfunction!(momentum_optimize, m)?)?;
    Ok(())
}
