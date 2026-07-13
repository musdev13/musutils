use std::path::{Path, PathBuf};

pub fn tilda_desir<P: AsRef<Path>>(path: P) -> PathBuf {
    let path_ref = path.as_ref();

    if path_ref.starts_with("~") {
        if let Some(home) = home::home_dir() {
            if let Ok(stripped) = path_ref.strip_prefix("~") {
                return home.join(stripped);
            }
        }
    }

    path_ref.to_path_buf()
}
