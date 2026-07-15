use std::path::PathBuf;

pub fn get_curr_dir() -> PathBuf {
    std::env::current_dir()
        .expect("Failed to get current working directory")
}
