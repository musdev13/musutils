use std::fs;
use std::path::PathBuf;

use crate::fs::{new_dir, tilda_desir};

pub fn get(app_name: &str, file_path: &str, def_content: Option<&str>) -> String {
    let base_path: PathBuf;

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        base_path = tilda_desir("~/.config").join(app_name);
    }

    #[cfg(target_os = "windows")]
    {
        base_path = tilda_desir("~")
            .join("AppData")
            .join("Roaming")
            .join(app_name);
    }

    let full_file_path = base_path.join(file_path);

    if !full_file_path.exists() {
        if let Some(content) = def_content {
            if let Some(parent_dir) = full_file_path.parent() {
                new_dir(parent_dir);
            }
            fs::write(&full_file_path, content).expect("Failed to write default config file");
            return content.to_string();
        }
    }

    fs::read_to_string(full_file_path).expect("Failed to read config file")
}
