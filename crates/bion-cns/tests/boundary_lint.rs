//! CNS must never reference infinite-db vocabulary.

use std::fs;
use std::path::Path;

fn read_all_rust_sources(dir: &Path, out: &mut Vec<(std::path::PathBuf, String)>) {
    for entry in fs::read_dir(dir).expect("read dir") {
        let entry = entry.expect("entry");
        let path = entry.path();
        if path.is_dir() {
            read_all_rust_sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push((path.clone(), fs::read_to_string(&path).expect("read")));
        }
    }
}

#[test]
fn no_db_vocabulary_in_cns_src() {
    let banned = [
        "infinite_db",
        "InfiniteDb",
        "SpaceId",
        "RevisionId",
        "HilbertKey",
    ];
    let src = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    read_all_rust_sources(&src, &mut files);
    for (path, text) in files {
        for term in banned {
            assert!(
                !text.contains(term),
                "{} contains banned term {term}",
                path.display()
            );
        }
    }
}
