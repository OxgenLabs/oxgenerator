use crate::core::database::DatabaseEngine;

#[derive(Debug, Clone, Copy)]
pub struct GeneratorContext {
    pub force: bool,
    pub dry_run: bool,
    pub database: DatabaseEngine,
}

impl GeneratorContext {
    pub fn new(force: bool, dry_run: bool, database: DatabaseEngine) -> Self {
        Self {
            force,
            dry_run,
            database,
        }
    }
}
