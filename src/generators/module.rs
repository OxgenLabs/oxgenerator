use crate::core::generator::Generator;
use crate::core::result::OxgenResult;

pub struct ModuleGenerator {
    name: String,
    force: bool,
    dry_run: bool,
}

impl ModuleGenerator {
    pub fn new(name: String, force: bool, dry_run: bool) -> Self {
        Self {
            name,
            force,
            dry_run,
        }
    }
}

impl Generator for ModuleGenerator {
    fn generate(&self) -> OxgenResult<()> {
        Ok(())
    }
}
