#![allow(dead_code)]

use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread;

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    pub fn new(prefix: &str) -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let name = format!("{}_{}_{}_{}", prefix, std::process::id(), id, nanos);
        let path = std::env::temp_dir().join(name);
        std::fs::create_dir_all(&path).expect("failed to create temp dir");
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn join(&self, p: &str) -> PathBuf {
        self.path.join(p)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

pub fn env_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|p| p.into_inner())
}

pub struct TestServer {
    pub url: String,
}

pub struct HttpConfig {
    pub status: u16,
    pub body: Vec<u8>,
    pub requests: usize,
    pub content_type: &'static str,
}

impl HttpConfig {
    pub fn ok_json(body: impl Into<Vec<u8>>, requests: usize) -> Self {
        Self {
            status: 200,
            body: body.into(),
            requests,
            content_type: "application/json",
        }
    }

    pub fn ok_bytes(body: impl Into<Vec<u8>>, requests: usize) -> Self {
        Self {
            status: 200,
            body: body.into(),
            requests,
            content_type: "application/octet-stream",
        }
    }

    pub fn with_status(status: u16, requests: usize) -> Self {
        Self {
            status,
            body: Vec::new(),
            requests,
            content_type: "text/plain",
        }
    }
}

pub fn spawn_http_server(config: HttpConfig) -> TestServer {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind failed");
    let addr = listener.local_addr().expect("local_addr");
    let url = format!("http://{}", addr);

    let status_text: &'static str = match config.status {
        200 => "OK",
        404 => "Not Found",
        408 => "Request Timeout",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        _ => "Status",
    };

    let _ = thread::spawn(move || {
        for _ in 0..config.requests {
            let (mut stream, _) = match listener.accept() {
                Ok(s) => s,
                Err(_) => return,
            };
            let mut buf = [0u8; 8192];
            let _ = stream.read(&mut buf);
            let header = format!(
                "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: {}\r\nConnection: close\r\n\r\n",
                config.status,
                status_text,
                config.body.len(),
                config.content_type,
            );
            let _ = stream.write_all(header.as_bytes());
            let _ = stream.write_all(&config.body);
            let _ = stream.flush();
        }
    });

    TestServer { url }
}
