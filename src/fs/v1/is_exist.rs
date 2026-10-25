use crate::fs::tilda_desir;
use std::path::Path;

pub fn is_exist<P: AsRef<Path>>(path: P) -> bool {
    tilda_desir(path).exists()
}
