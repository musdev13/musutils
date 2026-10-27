use std::io;
use std::path::Path;

pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
    std::fs::remove_dir_all(path)
}
