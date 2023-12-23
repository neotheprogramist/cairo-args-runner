use std::fs;

use anyhow::{Context, Result};
use cairo_lang_sierra::ProgramParser;

use crate::utils::run::Runner;

pub trait SierraParser<T> {
    fn parse(self) -> Result<T>;
}

pub struct SingleFileParser {
    file_name: String,
}

impl SingleFileParser {
    pub fn new(file_name: String) -> Self {
        SingleFileParser { file_name }
    }
}

impl SierraParser<Runner> for SingleFileParser {
    fn parse(self) -> Result<Runner> {
        let sierra_code =
            fs::read_to_string(self.file_name).with_context(|| "Could not read file!")?;
        let Ok(sierra_program) = ProgramParser::new().parse(&sierra_code) else {
            anyhow::bail!("Failed to parse sierra program.")
        };
        Ok(Runner::new(sierra_program))
    }
}
