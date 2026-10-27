use std::path::Path;
use std::time::Duration;

use sha1::{Digest, Sha1};

use super::HttpError;
use super::download_to::download_to;

pub async fn download_to_sha1(
    url: impl Into<String>,
    path: impl AsRef<Path>,
    expected_sha1: impl Into<String>,
    retries: u32,
) -> Result<(), HttpError> {
    let url = url.into();
    let path = path.as_ref().to_path_buf();
    let expected = expected_sha1.into().to_lowercase();

    let mut last: Option<HttpError> = None;
    for attempt in 0..=retries {
        match download_to(url.clone(), path.clone()).await {
            Ok(()) => {
                let bytes = tokio::fs::read(&path).await?;
                let mut hasher = Sha1::new();
                hasher.update(&bytes);
                let actual: String = hasher
                    .finalize()
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect();
                if actual == expected {
                    return Ok(());
                }
                last = Some(HttpError::HashMismatch {
                    expected: expected.clone(),
                    actual,
                });
                let _ = tokio::fs::remove_file(&path).await;
            }
            Err(e) => {
                if e.is_permanent() {
                    return Err(e);
                }
                last = Some(e);
            }
        }
        if attempt < retries {
            let backoff = Duration::from_millis(200u64 * (1u64 << attempt.min(4)));
            tokio::time::sleep(backoff).await;
        }
    }
    Err(last.unwrap_or(HttpError::RetriesExceeded(retries)))
}
