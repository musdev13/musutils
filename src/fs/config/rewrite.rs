use std::fs;
use std::path::PathBuf;

use crate::fs::{new_dir, tilda_desir};

pub fn rewrite(app_name: &str, file_path: &str, json_content: &str) {
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

    if let Some(parent_dir) = full_file_path.parent() {
        new_dir(parent_dir);
    }

    fs::write(&full_file_path, json_content).expect("Failed to write config file");
}
