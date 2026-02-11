use std::env;
use sha2::{Sha256, Digest};

const SECRET: &str = "oe_super_secret_2026";

pub fn ensure_pro() {
    let key = env::var("OPTIENGINE_LICENSE_KEY")
        .expect("OptiEngine Pro license key not found");

    if !is_valid_key(&key) {
        panic!("Invalid OptiEngine Pro license key");
    }
}

fn is_valid_key(key: &str) -> bool {
    let parts: Vec<&str> = key.split('-').collect();
    if parts.len() != 5 {
        return false;
    }

    let base = format!(
        "{}-{}-{}-{}",
        parts[0], parts[1], parts[2], parts[3]
    );

    let mut hasher = Sha256::new();
    hasher.update(base.as_bytes());
    hasher.update(SECRET.as_bytes());
    let result = hasher.finalize();
    let expected = format!("{:x}", result)[..8].to_uppercase();

    expected == parts[4]
}
