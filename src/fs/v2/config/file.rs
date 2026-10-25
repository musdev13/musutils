use super::ConfigError;
use super::paths::config_path;
use crate::fs::write_atomic;

pub fn get(
    app_name: &str,
    file_path: &str,
    def_content: Option<&str>,
) -> Result<String, ConfigError> {
    let path = config_path(app_name, file_path);
    if !path.exists()
        && let Some(content) = def_content
    {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        write_atomic(&path, content)?;
        return Ok(content.to_string());
    }
    Ok(std::fs::read_to_string(&path)?)
}

pub fn rewrite(app_name: &str, file_path: &str, json_content: &str) -> Result<(), ConfigError> {
    let path = config_path(app_name, file_path);
    write_atomic(&path, json_content)?;
    Ok(())
}
