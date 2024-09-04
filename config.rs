use std::env;
use std::path::PathBuf;

/// Loads the configuration path from an environment variable.
///
/// # Arguments
///
/// * `default` - Optional default value for the configuration path if the environment variable is not set.
///
/// # Returns
///
/// * `Ok(PathBuf)` - If the `CONFIG_PATH` environment variable is set or a default value is provided, returns the path as a `PathBuf`.
/// * `Err(String)` - If the environment variable is not set and no default is provided, returns an error message.
///
/// # Examples
///
/// ```
/// let config_path = load_config(None).unwrap();
/// println!("Config path: {}", config_path.display());
/// ```
///
/// # Notes
/// This function attempts to read the `CONFIG_PATH` environment variable. If it is not set, it uses the provided default value if available.
pub fn load_config(default: Option<&str>) -> Result<PathBuf, String> {
    match env::var("CONFIG_PATH") {
        Ok(path) => {
            let path_buf = PathBuf::from(path);
            if path_buf.exists() {
                Ok(path_buf)
            } else {
                Err("Configuration path does not exist".to_string())
            }
        }
        Err(_) => {
            if let Some(default_path) = default {
                Ok(PathBuf::from(default_path))
            } else {
                Err("Failed to load config path from environment and no default path provided".to_string())
            }
        }
    }
}
