//! CI purity gate — fails if effect vocabulary appears in `bion-soma/src/`.

use std::fs;
use std::path::PathBuf;

fn src_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn read_all_rust_sources(dir: &std::path::Path) -> Vec<(PathBuf, String)> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir).expect("read src dir") {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.is_dir() {
            out.extend(read_all_rust_sources(&path));
        } else if path.extension().is_some_and(|e| e == "rs") {
            let text = fs::read_to_string(&path).expect("read source");
            out.push((path, text));
        }
    }
    out
}

#[test]
fn no_effect_vocabulary_in_src() {
    let banned_substrings = [
        "Cell",
        "RefCell",
        "Mutex",
        "RwLock",
        "Atomic",
        "static mut",
        "SystemTime",
        "Instant",
        "thread_rng",
        "getrandom",
        "rand::",
        "uuid::",
        "UuidIdGen",
        "SequentialIdGen",
        "trait IdGen",
    ];

    for (path, text) in read_all_rust_sources(&src_root()) {
        let rel = path.strip_prefix(src_root()).unwrap_or(&path);
        for needle in banned_substrings {
            assert!(
                !text.contains(needle),
                "{}: banned effect vocabulary {:?}",
                rel.display(),
                needle
            );
        }
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") || trimmed.starts_with("///") || trimmed.starts_with("//!")
            {
                continue;
            }
            if trimmed.contains("fn ") && trimmed.contains("(&mut self") {
                panic!(
                    "{}: `&mut self` method — effects belong above Soma: {}",
                    rel.display(),
                    trimmed
                );
            }
        }
    }
}

#[test]
fn no_bion_imports_in_src() {
    for (path, text) in read_all_rust_sources(&src_root()) {
        let rel = path.strip_prefix(src_root()).unwrap_or(&path);
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("use bion_") {
                panic!("{}: layer boundary violated: {}", rel.display(), trimmed);
            }
        }
    }
}

#[test]
fn no_infinite_db_vocabulary_in_src() {
    let banned = [
        "Node", "Edge", "Record", "Table", "Row", "Column", "Schema", "Query", "Index",
        "Transaction", "Commit", "Store", "Fetch", "Cursor", "Collection",
    ];
    for (path, text) in read_all_rust_sources(&src_root()) {
        let rel = path.strip_prefix(src_root()).unwrap_or(&path);
        for word in banned {
            if text.split_whitespace().any(|w| w == word) {
                panic!(
                    "{}: infinite-db vocabulary {:?} must not appear in Soma",
                    rel.display(),
                    word
                );
            }
        }
    }
}
