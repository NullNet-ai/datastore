use std::env;

pub fn default_sensitivity_level() -> Option<i32> {
    let default_value = env::var("DEFAULT_SENSITIVITY_LEVEL")
        .ok()
        .and_then(|val| val.parse::<i32>().ok())
        .unwrap_or(1000);

    Some(default_value)
}