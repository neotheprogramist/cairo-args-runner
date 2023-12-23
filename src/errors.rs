use cairo_felt::Felt252;
use thiserror::Error;

use crate::utils::parse::SierraParseError;

#[derive(Error, Debug)]
pub enum SierraRunnerError {
    #[error("Failed setting up: {0}")]
    FailedSettingUp(String),

    #[error("Failed finding function")]
    FailedFindingFunction,

    #[error("Failed running")]
    FailedRunning,

    #[error("Panicked: {0:?}")]
    Panicked(Vec<Felt252>),

    #[error("Failed to parse arguments: {0}")]
    ParseError(#[from] SierraParseError),
}
