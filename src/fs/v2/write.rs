use std::io;
use std::path::Path;

pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, content: C) -> io::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content)
}

pub fn write_atomic<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, content: C) -> io::Result<()> {
    use std::io::Write;

    let path = path.as_ref();
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
    }
    let file_name = path
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "path has no file name"))?;
    let tmp_name = format!(
        ".{}.tmp-{}",
        file_name.to_string_lossy(),
        std::process::id()
    );
    let tmp_path = path.with_file_name(tmp_name);
    {
        let mut file = std::fs::File::create(&tmp_path)?;
        file.write_all(content.as_ref())?;
        file.sync_all()?;
    }
    match std::fs::rename(&tmp_path, path) {
        Ok(()) => Ok(()),
        Err(err) => {
            let _ = std::fs::remove_file(&tmp_path);
            Err(err)
        }
    }
}

pub fn write_bytes<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()> {
    write(path, contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_creates_parents() {
        let dir = std::env::temp_dir().join(format!("musutils_fs_v2_{}", std::process::id()));
        let file = dir.join("sub").join("a.txt");
        write(&file, b"hi").unwrap();
        assert!(file.exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn write_atomic_creates_file() {
        let dir =
            std::env::temp_dir().join(format!("musutils_fs_v2_atomic_{}", std::process::id()));
        let file = dir.join("a.txt");
        write_atomic(&file, b"data").unwrap();
        assert_eq!(std::fs::read(&file).unwrap(), b"data");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
