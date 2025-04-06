#[cfg(feature = "web")]
pub fn now() -> f64 {
    web_sys::window()
    .expect("should have a window")
    .performance()
    .expect("performance should be available")
    .now()
}

#[cfg(not(feature = "web"))]
pub fn now() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now();
    // Calculate the duration since UNIX_EPOCH.
    let duration = now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration.as_millis() as f64
}