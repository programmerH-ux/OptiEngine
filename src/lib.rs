use pyo3::prelude::*;

mod license;
mod core;
#[cfg(feature = "pro")]
mod pro;

#[pymodule]
fn optiengine(_py: Python, m: &PyModule) -> PyResult<()> {
    // Free optimizer
    m.add_class::<core::gradient::GradientDescent>()?;

    #[cfg(feature = "pro")]
    {
        // Pro optimizers
        m.add_class::<pro::adam::Adam>()?;
        m.add_class::<pro::rmsprop::RMSProp>()?;

        // Expose license activation function as clean name
        m.add_function(pyo3::wrap_pyfunction!(license::activate_license_py, m)?)?;
    }

    Ok(())
}
