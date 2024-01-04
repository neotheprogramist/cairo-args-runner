use std::fs;

use cairo_lang_sierra::program::VersionedProgram;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

use crate::utils::run::Runner;

#[derive(Error, Debug)]
pub enum SierraParseError {
    #[error("Could not read file")]
    FileRead(#[from] std::io::Error),

    #[error("Failed to parse sierra program")]
    SierraProgramParse(String),

    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] SerdeJsonError),
}

pub trait SierraParser<T, E> {
    fn parse(self) -> Result<T, E>;
}

pub struct SingleFileParser {
    file_name: String,
}

impl SingleFileParser {
    pub fn new(file_name: &str) -> Self {
        SingleFileParser {
            file_name: file_name.into(),
        }
    }
}

impl SierraParser<Runner, SierraParseError> for SingleFileParser {
    fn parse(self) -> Result<Runner, SierraParseError> {
        let file_name = self.file_name;
        let sierra_program =
            serde_json::from_str::<VersionedProgram>(&fs::read_to_string(file_name)?)?
                .into_v1()
                .map_err(|err| SierraParseError::SierraProgramParse(err.to_string()))?;

        Ok(Runner::new(sierra_program))
    }
}
