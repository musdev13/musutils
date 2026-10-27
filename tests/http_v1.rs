#![cfg(feature = "http-v1")]

mod common;

use std::fs;
use std::path::PathBuf;

use common::{HttpConfig, TempDir, spawn_http_server};
use musutils::http::{
    AsyncDownloader, download_to, download_to_sha1, get_file, get_github_json_str, get_json_str,
};

const HELLO_SHA1: &str = "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d";

#[tokio::test]
async fn get_file_returns_bytes() {
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"hello".to_vec(), 1));
    let bytes = get_file(srv.url).await.unwrap();
    assert_eq!(bytes, b"hello");
}

#[tokio::test]
async fn get_file_404_still_returns_body() {
    let mut cfg = HttpConfig::with_status(404, 1);
    cfg.body = b"not found".to_vec();
    let srv = spawn_http_server(cfg);
    let bytes = get_file(srv.url).await.unwrap();
    assert_eq!(bytes, b"not found");
}

#[tokio::test]
async fn get_file_connection_refused_err() {
    let url = "http://127.0.0.1:1".to_string();
    assert!(get_file(url).await.is_err());
}

#[tokio::test]
async fn get_json_str_pretty_prints() {
    let srv = spawn_http_server(HttpConfig::ok_json(b"{\"a\":1}".to_vec(), 1));
    let s = get_json_str(srv.url).await.unwrap();
    assert!(s.contains("\"a\""));
    assert!(s.contains('1'));
    assert!(s.contains('\n'));
}

#[tokio::test]
async fn get_json_str_invalid_json_err() {
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"not json".to_vec(), 1));
    assert!(get_json_str(srv.url).await.is_err());
}

#[tokio::test]
async fn get_github_json_str_success() {
    let srv = spawn_http_server(HttpConfig::ok_json(b"{\"k\":\"v\"}".to_vec(), 1));
    let s = get_github_json_str(srv.url).await.unwrap();
    assert!(s.contains("\"k\""));
    assert!(s.contains("\"v\""));
}

#[tokio::test]
async fn get_github_json_str_invalid_json_err() {
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"<html>".to_vec(), 1));
    assert!(get_github_json_str(srv.url).await.is_err());
}

#[tokio::test]
async fn download_to_writes_file() {
    let dir = TempDir::new("httpv1_dl");
    let path: PathBuf = dir.join("a").join("b").join("f.bin");
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"payload".to_vec(), 1));
    download_to(srv.url, path.clone()).await.unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"payload");
}

#[tokio::test]
async fn download_to_overwrites() {
    let dir = TempDir::new("httpv1_dl_ow");
    let path = dir.join("f.bin");
    fs::write(&path, b"old").unwrap();
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"new".to_vec(), 1));
    download_to(srv.url, path.clone()).await.unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"new");
}

#[tokio::test]
async fn download_to_sha1_success() {
    let dir = TempDir::new("httpv1_sha1");
    let path = dir.join("f.bin");
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"hello".to_vec(), 1));
    download_to_sha1(srv.url, path.clone(), HELLO_SHA1.to_string())
        .await
        .unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"hello");
}

#[tokio::test]
async fn async_downloader_single_task_ok() {
    let dir = TempDir::new("httpv1_ad_single");
    let path = dir.join("out.bin");
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"data".to_vec(), 1));

    let mut dl = AsyncDownloader::new(2, musutils::http::async_downloader::HashAlgo::None);
    dl.push(srv.url, path.clone(), None, None, None);
    dl.join().await.unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"data");
}

#[tokio::test]
async fn async_downloader_two_tasks_ok() {
    let dir = TempDir::new("httpv1_ad_two");
    let p1 = dir.join("a.bin");
    let p2 = dir.join("b.bin");
    let srv = spawn_http_server(HttpConfig::ok_bytes(b"x".to_vec(), 2));

    let mut dl = AsyncDownloader::new(2, musutils::http::async_downloader::HashAlgo::None);
    dl.push(srv.url.clone(), p1.clone(), None, None, None);
    dl.push(srv.url, p2.clone(), None, None, None);
    dl.join().await.unwrap();
    assert_eq!(fs::read(&p1).unwrap(), b"x");
    assert_eq!(fs::read(&p2).unwrap(), b"x");
}

#[tokio::test]
async fn async_downloader_empty_ok() {
    let mut dl = AsyncDownloader::new(2, musutils::http::async_downloader::HashAlgo::None);
    dl.join().await.unwrap();
}
