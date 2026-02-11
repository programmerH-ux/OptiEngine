use std::fs;
use std::path::PathBuf;

fn license_path() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME directory not found");
    PathBuf::from(home).join(".optiengine").join("license.key")
}

pub fn ensure_pro() {
    let path = license_path();
    if !path.exists() {
        panic!("OptiEngine Pro license not found");
    }

    let key = fs::read_to_string(&path).expect("Failed to read license file");
    if key.trim() != "OPTIPRO-2026-VALID" {
        panic!("Invalid OptiEngine Pro license");
    }
}

pub mod adam;
pub mod rmsprop;
