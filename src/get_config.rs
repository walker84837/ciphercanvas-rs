use log::error;
use toml::Value;

pub fn get_config_str<'a>(
    config: &'a Value,
    key: &str,
    second_key: &str,
    default: &'a str,
) -> &'a str {
    config[key][second_key].as_str().unwrap_or_else(|| {
        error!(
            "Configuration error: Unable to find the string value for '{}.{}'. Using default: '{}'.",
            key, second_key, default
        );
        default
    })
}

pub fn get_config_int(config: &Value, key: &str, second_key: &str, default: i64) -> i64 {
    config[key][second_key].as_integer().unwrap_or_else(|| {
        error!(
            "Configuration error: Unable to find the integer value for '{}.{}'. Using default: '{}'.",
            key, second_key, default
        );
        default
    })
}
