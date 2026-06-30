pub fn pro_enabled() -> bool {
    std::env::var("OPTICORE_PRO")
        .map(|v| v == "1")
        .unwrap_or(false)
}
