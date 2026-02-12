use pyo3::prelude::*;
use sha2::{Sha256, Digest};
use std::sync::atomic::{AtomicBool, Ordering};

// License state
static LICENSE_VALID: AtomicBool = AtomicBool::new(false);

// Replace this with your actual SHA256 hash of the license key
const VALID_HASH: &str = "c49bc20ce40ca9e4eb9f6224484d1788b632a6c38dcd6169ecc1fa105f8a3456";

// Ensure Pro license is active
pub fn ensure_pro() {
    if !LICENSE_VALID.load(Ordering::Relaxed) {
        panic!("Pro license not activated. Please call activate_license(\"YOUR_KEY\") first.");
    }
}

// Expose to Python as clean name: activate_license
#[pyfunction(name = "activate_license")]
pub fn activate_license_py(key: &str) -> PyResult<()> {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    let result = hasher.finalize();
    let hash = format!("{:x}", result);

    if hash == VALID_HASH {
        LICENSE_VALID.store(true, Ordering::Relaxed);
        Ok(())
    } else {
        Err(pyo3::exceptions::PyPermissionError::new_err(
            "Invalid license key",
        ))
    }
}
