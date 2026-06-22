use std::fmt;
use std::str::FromStr;

use crate::core::error::OxgenError;
use crate::core::result::OxgenResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseEngine {
    Mock,
    MongoDb,
}

impl DatabaseEngine {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Mock => "mock",
            Self::MongoDb => "mongodb",
        }
    }

    pub fn supports_collection(self) -> bool {
        matches!(self, Self::MongoDb)
    }
}

impl FromStr for DatabaseEngine {
    type Err = OxgenError;

    fn from_str(value: &str) -> OxgenResult<Self> {
        match value {
            "mock" => Ok(Self::Mock),
            "mongodb" => Ok(Self::MongoDb),
            _ => Err(OxgenError::UnknownDatabase),
        }
    }
}

impl fmt::Display for DatabaseEngine {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
