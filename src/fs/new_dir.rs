use std::fs;
use std::path::Path;

use crate::fs::tilda_desir;

pub fn new_dir<P: AsRef<Path>>(path: P) -> bool {
    fs::create_dir_all(tilda_desir(path)).expect(&format!("{}: can't create dir", crate::types::Status::Err.as_colored_str()));
    true
}
