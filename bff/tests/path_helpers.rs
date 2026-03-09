use std::env;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

fn normalize_relative_path(path: &str) -> PathBuf {
    PathBuf::from(path.replace('\\', "/"))
}

pub fn resolve_bigfile_path(path: &str) -> PathBuf {
    let candidate = normalize_relative_path(path);
    if candidate.is_absolute() {
        return candidate;
    }

    bigfile_root().join(candidate)
}

pub fn resolve_repo_data_path(path: &str) -> PathBuf {
    let candidate = normalize_relative_path(path);
    if candidate.is_absolute() {
        return candidate;
    }

    repo_root().join("data").join(candidate)
}

fn bigfile_root() -> &'static Path {
    static BIGFILE_ROOT: OnceLock<PathBuf> = OnceLock::new();
    BIGFILE_ROOT
        .get_or_init(|| {
            let value = env::var_os("BIGFILE_DIR").expect(
                "BIGFILE_DIR is not set. Set BIGFILE_DIR to the root directory containing bigfile test inputs.",
            );
            PathBuf::from(value)
        })
        .as_path()
}

fn repo_root() -> &'static Path {
    static REPO_ROOT: OnceLock<PathBuf> = OnceLock::new();
    REPO_ROOT
        .get_or_init(|| {
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .expect("could not determine repo root from CARGO_MANIFEST_DIR")
                .to_path_buf()
        })
        .as_path()
}
