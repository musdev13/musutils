use std::path::{Path, PathBuf};

use super::normalize::normalize;

pub fn expand<P: AsRef<Path>>(path: P) -> PathBuf {
    let path_ref = path.as_ref();
    let base = if let Ok(stripped) = path_ref.strip_prefix("~") {
        match home::home_dir() {
            Some(home) => home.join(stripped),
            None => path_ref.to_path_buf(),
        }
    } else if path_ref.is_relative() {
        match std::env::current_dir() {
            Ok(cwd) => cwd.join(path_ref),
            Err(_) => path_ref.to_path_buf(),
        }
    } else {
        path_ref.to_path_buf()
    };
    normalize(&base)
}

pub fn tilda_desir<P: AsRef<Path>>(path: P) -> PathBuf {
    expand(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_absolute_unchanged() {
        let p = Path::new("/tmp/x");
        assert_eq!(expand(p), PathBuf::from("/tmp/x"));
    }
}
