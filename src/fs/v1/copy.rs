use std::fs;
use std::io;
use std::path::Path;

pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<u64> {
    fs::copy(from, to)
}
