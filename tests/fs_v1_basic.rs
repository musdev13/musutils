#![cfg(feature = "fs-v1")]

mod common;

use std::fs;
use std::path::PathBuf;

use common::TempDir;
use musutils::fs::{
    copy, copy_dir_all, get_curr_dir, is_exist, new_dir, normalize_path, remove_dir_all,
    tilda_desir, write, write_bytes,
};

fn fake_home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

#[test]
fn get_curr_dir_matches_std() {
    let cwd = std::env::current_dir().unwrap();
    assert_eq!(get_curr_dir(), cwd);
}

#[test]
fn copy_file_success() {
    let dir = TempDir::new("fsv1_copy");
    let from = dir.join("a.txt");
    let to = dir.join("b.txt");
    fs::write(&from, b"hello").unwrap();
    let n = copy(&from, &to).unwrap();
    assert_eq!(n, 5);
    assert_eq!(fs::read(&to).unwrap(), b"hello");
}

#[test]
fn copy_missing_source_err() {
    let dir = TempDir::new("fsv1_copy_err");
    let from = dir.join("nope.txt");
    let to = dir.join("out.txt");
    assert!(copy(&from, &to).is_err());
    assert!(!to.exists());
}

#[test]
fn write_creates_parents_and_content() {
    let dir = TempDir::new("fsv1_write");
    let path = dir.join("a").join("b").join("c.txt");
    write(path.clone(), b"payload").unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"payload");
}

#[test]
fn write_overwrites_existing() {
    let dir = TempDir::new("fsv1_write_ow");
    let path = dir.join("x.txt");
    fs::write(&path, b"old").unwrap();
    write(path.clone(), b"new content").unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"new content");
}

#[test]
fn write_bytes_roundtrip() {
    let dir = TempDir::new("fsv1_wbytes");
    let path = dir.join("raw.bin");
    write_bytes(&path, [0u8, 1, 2, 255]).unwrap();
    assert_eq!(fs::read(&path).unwrap(), vec![0u8, 1, 2, 255]);
}

#[test]
fn write_bytes_missing_parent_err() {
    let dir = TempDir::new("fsv1_wbytes_err");
    let path = dir.join("nope").join("x.bin");
    assert!(write_bytes(&path, b"x").is_err());
}

#[test]
fn is_exist_true_and_false() {
    let dir = TempDir::new("fsv1_is_exist");
    let f = dir.join("f.txt");
    fs::write(&f, b"x").unwrap();
    assert!(is_exist(&f));
    assert!(!is_exist(dir.join("missing")));
}

#[test]
fn new_dir_creates_nested() {
    let dir = TempDir::new("fsv1_newdir");
    let p = dir.join("x").join("y").join("z");
    assert!(new_dir(&p));
    assert!(p.is_dir());
}

#[test]
fn new_dir_idempotent() {
    let dir = TempDir::new("fsv1_newdir2");
    let p = dir.join("a");
    assert!(new_dir(&p));
    assert!(new_dir(&p));
    assert!(p.is_dir());
}

#[test]
#[should_panic]
fn new_dir_panics_on_invalid_target() {
    let dir = TempDir::new("fsv1_newdir_panic");
    let file = dir.join("file.txt");
    fs::write(&file, b"x").unwrap();
    new_dir(file.join("sub"));
}

#[test]
fn remove_dir_all_removes_tree() {
    let dir = TempDir::new("fsv1_rm");
    let root = dir.join("tree");
    fs::create_dir_all(root.join("a").join("b")).unwrap();
    fs::write(root.join("a").join("b").join("f.txt"), b"x").unwrap();
    remove_dir_all(&root).unwrap();
    assert!(!root.exists());
}

#[test]
fn remove_dir_all_missing_err() {
    let dir = TempDir::new("fsv1_rm_err");
    assert!(remove_dir_all(dir.join("nope")).is_err());
}

#[test]
fn copy_dir_all_recursive() {
    let dir = TempDir::new("fsv1_cpdir");
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
    let dir = TempDir::new("fsv1_cpdir_err");
    assert!(copy_dir_all(dir.join("nope"), dir.join("out")).is_err());
}

#[test]
fn normalize_path_basic() {
    assert_eq!(
        normalize_path(std::path::Path::new("a/./b/../c")),
        PathBuf::from("a/c")
    );
}

#[test]
fn normalize_path_empty_and_dot() {
    assert_eq!(normalize_path(std::path::Path::new("")), PathBuf::new());
    assert_eq!(normalize_path(std::path::Path::new(".")), PathBuf::new());
}

#[test]
fn normalize_path_no_change() {
    assert_eq!(
        normalize_path(std::path::Path::new("/tmp/x/y")),
        PathBuf::from("/tmp/x/y")
    );
}

#[test]
fn tilda_desir_expands_home() {
    let home = match fake_home() {
        Some(h) => h,
        None => return,
    };
    let expanded = tilda_desir("~");
    assert_eq!(expanded, normalize_path(&home));
}

#[test]
fn tilda_desir_home_subpath() {
    let home = match fake_home() {
        Some(h) => h,
        None => return,
    };
    let expanded = tilda_desir("~/sub/file.txt");
    assert_eq!(expanded, normalize_path(&home.join("sub").join("file.txt")));
}

#[test]
fn tilda_desir_relative_joins_cwd() {
    let cwd = std::env::current_dir().unwrap();
    let expanded = tilda_desir("rel/path");
    assert_eq!(expanded, normalize_path(&cwd.join("rel").join("path")));
}

#[test]
fn tilda_desir_absolute_unchanged() {
    let expanded = tilda_desir("/tmp/abs");
    assert_eq!(expanded, PathBuf::from("/tmp/abs"));
}

#[test]
fn test_smoke() {
    musutils::fs::test();
}
