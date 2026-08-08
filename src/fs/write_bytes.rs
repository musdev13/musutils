use std::io;
use std::path::Path;

pub fn write_bytes<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()> {
    std::fs::write(path, contents)
}
