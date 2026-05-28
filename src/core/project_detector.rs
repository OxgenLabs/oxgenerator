use std::path::Path;

use crate::core::error::OxgenError;
use crate::core::result::OxgenResult;

fn ensure_rust_project_root() -> OxgenResult<()> {
    let cargo_toml = Path::new("Cargo.toml");
    let src_dir = Path::new("src");

    if !cargo_toml.exists() || !cargo_toml.is_file() {
        return Err(OxgenError::ProjectNotFound);
    }

    if !src_dir.exists() || !src_dir.is_dir() {
        return Err(OxgenError::ProjectNotFound);
    }

    Ok(())
}

pub fn ensure_oxgen_project_root() -> OxgenResult<()> {
    ensure_rust_project_root()?;

    let oxgen_config = Path::new(".oxgen").join("config.toml");

    if !oxgen_config.exists() || !oxgen_config.is_file() {
        return Err(OxgenError::OxgenProjectNotFound);
    }

    Ok(())
}
