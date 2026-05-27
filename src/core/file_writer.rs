use std::fs;
use std::path::Path;

use crate::core::error::OxgenError;
use crate::core::result::OxgenResult;

pub struct FileWriter {
    force: bool,
    dry_run: bool,
}

impl FileWriter {
    pub fn new(force: bool, dry_run: bool) -> Self {
        Self { force, dry_run }
    }

    pub fn write_file<P: AsRef<Path>>(&self, path: P, content: &str) -> OxgenResult<()> {
        let path = path.as_ref();

        if path.exists() && !self.force {
            return Err(OxgenError::FileAlreadyExists(
                path.display().to_string(),
            ));
        }

        if self.dry_run {
            println!("[CREATE] {}", path.display());
            return Ok(());
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, content)?;
        println!("[CREATE] {}", path.display());

        Ok(())
    }

    pub fn create_dir<P: AsRef<Path>>(&self, path: P) -> OxgenResult<()> {
        let path = path.as_ref();

        if self.dry_run {
            println!("[CREATE] {}", path.display());
            return Ok(());
        }

        fs::create_dir_all(path)?;
        println!("[CREATE] {}", path.display());

        Ok(())
    }
}
