use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

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
        task_start_msg: Option<String>,
        success_msg: Option<String>,
    ) {
        let sem = Arc::clone(&self.semaphore);
        let algo = match &self.algo {
            HashAlgo::None => HashAlgo::None,
            HashAlgo::Sha1 => HashAlgo::Sha1,
        };

        self.join_set.spawn(async move {
            let _permit = sem.acquire_owned().await
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            let file_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            let path_str = path.to_str().unwrap_or("");

            if let Some(msg) = task_start_msg {
                let formatted = msg
                    .replace("{0}", file_name)
                    .replace("{1}", path_str);
                println!("{}", formatted);
            }

            match (algo, hash) {
                (HashAlgo::Sha1, Some(expected_hash)) => {
                    crate::http::download_to_sha1(url, path.clone(), expected_hash).await?;
                }
                _ => {
                    loop {
                        if crate::http::download_to(url.clone(), path.clone()).await.is_ok() {
                            break;
                        }
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                }
            }

            if let Some(msg) = success_msg {
                let formatted = msg
                    .replace("{0}", file_name)
                    .replace("{1}", path_str);
                println!("{}", formatted);
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
