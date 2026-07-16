use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use sha1::{Sha1, Digest};

pub enum HashAlgo {
    None,
    Sha1,
}

pub struct AsyncDownloader {
    semaphore: Arc<Semaphore>,
    join_set: JoinSet<std::io::Result<()>>,
    algo: HashAlgo,
}

impl AsyncDownloader {
    pub fn new(max_concurrent: usize, algo: HashAlgo) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            join_set: JoinSet::new(),
            algo,
        }
    }

    pub fn push(
        &mut self,
        url: String,
        path: PathBuf,
        hash: Option<String>,
    ) {
        let sem = Arc::clone(&self.semaphore);
        let algo = match &self.algo {
            HashAlgo::None => HashAlgo::None,
            HashAlgo::Sha1 => HashAlgo::Sha1,
        };

        self.join_set.spawn(async move {
            let _permit = sem.acquire_owned().await
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            match (algo, hash) {
                (HashAlgo::Sha1, Some(expected_hash)) => {
                    loop {
                        crate::http::download_to(url.clone(), path.clone()).await?;

                        if let Ok(file_bytes) = std::fs::read(&path) {
                            let mut hasher = Sha1::new();
                            hasher.update(&file_bytes);
                            let result = hasher.finalize();
                            let actual_hash = format!("{:x}", result);

                            if actual_hash == expected_hash {
                                break;
                            } else {
                                let _ = std::fs::remove_file(&path);
                            }
                        }
                    }
                }
                _ => {
                    crate::http::download_to(url, path).await?;
                }
            }

            Ok(())
        });
    }

    pub async fn join(&mut self) -> std::io::Result<()> {
        while let Some(result) = self.join_set.join_next().await {
            match result {
                Err(join_err) => {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, join_err));
                }
                Ok(Err(io_err)) => {
                    return Err(io_err);
                }
                Ok(Ok(())) => {}
            }
        }
        Ok(())
    }
}
