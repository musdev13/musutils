use std::path::{Path, PathBuf};

pub fn tilda_desir<P: AsRef<Path>>(path: P) -> PathBuf {
    let path_ref = path.as_ref();

    let base_path = if path_ref.starts_with("~") {
        if let Some(home) = home::home_dir() {
            if let Ok(stripped) = path_ref.strip_prefix("~") {
                home.join(stripped)
            } else {
                path_ref.to_path_buf()
            }
        } else {
            path_ref.to_path_buf()
        }
    } else if path_ref.is_relative() {
        crate::fs::get_curr_dir().join(path_ref)
    } else {
        path_ref.to_path_buf()
    };

    crate::fs::normalize_path(&base_path)
}
