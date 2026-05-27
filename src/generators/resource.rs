use crate::core::generator::Generator;
use crate::core::result::OxgenResult;

pub struct ResourceGenerator {
    name: String,
    force: bool,
    dry_run: bool,
}

impl ResourceGenerator {
    pub fn new(name: String, force: bool, dry_run: bool) -> Self {
        Self {
            name,
            force,
            dry_run,
        }
    }
}

impl Generator for ResourceGenerator {
    fn generate(&self) -> OxgenResult<()> {
        Ok(())
    }
}
