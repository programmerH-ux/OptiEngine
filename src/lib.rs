use pyo3::prelude::*;

pub mod schedulers;
pub mod license;
pub mod pro;

use pro::adam::Adam;
use pro::rmsprop::RMSProp;
use pro::momentum::Momentum;

#[pymodule]
fn optiengine(_py: Python, m: &PyModule) -> PyResult<()> {

    // License functions
    m.add_function(wrap_pyfunction!(license::activate_license_py, m)?)?;
    m.add_function(wrap_pyfunction!(license::machine_id_py, m)?)?;

    // Optimizers
    m.add_class::<Adam>()?;
    m.add_class::<RMSProp>()?;
    m.add_class::<Momentum>()?;

    Ok(())
}
