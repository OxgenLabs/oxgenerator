use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;

use crate::core::database::DatabaseEngine;
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

pub fn which_db_engine() -> OxgenResult<DatabaseEngine> {
    ensure_oxgen_project_root()?;

    let config_file =
        std::fs::File::open(".oxgen/config.toml").map_err(|_| OxgenError::OxgenProjectNotFound)?;

    let reader = std::io::BufReader::new(config_file);

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        if key.trim() != "database" {
            continue;
        }

        let value = value.trim().trim_matches('"');

        return DatabaseEngine::from_str(value);
    }

    Ok(DatabaseEngine::Mock)
}
