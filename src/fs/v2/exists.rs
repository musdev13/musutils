use std::path::Path;

pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

pub fn is_exist<P: AsRef<Path>>(path: P) -> bool {
    exists(path)
}
