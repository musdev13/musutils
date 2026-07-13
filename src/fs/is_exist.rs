use std::path::Path;
use crate::fs::tilda_desir;

pub fn is_exist<P: AsRef<Path>>(path: P) -> bool {
    tilda_desir(path).exists()
}
