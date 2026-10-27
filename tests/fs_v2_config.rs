#![cfg(feature = "fs-v2")]

mod common;

use std::ffi::OsString;
use std::path::Path;

use common::TempDir;
use musutils::fs::config::{
    Config, ConfigError, config_dir, config_path, get, get_value, parse_value_as, rewrite,
};
use serde_json::json;

#[cfg(unix)]
use common::env_lock;

#[cfg(unix)]
struct HomeGuard {
    _g: std::sync::MutexGuard<'static, ()>,
    saved: Option<OsString>,
}

#[cfg(unix)]
impl HomeGuard {
    fn new(home: &Path) -> Self {
        let _g = env_lock();
        let saved = std::env::var_os("HOME");
        unsafe {
            std::env::set_var("HOME", home);
        }
        Self { _g, saved }
    }
}

#[cfg(unix)]
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
fn from_value_and_accessors() {
    let cfg = Config::from_value(json!({"a": 1}));
    assert_eq!(cfg.as_value(), &json!({"a": 1}));
    assert_eq!(cfg.path(), Path::new(""));
}

#[test]
fn get_top_level() {
    let cfg = Config::from_value(json!({"a": 1, "b": "x"}));
    assert_eq!(cfg.get("a"), Some(&json!(1)));
    assert_eq!(cfg.get("b"), Some(&json!("x")));
}

#[test]
fn get_nested() {
    let cfg = Config::from_value(json!({"a": {"b": {"c": 42}}}));
    assert_eq!(cfg.get("a.b.c"), Some(&json!(42)));
}

#[test]
fn get_missing_key_returns_none() {
    let cfg = Config::from_value(json!({"a": 1}));
    assert_eq!(cfg.get("x"), None);
    assert_eq!(cfg.get("a.b"), None);
}

#[test]
fn get_empty_segment_returns_none() {
    let cfg = Config::from_value(json!({"a": {"b": 1}}));
    assert_eq!(cfg.get(""), None);
    assert_eq!(cfg.get("a..b"), None);
    assert_eq!(cfg.get(".a"), None);
    assert_eq!(cfg.get("a."), None);
}

#[test]
fn get_non_object_returns_none() {
    let cfg = Config::from_value(json!({"a": 5}));
    assert_eq!(cfg.get("a.b"), None);
    let cfg = Config::from_value(json!(5));
    assert_eq!(cfg.get("a"), None);
}

#[test]
fn get_as_ok() {
    let cfg = Config::from_value(json!({"a": 7, "s": "hi", "b": true}));
    assert_eq!(cfg.get_as::<i32>("a").unwrap(), 7);
    assert_eq!(cfg.get_as::<String>("s").unwrap(), "hi");
    assert!(cfg.get_as::<bool>("b").unwrap());
}

#[test]
fn get_as_missing_key_err() {
    let cfg = Config::from_value(json!({}));
    let err = cfg.get_as::<i32>("x").unwrap_err();
    assert!(matches!(err, ConfigError::KeyNotFound(_)));
    assert!(err.to_string().contains("x"));
}

#[test]
fn get_as_type_mismatch_err() {
    let cfg = Config::from_value(json!({"a": "str"}));
    let err = cfg.get_as::<i32>("a").unwrap_err();
    assert!(matches!(err, ConfigError::TypeMismatch { .. }));
    if let ConfigError::TypeMismatch { key, expected } = &err {
        assert_eq!(key, "a");
        assert!(expected.contains("i32"));
    }
}

#[test]
fn get_or_returns_default_on_missing() {
    let cfg = Config::from_value(json!({}));
    assert_eq!(cfg.get_or::<i32>("x", 99), 99);
}

#[test]
fn get_or_returns_default_on_mismatch() {
    let cfg = Config::from_value(json!({"a": "str"}));
    assert_eq!(cfg.get_or::<i32>("a", 99), 99);
}

#[test]
fn get_or_returns_value_on_hit() {
    let cfg = Config::from_value(json!({"a": 5}));
    assert_eq!(cfg.get_or::<i32>("a", 99), 5);
}

#[test]
fn set_creates_nested_path() {
    let mut cfg = Config::from_value(json!({}));
    cfg.set("a.b.c", json!(1)).unwrap();
    assert_eq!(cfg.get("a.b.c"), Some(&json!(1)));
}

#[test]
fn set_overwrites_existing() {
    let mut cfg = Config::from_value(json!({"a": {"b": 1}}));
    cfg.set("a.b", json!(2)).unwrap();
    assert_eq!(cfg.get("a.b"), Some(&json!(2)));
}

#[test]
fn set_intermediate_non_object_value_err() {
    let mut cfg = Config::from_value(json!({"a": 5}));
    let err = cfg.set("a.b", json!(1)).unwrap_err();
    assert!(matches!(err, ConfigError::InvalidKeyPath(_)));
    assert_eq!(cfg.get("a"), Some(&json!(5)));
}

#[test]
fn set_empty_key_err() {
    let mut cfg = Config::from_value(json!({}));
    let err = cfg.set("", json!(1)).unwrap_err();
    assert!(matches!(err, ConfigError::InvalidKeyPath(_)));
}

#[test]
fn set_key_with_empty_segment_err() {
    let mut cfg = Config::from_value(json!({}));
    for key in ["a..b", ".a", "a."] {
        let err = cfg.set(key, json!(1)).unwrap_err();
        assert!(matches!(err, ConfigError::InvalidKeyPath(_)), "key {}", key);
    }
}

#[test]
fn set_replaces_root_if_not_object() {
    let mut cfg = Config::from_value(json!(5));
    cfg.set("a", json!(1)).unwrap();
    assert_eq!(cfg.get("a"), Some(&json!(1)));
}

#[test]
fn set_intermediate_not_object_err() {
    let mut cfg = Config::from_value(json!({"a": {"b": 1}}));
    let err = cfg.set("a.b.c", json!(2)).unwrap_err();
    assert!(matches!(err, ConfigError::InvalidKeyPath(_)));
}

#[test]
fn save_without_path_err() {
    let cfg = Config::from_value(json!({"a": 1}));
    let err = cfg.save().unwrap_err();
    match err {
        ConfigError::Io(e) => assert_eq!(e.kind(), std::io::ErrorKind::InvalidInput),
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn save_to_and_load_path_roundtrip() {
    let dir = TempDir::new("fsv2_cfg_save");
    let path = dir.join("cfg.json");
    let cfg = Config::from_value(json!({"a": 1, "b": {"c": "x"}}));
    cfg.save_to(&path).unwrap();
    let loaded = Config::load_path(&path).unwrap();
    assert_eq!(loaded.as_value(), cfg.as_value());
    assert_eq!(loaded.path(), path.as_path());
}

#[test]
fn save_to_pretty_printed() {
    let dir = TempDir::new("fsv2_cfg_pretty");
    let path = dir.join("cfg.json");
    let cfg = Config::from_value(json!({"a": 1}));
    cfg.save_to(&path).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(text.contains('\n'));
}

#[test]
fn save_after_load_uses_stored_path() {
    let dir = TempDir::new("fsv2_cfg_save_stored");
    let path = dir.join("cfg.json");
    std::fs::write(&path, b"{\"k\":1}").unwrap();
    let mut cfg = Config::load_path(&path).unwrap();
    cfg.set("k", json!(2)).unwrap();
    cfg.save().unwrap();
    let reloaded = Config::load_path(&path).unwrap();
    assert_eq!(reloaded.get_as::<i32>("k").unwrap(), 2);
}

#[test]
fn load_path_missing_file_err() {
    let dir = TempDir::new("fsv2_cfg_load_missing");
    let err = Config::load_path(dir.join("nope.json")).err().unwrap();
    assert!(matches!(err, ConfigError::Io(_)));
}

#[test]
fn load_path_invalid_json_err() {
    let dir = TempDir::new("fsv2_cfg_load_invalid");
    let path = dir.join("bad.json");
    std::fs::write(&path, b"{ not valid").unwrap();
    let err = Config::load_path(&path).err().unwrap();
    assert!(matches!(err, ConfigError::Parse(_)));
}

#[test]
fn config_error_display_all_variants() {
    let io = ConfigError::Io(std::io::Error::other("boom"));
    assert!(io.to_string().contains("io error"));

    let parse_err = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
    let parse = ConfigError::Parse(parse_err);
    assert!(parse.to_string().contains("parse error"));

    let knf = ConfigError::KeyNotFound("a.b".into());
    assert!(knf.to_string().contains("key not found"));
    assert!(knf.to_string().contains("a.b"));

    let tm = ConfigError::TypeMismatch {
        key: "k".into(),
        expected: "u32",
    };
    assert!(tm.to_string().contains("type mismatch"));

    let ikp = ConfigError::InvalidKeyPath("a..b".into());
    assert!(ikp.to_string().contains("invalid key path"));
}

#[test]
fn config_error_source() {
    use std::error::Error;
    let io = ConfigError::Io(std::io::Error::other("boom"));
    assert!(io.source().is_some());

    let parse_err = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
    let parse = ConfigError::Parse(parse_err);
    assert!(parse.source().is_some());

    assert!(ConfigError::KeyNotFound("x".into()).source().is_none());
    assert!(
        ConfigError::TypeMismatch {
            key: "k".into(),
            expected: "t"
        }
        .source()
        .is_none()
    );
    assert!(ConfigError::InvalidKeyPath("k".into()).source().is_none());
}

#[test]
fn config_error_from_io_and_parse() {
    let io_err = std::io::Error::other("x");
    let ce: ConfigError = io_err.into();
    assert!(matches!(ce, ConfigError::Io(_)));

    let parse_err = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
    let ce: ConfigError = parse_err.into();
    assert!(matches!(ce, ConfigError::Parse(_)));
}

#[test]
fn get_value_and_parse_value_as() {
    let v = json!({"a": 1, "s": "x"});
    assert_eq!(get_value(&v, "a"), Some(json!(1)));
    assert_eq!(get_value(&v, "s"), Some(json!("x")));
    assert_eq!(get_value(&v, "missing"), None);
    assert_eq!(get_value(&json!(5), "a"), None);

    assert_eq!(parse_value_as::<i32>(&json!(5)), Some(5));
    assert_eq!(parse_value_as::<i32>(&json!("str")), None);
}

#[cfg(unix)]
#[test]
fn config_dir_uses_home() {
    let dir = TempDir::new("fsv2_cfg_dir");
    let _g = HomeGuard::new(dir.path());
    assert_eq!(
        config_dir("myapp"),
        dir.path().join(".config").join("myapp")
    );
}

#[cfg(unix)]
#[test]
fn config_path_joins_file() {
    let dir = TempDir::new("fsv2_cfg_path");
    let _g = HomeGuard::new(dir.path());
    assert_eq!(
        config_path("myapp", "conf.json"),
        dir.path().join(".config").join("myapp").join("conf.json")
    );
}

#[cfg(unix)]
#[test]
fn file_get_creates_default_and_returns_it() {
    let dir = TempDir::new("fsv2_cfg_file_get");
    let _g = HomeGuard::new(dir.path());
    let content = get("myapp", "conf.json", Some("{\"a\":1}")).unwrap();
    assert_eq!(content, "{\"a\":1}");
    let path = dir.path().join(".config").join("myapp").join("conf.json");
    assert!(path.exists());
}

#[cfg(unix)]
#[test]
fn file_get_reads_existing() {
    let dir = TempDir::new("fsv2_cfg_file_ex");
    let _g = HomeGuard::new(dir.path());
    let path = dir.path().join(".config").join("myapp");
    std::fs::create_dir_all(&path).unwrap();
    std::fs::write(path.join("conf.json"), b"existing").unwrap();
    let content = get("myapp", "conf.json", Some("{}")).unwrap();
    assert_eq!(content, "existing");
}

#[cfg(unix)]
#[test]
fn file_get_missing_no_default_err() {
    let dir = TempDir::new("fsv2_cfg_file_miss");
    let _g = HomeGuard::new(dir.path());
    let err = get("myapp", "nope.json", None).unwrap_err();
    assert!(matches!(err, ConfigError::Io(_)));
}

#[cfg(unix)]
#[test]
fn file_rewrite_then_get() {
    let dir = TempDir::new("fsv2_cfg_file_rw");
    let _g = HomeGuard::new(dir.path());
    rewrite("myapp", "conf.json", "{\"k\":1}").unwrap();
    assert_eq!(get("myapp", "conf.json", None).unwrap(), "{\"k\":1}");
}

#[cfg(unix)]
#[test]
fn load_by_app_name() {
    let dir = TempDir::new("fsv2_cfg_load");
    let _g = HomeGuard::new(dir.path());
    rewrite("myapp", "conf.json", "{\"a\":42}").unwrap();
    let cfg = Config::load("myapp", "conf.json").unwrap();
    assert_eq!(cfg.get_as::<i32>("a").unwrap(), 42);
    assert_eq!(
        cfg.path(),
        dir.path().join(".config").join("myapp").join("conf.json")
    );
}

#[cfg(unix)]
#[test]
fn load_or_create_writes_and_loads_default() {
    let dir = TempDir::new("fsv2_cfg_loc");
    let _g = HomeGuard::new(dir.path());
    let cfg = Config::load_or_create("myapp", "conf.json", "{\"x\":1}").unwrap();
    assert_eq!(cfg.get_as::<i32>("x").unwrap(), 1);
    let path = dir.path().join(".config").join("myapp").join("conf.json");
    assert!(path.exists());
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "{\"x\":1}");
}

#[cfg(unix)]
#[test]
fn load_or_create_uses_existing_file() {
    let dir = TempDir::new("fsv2_cfg_loc_ex");
    let _g = HomeGuard::new(dir.path());
    rewrite("myapp", "conf.json", "{\"x\":9}").unwrap();
    let cfg = Config::load_or_create("myapp", "conf.json", "{\"x\":1}").unwrap();
    assert_eq!(cfg.get_as::<i32>("x").unwrap(), 9);
}

#[cfg(unix)]
#[test]
fn load_or_create_invalid_default_err() {
    let dir = TempDir::new("fsv2_cfg_loc_bad");
    let _g = HomeGuard::new(dir.path());
    let err = Config::load_or_create("myapp", "conf.json", "not json")
        .err()
        .unwrap();
    assert!(matches!(err, ConfigError::Parse(_)));
}
