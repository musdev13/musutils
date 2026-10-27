use std::io;
use std::path::Path;

pub fn create_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
    std::fs::create_dir_all(path)
}

pub fn new_dir<P: AsRef<Path>>(path: P) -> bool {
    create_dir_all(path).is_ok()
}
