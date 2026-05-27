use crate::core::result::OxgenResult;

pub trait Generator {
    fn generate(&self) -> OxgenResult<()>;
}
