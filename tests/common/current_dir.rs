use std::env;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};

static CURRENT_DIR_LOCK: Mutex<()> = Mutex::new(());

pub struct CurrentDirGuard {
    previous_dir: PathBuf,
    _lock: MutexGuard<'static, ()>,
}

impl CurrentDirGuard {
    pub fn change_to(path: &Path) -> Self {
        let lock = CURRENT_DIR_LOCK.lock().unwrap();
        let previous_dir = env::current_dir().unwrap();

        env::set_current_dir(path).unwrap();

        Self {
            previous_dir,
            _lock: lock,
        }
    }
}

impl Drop for CurrentDirGuard {
    fn drop(&mut self) {
        env::set_current_dir(&self.previous_dir).unwrap();
    }
}
