extern crate tempdir;
extern crate unrar;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use tempdir::TempDir;

#[test]
fn version_list() {
    let mut entries = unrar::Archive::new("data/version.rar")
        .unwrap()
        .list()
        .unwrap();
    assert_eq!(
        entries.next().unwrap().unwrap().filename,
        PathBuf::from("VERSION")
    );
}

#[test]
fn version_cat() {
    let t = TempDir::new("unrar").unwrap();
    unrar::Archive::new("data/version.rar")
        .unwrap()
        .extract_to(t.path())
        .unwrap()
        .process()
        .unwrap();
    let mut file = File::open(t.path().join("VERSION")).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    assert_eq!(s, "unrar-0.4.0");
}

#[test]
fn filename_list() {
    let names = vec![
        "build.rs",
        "Cargo.toml",
        "examples/lister.rs",
        "src/lib.rs",
        "vendor/unrar/acknow.txt",
        "vendor/unrar/arccmt.cpp",
    ];

    let archive = unrar::Archive::new("data/archive.part1.rar").unwrap();

    for (i, entry) in archive.list().unwrap().filter_map(|e| e.ok()).enumerate() {
        assert_eq!(entry.filename.as_path().to_str().unwrap(), names[i]);
    }
}
