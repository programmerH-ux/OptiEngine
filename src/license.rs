use std::sync::Mutex;
use once_cell::sync::Lazy;

static LICENSE_ACTIVE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

pub fn activate_license() {
    let mut license = LICENSE_ACTIVE.lock().unwrap();
    *license = true;
}

pub fn ensure_pro() {
    let license = LICENSE_ACTIVE.lock().unwrap();
    if !*license {
        panic!("Pro license not activated. Call optiengine.activate() first.");
    }
}
