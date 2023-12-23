use std::fs;

use cairo_lang_sierra::ProgramParser;
use thiserror::Error;

use crate::utils::run::Runner;

#[derive(Error, Debug)]
pub enum SierraParseError {
    #[error("Could not read file")]
    FileReadError(#[from] std::io::Error),

    #[error("Failed to parse sierra program")]
    SierraProgramParseError(String),
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
        let sierra_code = fs::read_to_string(self.file_name)?;
        let sierra_program = ProgramParser::new()
            .parse(&sierra_code)
            .map_err(|err| SierraParseError::SierraProgramParseError(err.to_string()))?;
        Ok(Runner::new(sierra_program))
    }
}
