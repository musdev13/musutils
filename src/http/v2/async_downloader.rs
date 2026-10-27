use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use super::{HttpError, download_to, download_to_sha1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgo {
    None,
    Sha1,
}

pub struct AsyncDownloader {
    semaphore: Arc<Semaphore>,
    join_set: JoinSet<Result<(), HttpError>>,
    algo: HashAlgo,
    retries: u32,
    total: Arc<AtomicUsize>,
    done: Arc<AtomicUsize>,
    errors: Arc<AtomicUsize>,
}

impl AsyncDownloader {
    pub fn new(max_concurrent: usize, algo: HashAlgo) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent.max(1))),
            join_set: JoinSet::new(),
            algo,
            retries: 25,
            total: Arc::new(AtomicUsize::new(0)),
            done: Arc::new(AtomicUsize::new(0)),
            errors: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn with_retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
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
        let algo = self.algo;
        let retries = self.retries;
        let total = Arc::clone(&self.total);
        let done = Arc::clone(&self.done);
        let errors_counter = Arc::clone(&self.errors);

        total.fetch_add(1, Ordering::Relaxed);

        self.join_set.spawn(async move {
            let result = async {
                let _permit = sem
                    .acquire_owned()
                    .await
                    .map_err(|e| HttpError::Io(std::io::Error::other(e)))?;

                let file_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();
                let path_str = path.to_string_lossy().into_owned();

                if let Some(msg) = task_start_msg {
                    let formatted = msg.replace("{0}", &file_name).replace("{1}", &path_str);
                    println!("{formatted}");
                }

                match (algo, hash) {
                    (HashAlgo::Sha1, Some(expected)) => {
                        download_to_sha1(url, path.clone(), expected, retries).await?;
                    }
                    _ => {
                        let mut last: Option<HttpError> = None;
                        for attempt in 0..=retries {
                            match download_to(url.clone(), path.clone()).await {
                                Ok(()) => {
                                    last = None;
                                    break;
                                }
                                Err(e) => {
                                    if e.is_permanent() {
                                        return Err(e);
                                    }
                                    last = Some(e);
                                    if attempt < retries {
                                        let backoff = Duration::from_millis(
                                            200u64 * (1u64 << attempt.min(4)),
                                        )
                                        .min(Duration::from_secs(5));
                                        tokio::time::sleep(backoff).await;
                                    }
                                }
                            }
                        }
                        if let Some(e) = last {
                            return Err(e);
                        }
                    }
                }

                if let Some(msg) = success_msg {
                    let formatted = msg.replace("{0}", &file_name).replace("{1}", &path_str);
                    println!("{formatted}");
                }

                Ok::<(), HttpError>(())
            }
            .await;

            if result.is_err() {
                errors_counter.fetch_add(1, Ordering::Relaxed);
            }
            done.fetch_add(1, Ordering::Relaxed);
            result
        });
    }

    pub async fn join(&mut self) -> Result<(), Vec<HttpError>> {
        let mut errors = Vec::new();
        let started = Instant::now();
        let total = self.total.load(Ordering::Relaxed);
        let mut next_report = Instant::now() + Duration::from_secs(1);

        eprintln!("  starting {} download(s)...", total);

        loop {
            let timed =
                tokio::time::timeout(Duration::from_millis(500), self.join_set.join_next()).await;

            match timed {
                Ok(Some(result)) => match result {
                    Err(join_err) => errors.push(HttpError::Io(std::io::Error::other(join_err))),
                    Ok(Err(e)) => errors.push(e),
                    Ok(Ok(())) => {}
                },
                Ok(None) => break,
                Err(_) => {}
            }

            let now = Instant::now();
            if now >= next_report {
                let done = self.done.load(Ordering::Relaxed);
                let errors_n = self.errors.load(Ordering::Relaxed);
                let elapsed = started.elapsed().as_secs_f64();
                let rate = if elapsed > 0.0 {
                    done as f64 / elapsed
                } else {
                    0.0
                };
                eprintln!(
                    "  [{}/{}] done, {} error(s), {:.1}s elapsed, {:.1} task(s)/s",
                    done, total, errors_n, elapsed, rate
                );
                next_report = now + Duration::from_secs(1);
            }
        }

        let done = self.done.load(Ordering::Relaxed);
        let elapsed = started.elapsed().as_secs_f64();
        eprintln!(
            "  [done] {}/{} completed, {} error(s) in {:.1}s",
            done,
            total,
            errors.len(),
            elapsed
        );

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn progress(&self) -> (usize, usize, usize) {
        (
            self.done.load(Ordering::Relaxed),
            self.total.load(Ordering::Relaxed),
            self.errors.load(Ordering::Relaxed),
        )
    }
}
