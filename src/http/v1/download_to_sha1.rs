use sha1::{Digest, Sha1};
use std::path::PathBuf;

pub async fn download_to_sha1(
    url: String,
    path: PathBuf,
    expected_sha1: String,
) -> std::io::Result<()> {
    loop {
        match crate::http::download_to(url.clone(), path.clone()).await {
            Ok(()) => {
                if let Ok(file_bytes) = std::fs::read(&path) {
                    let mut hasher = Sha1::new();
                    hasher.update(&file_bytes);
                    let result = hasher.finalize();
                    let actual_sha1: String = result.iter().map(|b| format!("{:02x}", b)).collect();

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
            Err(_) => {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }
    Ok(())
}
