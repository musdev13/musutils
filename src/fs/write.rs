use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn write<C: AsRef<[u8]>>(path: PathBuf, content: C) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            crate::fs::new_dir(parent);
        }
    }

    let mut file = File::create(&path)?;
    file.write_all(content.as_ref())?;

    Ok(())
}
