#![cfg(feature = "fs-v2")]

mod common;

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use common::{TempDir, env_lock};
use musutils::fs::{
    copy, copy_dir_all, create_dir_all, current_dir, exists, expand, get_curr_dir, is_exist,
    new_dir, normalize, normalize_path, remove_dir_all, tilda_desir, write, write_atomic,
    write_bytes,
};

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
fn current_dir_and_get_curr_dir_match_std() {
    let cwd = std::env::current_dir().unwrap();
    assert_eq!(current_dir().unwrap(), cwd);
    assert_eq!(get_curr_dir(), cwd);
}

#[test]
fn copy_file_success() {
    let dir = TempDir::new("fsv2_copy");
    let from = dir.join("a.txt");
    let to = dir.join("b.txt");
    fs::write(&from, b"hello").unwrap();
    let n = copy(&from, &to).unwrap();
    assert_eq!(n, 5);
    assert_eq!(fs::read(&to).unwrap(), b"hello");
}

#[test]
fn copy_missing_source_err() {
    let dir = TempDir::new("fsv2_copy_err");
    assert!(copy(dir.join("nope"), dir.join("out")).is_err());
}

#[test]
fn create_dir_all_nested() {
    let dir = TempDir::new("fsv2_mkdir");
    let p = dir.join("a").join("b").join("c");
    create_dir_all(&p).unwrap();
    assert!(p.is_dir());
}

#[test]
fn create_dir_all_on_file_err() {
    let dir = TempDir::new("fsv2_mkdir_err");
    let f = dir.join("f.txt");
    fs::write(&f, b"x").unwrap();
    assert!(create_dir_all(f.join("sub")).is_err());
}

#[test]
fn new_dir_true_and_false() {
    let dir = TempDir::new("fsv2_newdir");
    let p = dir.join("x").join("y");
    assert!(new_dir(&p));
    assert!(p.is_dir());

    let file = dir.join("file.txt");
    fs::write(&file, b"x").unwrap();
    assert!(!new_dir(file.join("sub")));
}

#[test]
fn exists_and_is_exist() {
    let dir = TempDir::new("fsv2_exists");
    let f = dir.join("f.txt");
    assert!(!exists(&f));
    assert!(!is_exist(&f));
    fs::write(&f, b"x").unwrap();
    assert!(exists(&f));
    assert!(is_exist(&f));
    assert!(!exists(dir.join("missing")));
}

#[test]
fn write_creates_parents() {
    let dir = TempDir::new("fsv2_write");
    let p = dir.join("a").join("b").join("c.txt");
    write(&p, b"data").unwrap();
    assert_eq!(fs::read(&p).unwrap(), b"data");
}

#[test]
fn write_overwrites() {
    let dir = TempDir::new("fsv2_write_ow");
    let p = dir.join("x.txt");
    fs::write(&p, b"old").unwrap();
    write(&p, b"new").unwrap();
    assert_eq!(fs::read(&p).unwrap(), b"new");
}

#[test]
fn write_atomic_success_content() {
    let dir = TempDir::new("fsv2_atomic");
    let p = dir.join("a.txt");
    write_atomic(&p, b"data").unwrap();
    assert_eq!(fs::read(&p).unwrap(), b"data");
}

#[test]
fn write_atomic_no_tmp_left_behind() {
    let dir = TempDir::new("fsv2_atomic_tmp");
    let p = dir.join("a.txt");
    write_atomic(&p, b"v1").unwrap();
    write_atomic(&p, b"v2").unwrap();
    let entries: Vec<_> = fs::read_dir(dir.path()).unwrap().collect();
    assert_eq!(entries.len(), 1);
    assert_eq!(fs::read(&p).unwrap(), b"v2");
}

#[test]
fn write_atomic_creates_parents() {
    let dir = TempDir::new("fsv2_atomic_parents");
    let p = dir.join("x").join("y").join("a.txt");
    write_atomic(&p, b"z").unwrap();
    assert_eq!(fs::read(&p).unwrap(), b"z");
}

#[test]
fn write_atomic_no_file_name_err() {
    let err = write_atomic(Path::new("/"), b"x").unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
}

#[test]
fn write_bytes_roundtrip() {
    let dir = TempDir::new("fsv2_wbytes");
    let p = dir.join("raw.bin");
    write_bytes(&p, [0u8, 1, 2, 255]).unwrap();
    assert_eq!(fs::read(&p).unwrap(), vec![0u8, 1, 2, 255]);
}

#[test]
fn remove_dir_all_removes_tree() {
    let dir = TempDir::new("fsv2_rm");
    let root = dir.join("tree");
    fs::create_dir_all(root.join("a").join("b")).unwrap();
    fs::write(root.join("a").join("b").join("f.txt"), b"x").unwrap();
    remove_dir_all(&root).unwrap();
    assert!(!root.exists());
}

#[test]
fn remove_dir_all_missing_err() {
    let dir = TempDir::new("fsv2_rm_err");
    assert!(remove_dir_all(dir.join("nope")).is_err());
}

#[test]
fn copy_dir_all_recursive() {
    let dir = TempDir::new("fsv2_cpdir");
    let src = dir.join("src");
    fs::create_dir_all(src.join("a").join("b")).unwrap();
    fs::write(src.join("top.txt"), b"t").unwrap();
    fs::write(src.join("a").join("mid.txt"), b"m").unwrap();
    fs::write(src.join("a").join("b").join("deep.txt"), b"d").unwrap();

    let dst = dir.join("dst");
    copy_dir_all(&src, &dst).unwrap();

    assert_eq!(fs::read(dst.join("top.txt")).unwrap(), b"t");
    assert_eq!(fs::read(dst.join("a").join("mid.txt")).unwrap(), b"m");
    assert_eq!(
        fs::read(dst.join("a").join("b").join("deep.txt")).unwrap(),
        b"d"
    );
}

#[test]
fn copy_dir_all_missing_src_err() {
    let dir = TempDir::new("fsv2_cpdir_err");
    assert!(copy_dir_all(dir.join("nope"), dir.join("out")).is_err());
}

#[test]
fn normalize_basic() {
    assert_eq!(normalize(Path::new("a/./b/../c")), PathBuf::from("a/c"));
}

#[test]
fn normalize_empty_and_dot() {
    assert_eq!(normalize(Path::new("")), PathBuf::new());
    assert_eq!(normalize(Path::new(".")), PathBuf::new());
}

#[test]
fn normalize_leading_parent_kept() {
    assert_eq!(normalize(Path::new("../a")), PathBuf::from("../a"));
    assert_eq!(normalize(Path::new("../../a")), PathBuf::from("../../a"));
}

#[test]
fn normalize_root_parent_collapses() {
    assert_eq!(normalize(Path::new("/..")), PathBuf::from("/"));
    assert_eq!(normalize(Path::new("/../..")), PathBuf::from("/"));
}

#[test]
fn normalize_relative_parent() {
    assert_eq!(normalize(Path::new("a/../../b")), PathBuf::from("../b"));
}

#[test]
fn normalize_path_alias() {
    assert_eq!(
        normalize_path(Path::new("a/./b")),
        normalize(Path::new("a/./b"))
    );
}

#[test]
fn expand_absolute_unchanged() {
    assert_eq!(expand(Path::new("/tmp/x")), PathBuf::from("/tmp/x"));
}

#[test]
fn expand_relative_joins_cwd() {
    let cwd = std::env::current_dir().unwrap();
    let got = expand(Path::new("rel/x"));
    assert_eq!(got, normalize(&cwd.join("rel").join("x")));
}

#[cfg(unix)]
#[test]
fn expand_tilda_uses_home() {
    let dir = TempDir::new("fsv2_expand_home");
    let _guard = HomeGuard::new(dir.path());
    let got = expand(Path::new("~/sub/file"));
    assert_eq!(got, dir.path().join("sub").join("file"));
}

#[test]
fn tilda_desir_alias_for_expand() {
    let abs = Path::new("/tmp/abs");
    assert_eq!(tilda_desir(abs), expand(abs));
}

#[test]
fn test_smoke() {
    musutils::fs::test();
}
