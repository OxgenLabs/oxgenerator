use crate::core::generator::Generator;
use crate::core::result::OxgenResult;

pub struct ModelGenerator {
    name: String,
    force: bool,
    dry_run: bool,
}

impl ModelGenerator {
    pub fn new(name: String, force: bool, dry_run: bool) -> Self {
        Self {
            name,
            force,
            dry_run,
        }
    }
}

impl Generator for ModelGenerator {
    fn generate(&self) -> OxgenResult<()> {
        Ok(())
    }
}
