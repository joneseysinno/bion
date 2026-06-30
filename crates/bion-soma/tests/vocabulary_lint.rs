//! Fail if any `DB_TERMS` token from workspace `VOCABULARY.md` appears in Soma.

use std::fs;
use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read_db_terms(vocab_path: &Path) -> Vec<String> {
    let text = fs::read_to_string(vocab_path).expect("read VOCABULARY.md");
    let mut in_section = false;
    let mut terms = Vec::new();
    for line in text.lines() {
        if line.trim() == "## DB_TERMS" {
            in_section = true;
            continue;
        }
        if line.starts_with("## ") && line.trim() != "## DB_TERMS" {
            in_section = false;
            continue;
        }
        if in_section && !line.trim().is_empty() {
            for part in line.split(',') {
                let term = part.trim().trim_end_matches('.');
                if !term.is_empty() {
                    terms.push(term.to_string());
                }
            }
        }
    }
    terms
}

fn read_all_rust_sources(dir: &Path) -> Vec<(PathBuf, String)> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir).expect("read dir") {
        let entry = entry.expect("entry");
        let path = entry.path();
        if path.is_dir() {
            out.extend(read_all_rust_sources(&path));
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push((path.clone(), fs::read_to_string(&path).expect("read source")));
        }
    }
    out
}

#[test]
fn db_terms_absent_from_soma_src() {
    let terms = read_db_terms(&workspace_root().join("VOCABULARY.md"));
    assert!(!terms.is_empty(), "DB_TERMS section must list tokens");
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    for (path, text) in read_all_rust_sources(&src) {
        let rel = path.strip_prefix(&src).unwrap_or(&path);
        for term in &terms {
            assert!(
                !text.contains(term.as_str()),
                "{}: DB term {:?} must not appear in Soma",
                rel.display(),
                term
            );
        }
    }
}
