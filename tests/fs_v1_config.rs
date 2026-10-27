#![cfg(all(feature = "fs-v1", unix))]

mod common;

use std::ffi::OsString;
use std::fs;

use common::{TempDir, env_lock};
use musutils::fs::config::{get, get_value, parse_value_as, rewrite};
use serde_json::json;

struct HomeGuard {
    _g: std::sync::MutexGuard<'static, ()>,
    saved: Option<OsString>,
}

impl HomeGuard {
    fn new(home: &std::path::Path) -> Self {
        let _g = env_lock();
        let saved = std::env::var_os("HOME");
        unsafe {
            std::env::set_var("HOME", home);
        }
        Self { _g, saved }
    }
}

impl Drop for HomeGuard {
    fn drop(&mut self) {
        unsafe {
            match &self.saved {
                Some(v) => std::env::set_var("HOME", v),
                None => std::env::remove_var("HOME"),
            }
        }
    }
}

#[test]
fn get_returns_default_and_creates_file() {
    let dir = TempDir::new("fsv1_cfg_get");
    let _guard = HomeGuard::new(dir.path());

    let content = get("myapp", "conf.json", Some("{\"a\":1}"));
    assert_eq!(content, "{\"a\":1}");

    let expected = dir.path().join(".config").join("myapp").join("conf.json");
    assert!(expected.exists());
    assert_eq!(fs::read_to_string(&expected).unwrap(), "{\"a\":1}");
}

#[test]
fn get_reads_existing_file() {
    let dir = TempDir::new("fsv1_cfg_get_ex");
    let _guard = HomeGuard::new(dir.path());

    let path = dir.path().join(".config").join("myapp");
    fs::create_dir_all(&path).unwrap();
    fs::write(path.join("conf.json"), b"existing").unwrap();

    let content = get("myapp", "conf.json", Some("{\"a\":1}"));
    assert_eq!(content, "existing");
}

#[test]
#[should_panic]
fn get_panics_when_missing_and_no_default() {
    let dir = TempDir::new("fsv1_cfg_get_panic");
    let _guard = HomeGuard::new(dir.path());
    let _ = get("myapp", "missing.json", None);
}

#[test]
fn rewrite_writes_and_is_readable_via_get() {
    let dir = TempDir::new("fsv1_cfg_rw");
    let _guard = HomeGuard::new(dir.path());

    rewrite("myapp", "conf.json", "{\"k\":42}");
    let back = get("myapp", "conf.json", None);
    assert_eq!(back, "{\"k\":42}");
}

#[test]
fn rewrite_creates_dirs() {
    let dir = TempDir::new("fsv1_cfg_rw_dirs");
    let _guard = HomeGuard::new(dir.path());

    rewrite("newapp", "sub/dir/conf.json", "{}");
    let expected = dir
        .path()
        .join(".config")
        .join("newapp")
        .join("sub")
        .join("dir")
        .join("conf.json");
    assert!(expected.exists());
}

#[test]
fn get_value_existing_key() {
    let v = json!({"a": 1, "b": "x"});
    assert_eq!(get_value(&v, "a"), Some(json!(1)));
    assert_eq!(get_value(&v, "b"), Some(json!("x")));
}

#[test]
fn get_value_missing_key() {
    let v = json!({"a": 1});
    assert_eq!(get_value(&v, "nope"), None);
}

#[test]
fn get_value_non_object() {
    assert_eq!(get_value(&json!(5), "a"), None);
    assert_eq!(get_value(&json!("str"), "a"), None);
    assert_eq!(get_value(&json!(null), "a"), None);
}

#[test]
fn parse_value_as_ok() {
    assert_eq!(parse_value_as::<i32>(&json!(7)), Some(7));
    assert_eq!(
        parse_value_as::<String>(&json!("hi")),
        Some(String::from("hi"))
    );
    assert_eq!(parse_value_as::<bool>(&json!(true)), Some(true));
}

#[test]
fn parse_value_as_type_mismatch() {
    assert_eq!(parse_value_as::<i32>(&json!("str")), None);
    assert_eq!(parse_value_as::<bool>(&json!(1)), None);
}
