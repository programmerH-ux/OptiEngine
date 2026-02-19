use pyo3::prelude::*;
mod license;
mod pro;

#[pyfunction]
fn activate_license(key: &str) -> PyResult<()> {
    license::activate_license_py(key)
}

#[pyfunction]
fn machine_id() -> PyResult<String> {
    license::machine_id_py()
}

#[pymodule]
fn optiengine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(activate_license, m)?)?;
    m.add_function(wrap_pyfunction!(machine_id, m)?)?;
    Ok(())
}
