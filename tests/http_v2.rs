#![cfg(feature = "http-v2")]

mod common;

use std::fs;
use std::path::PathBuf;

use common::{HttpConfig, TempDir, spawn_http_server};
use musutils::http::HttpError;
use musutils::http::async_downloader::{AsyncDownloader, HashAlgo};
use musutils::http::{download_to, download_to_sha1, get_file, get_github_json_str, get_json_str};

const HELLO_SHA1: &str = "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d";

#[tokio::test]
async fn get_file_returns_bytes() {
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"hello".to_vec(), 1));
    let bytes = get_file(srv.url).await.unwrap();
    assert_eq!(bytes, b"hello");
}

#[tokio::test]
async fn get_file_404_status_err() {
    let srv = spawn_http_server(HttpConfig::with_status(404, 1));
    let err = get_file(srv.url).await.unwrap_err();
    match err {
        HttpError::Status(code) => assert_eq!(code, 404),
        other => panic!("unexpected: {:?}", other),
    }
}

#[tokio::test]
async fn get_file_500_status_err() {
    let srv = spawn_http_server(HttpConfig::with_status(500, 1));
    let err = get_file(srv.url).await.unwrap_err();
    assert!(matches!(err, HttpError::Status(500)));
}

#[tokio::test]
async fn get_file_connection_refused_err() {
    let err = get_file("http://127.0.0.1:1".to_string())
        .await
        .unwrap_err();
    assert!(matches!(err, HttpError::Request(_)));
}

#[tokio::test]
async fn get_json_str_pretty() {
    let srv = spawn_http_server(HttpConfig::ok_json(b"{\"a\":1}".to_vec(), 1));
    let s = get_json_str(srv.url).await.unwrap();
    assert!(s.contains("\"a\""));
    assert!(s.contains('1'));
    assert!(s.contains('\n'));
}

#[tokio::test]
async fn get_json_str_invalid_json_err() {
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"not json".to_vec(), 1));
    let err = get_json_str(srv.url).await.unwrap_err();
    assert!(matches!(err, HttpError::Io(_)));
}

#[tokio::test]
async fn get_json_str_non_utf8_err() {
    let srv = spawn_http_server(HttpConfig::ok_bytes(vec![0xffu8, 0xfe, 0xfd], 1));
    let err = get_json_str(srv.url).await.unwrap_err();
    assert!(matches!(err, HttpError::Io(_)));
}

#[tokio::test]
async fn get_json_str_404_err() {
    let srv = spawn_http_server(HttpConfig::with_status(404, 1));
    let err = get_json_str(srv.url).await.unwrap_err();
    assert!(matches!(err, HttpError::Status(404)));
}

#[tokio::test]
async fn get_github_json_str_success() {
    let srv = spawn_http_server(HttpConfig::ok_json(b"{\"k\":\"v\"}".to_vec(), 1));
    let s = get_github_json_str(srv.url).await.unwrap();
    assert!(s.contains("\"k\""));
    assert!(s.contains("\"v\""));
}

#[tokio::test]
async fn get_github_json_str_4xx_err() {
    let srv = spawn_http_server(HttpConfig::with_status(404, 1));
    let err = get_github_json_str(srv.url).await.unwrap_err();
    assert!(matches!(err, HttpError::Status(404)));
}

#[tokio::test]
async fn get_github_json_str_invalid_json_err() {
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"<html>".to_vec(), 1));
    let err = get_github_json_str(srv.url).await.unwrap_err();
    assert!(matches!(err, HttpError::Request(_)));
}

#[tokio::test]
async fn download_to_creates_parents_and_writes() {
    let dir = TempDir::new("httpv2_dl");
    let path: PathBuf = dir.join("a").join("b").join("f.bin");
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"payload".to_vec(), 1));
    download_to(srv.url, &path).await.unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"payload");
}

#[tokio::test]
async fn download_to_overwrites() {
    let dir = TempDir::new("httpv2_dl_ow");
    let path = dir.join("f.bin");
    fs::write(&path, b"old").unwrap();
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"new".to_vec(), 1));
    download_to(srv.url, &path).await.unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"new");
}

#[tokio::test]
async fn download_to_404_err() {
    let dir = TempDir::new("httpv2_dl_404");
    let path = dir.join("f.bin");
    let srv = spawn_http_server(HttpConfig::with_status(404, 1));
    let err = download_to(srv.url, &path).await.unwrap_err();
    assert!(matches!(err, HttpError::Status(404)));
    assert!(!path.exists());
}

#[tokio::test]
async fn download_to_sha1_success() {
    let dir = TempDir::new("httpv2_sha1");
    let path = dir.join("f.bin");
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"hello".to_vec(), 1));
    download_to_sha1(srv.url, &path, HELLO_SHA1.to_string(), 0)
        .await
        .unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"hello");
}

#[tokio::test]
async fn download_to_sha1_uppercase_expected_normalized() {
    let dir = TempDir::new("httpv2_sha1_up");
    let path = dir.join("f.bin");
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"hello".to_vec(), 1));
    let upper = HELLO_SHA1.to_uppercase();
    download_to_sha1(srv.url, &path, upper, 0).await.unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"hello");
}

#[tokio::test]
async fn download_to_sha1_hash_mismatch_err() {
    let dir = TempDir::new("httpv2_sha1_bad");
    let path = dir.join("f.bin");
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"hello".to_vec(), 1));
    let err = download_to_sha1(srv.url, &path, "0".repeat(40), 0)
        .await
        .unwrap_err();
    assert!(matches!(err, HttpError::HashMismatch { .. }));
    if let HttpError::HashMismatch { expected, actual } = &err {
        assert_eq!(expected.len(), 40);
        assert_eq!(actual.len(), 40);
    }
}

#[tokio::test]
async fn download_to_sha1_permanent_status_err() {
    let dir = TempDir::new("httpv2_sha1_perm");
    let path = dir.join("f.bin");
    let srv = spawn_http_server(HttpConfig::with_status(404, 1));
    let err = download_to_sha1(srv.url, &path, HELLO_SHA1.to_string(), 0)
        .await
        .unwrap_err();
    assert!(matches!(err, HttpError::Status(404)));
}

#[test]
fn http_error_display_all_variants() {
    let s = HttpError::Status(404).to_string();
    assert!(s.contains("404"));

    let s = HttpError::HashMismatch {
        expected: "aaa".into(),
        actual: "bbb".into(),
    }
    .to_string();
    assert!(s.contains("sha1 mismatch"));
    assert!(s.contains("aaa"));
    assert!(s.contains("bbb"));

    let s = HttpError::RetriesExceeded(7).to_string();
    assert!(s.contains("retries exceeded"));
    assert!(s.contains('7'));

    let s = HttpError::Io(std::io::Error::other("boom")).to_string();
    assert!(s.contains("io error"));
    assert!(s.contains("boom"));
}

#[test]
fn http_error_is_permanent() {
    assert!(HttpError::Status(400).is_permanent());
    assert!(HttpError::Status(404).is_permanent());
    assert!(HttpError::Status(499).is_permanent());
    assert!(!HttpError::Status(500).is_permanent());
    assert!(!HttpError::Status(404 - 5).is_permanent());
    assert!(!HttpError::Status(408).is_permanent());
    assert!(!HttpError::Status(429).is_permanent());
    assert!(!HttpError::RetriesExceeded(0).is_permanent());
    assert!(
        !HttpError::HashMismatch {
            expected: String::new(),
            actual: String::new()
        }
        .is_permanent()
    );
}

#[test]
fn http_error_source() {
    use std::error::Error;
    let e = HttpError::Io(std::io::Error::other("x"));
    assert!(e.source().is_some());

    let e = HttpError::Status(404);
    assert!(e.source().is_none());
}

#[test]
fn http_error_from_io() {
    let io_err = std::io::Error::other("x");
    let e: HttpError = io_err.into();
    assert!(matches!(e, HttpError::Io(_)));
}

#[tokio::test]
async fn async_downloader_single_task_ok() {
    let dir = TempDir::new("httpv2_ad_single");
    let path = dir.join("out.bin");
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"data".to_vec(), 1));

    let mut dl = AsyncDownloader::new(2, HashAlgo::None).with_retries(0);
    dl.push(srv.url, path.clone(), None, None, None);
    let (d0, t0, e0) = dl.progress();
    assert_eq!((d0, t0, e0), (0, 1, 0));
    dl.join().await.unwrap();
    let (d1, t1, e1) = dl.progress();
    assert_eq!(d1, 1);
    assert_eq!(t1, 1);
    assert_eq!(e1, 0);
    assert_eq!(fs::read(&path).unwrap(), b"data");
}

#[tokio::test]
async fn async_downloader_two_tasks_ok() {
    let dir = TempDir::new("httpv2_ad_two");
    let p1 = dir.join("a.bin");
    let p2 = dir.join("b.bin");
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"x".to_vec(), 2));

    let mut dl = AsyncDownloader::new(2, HashAlgo::None).with_retries(0);
    dl.push(srv.url.clone(), p1.clone(), None, None, None);
    dl.push(srv.url, p2.clone(), None, None, None);
    dl.join().await.unwrap();
    assert_eq!(fs::read(&p1).unwrap(), b"x");
    assert_eq!(fs::read(&p2).unwrap(), b"x");
}

#[tokio::test]
async fn async_downloader_empty_ok() {
    let mut dl = AsyncDownloader::new(2, HashAlgo::None).with_retries(0);
    dl.join().await.unwrap();
    assert_eq!(dl.progress(), (0, 0, 0));
}

#[tokio::test]
async fn async_downloader_404_collects_error() {
    let dir = TempDir::new("httpv2_ad_404");
    let path = dir.join("missing.bin");
    let srv = spawn_http_server(HttpConfig::with_status(404, 1));

    let mut dl = AsyncDownloader::new(1, HashAlgo::None).with_retries(0);
    dl.push(srv.url, path, None, None, None);
    let errs = dl.join().await.unwrap_err();
    assert_eq!(errs.len(), 1);
    assert!(matches!(errs[0], HttpError::Status(404)));
    let (done, total, errors) = dl.progress();
    assert_eq!(done, 1);
    assert_eq!(total, 1);
    assert_eq!(errors, 1);
}

#[tokio::test]
async fn async_downloader_sha1_success() {
    let dir = TempDir::new("httpv2_ad_sha1");
    let path = dir.join("f.bin");
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"hello".to_vec(), 1));

    let mut dl = AsyncDownloader::new(1, HashAlgo::Sha1).with_retries(0);
    dl.push(
        srv.url,
        path.clone(),
        Some(HELLO_SHA1.to_string()),
        None,
        None,
    );
    dl.join().await.unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"hello");
}

#[tokio::test]
async fn async_downloader_concurrency_limit_zero_becomes_one() {
    let dir = TempDir::new("httpv2_ad_zero");
    let path = dir.join("out.bin");
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"z".to_vec(), 1));

    let mut dl = AsyncDownloader::new(0, HashAlgo::None).with_retries(0);
    dl.push(srv.url, path.clone(), None, None, None);
    dl.join().await.unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"z");
}
