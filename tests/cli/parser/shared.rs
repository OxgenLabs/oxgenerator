use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

static CURRENT_DIR_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

pub fn current_dir_lock() -> &'static Mutex<()> {
    CURRENT_DIR_LOCK.get_or_init(|| Mutex::new(()))
}

pub struct CurrentDirGuard {
    original_dir: PathBuf,
}

impl CurrentDirGuard {
    pub fn enter(path: PathBuf) -> Self {
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(path).unwrap();

        Self { original_dir }
    }
}

impl Drop for CurrentDirGuard {
    fn drop(&mut self) {
        std::env::set_current_dir(&self.original_dir).unwrap();
    }
}

pub fn create_oxgen_project(database: &str) -> tempfile::TempDir {
    let temp_dir = tempfile::tempdir().unwrap();

    fs::write(
        temp_dir.path().join("Cargo.toml"),
        "[package]\nname = \"test-api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .unwrap();

    fs::create_dir_all(temp_dir.path().join("src")).unwrap();
    fs::create_dir_all(temp_dir.path().join(".oxgen")).unwrap();

    fs::write(
        temp_dir.path().join(".oxgen").join("config.toml"),
        format!("generator = \"oxgen\"\ndatabase = \"{}\"\n", database),
    )
    .unwrap();

    temp_dir
}
