use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current time in milliseconds since the Unix epoch.
///
/// # Returns
///
/// * `u128` - The current time in milliseconds since the Unix epoch.
///
/// # Panics
///
/// * If the current time is before the Unix epoch.
#[allow(clippy::expect_used)]
pub fn current_time_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards!")
        .as_millis()
}
