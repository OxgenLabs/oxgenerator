use crate::core::error::OxgeneratorError;

pub type OxgeneratorResult<T, E = OxgeneratorError> = Result<T, E>;
