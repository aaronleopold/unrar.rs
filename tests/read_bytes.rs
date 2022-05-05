extern crate tempdir;
extern crate unrar;

#[test]
fn read_bytes() {
    let archive = unrar::Archive::new("data/archive.part1.rar").unwrap();

    let bytes = archive
        .list_extract()
        .unwrap()
        .filter_map(|e| e.ok())
        .nth(0)
        .unwrap()
        .read_bytes()
        .unwrap();

    let s = match std::str::from_utf8(&bytes) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    assert_eq!(s, "[package]\nname = \"unrar_sys\"\nversion = \"0.2.0\"\nauthors = [\"Danyel Bayraktar <danyel@webhippie.de>\"]\n\nbuild = \"build.rs\"\nlicense = \"MIT\"\ndescription = \"FFI bindings to unrar (with minimal abstractions)\"\nrepository = \"https://github.com/muja/unrar.rs\"\n\n[dependencies]\nlibc = \"0.1\"\n\n[build-dependencies]\ngcc = \"0.3\"\n");
}
