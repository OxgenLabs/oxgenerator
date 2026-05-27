use crate::core::generator::Generator;
use crate::core::result::OxgenResult;

pub struct ServiceGenerator {
    name: String,
    force: bool,
    dry_run: bool,
}

impl ServiceGenerator {
    pub fn new(name: String, force: bool, dry_run: bool) -> Self {
        Self {
            name,
            force,
            dry_run,
        }
    }
}

impl Generator for ServiceGenerator {
    fn generate(&self) -> OxgenResult<()> {
        Ok(())
    }
}
