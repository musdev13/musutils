use std::path::PathBuf;
use sha1::{Sha1, Digest};

pub async fn download_to_sha1(url: String, path: PathBuf, expected_sha1: String) -> std::io::Result<()> {
    loop {
        crate::http::download_to(url.clone(), path.clone()).await?;

        if let Ok(file_bytes) = std::fs::read(&path) {
            let mut hasher = Sha1::new();
            hasher.update(&file_bytes);
            let result = hasher.finalize();
            let actual_sha1 = format!("{:x}", result);

            if actual_sha1 == expected_sha1 {
                break;
            } else {
                eprintln!(
                    "{}: Hash mismatch for {:?}. Retrying...", 
                    crate::types::Status::Warn.as_colored_str(), 
                    path
                );
                let _ = std::fs::remove_file(&path);
            }
        }
    }
    Ok(())
}
