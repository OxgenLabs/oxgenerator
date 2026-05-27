use crate::core::generator::Generator;
use crate::core::result::OxgenResult;

pub struct ControllerGenerator {
    name: String,
    force: bool,
    dry_run: bool,
}

impl ControllerGenerator {
    pub fn new(name: String, force: bool, dry_run: bool) -> Self {
        Self {
            name,
            force,
            dry_run,
        }
    }
}

impl Generator for ControllerGenerator {
    fn generate(&self) -> OxgenResult<()> {
        Ok(())
    }
}
