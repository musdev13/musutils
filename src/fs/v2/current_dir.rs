use std::io;
use std::path::PathBuf;

pub fn current_dir() -> io::Result<PathBuf> {
    std::env::current_dir()
}

pub fn get_curr_dir() -> PathBuf {
    current_dir().expect("Failed to get current working directory")
}
