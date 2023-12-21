use std::fmt;

use cairo_felt::Felt252;
use cairo_lang_runner::{Arg, SierraCasmRunner, StarknetState};
use cairo_lang_sierra::program::Program;
use cairo_lang_starknet::contract::ContractInfo;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;

pub enum SierraRunnerError {
    FailedSettingUp(String),
    FailedFindingFunction,
    FailedRunning,
    Panicked(Vec<Felt252>),
}

impl fmt::Display for SierraRunnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SierraRunnerError::FailedSettingUp(e) => write!(f, "Failed setting up: {}", e),
            SierraRunnerError::FailedFindingFunction => write!(f, "Failed finding function"),
            SierraRunnerError::FailedRunning => write!(f, "Failed running"),
            SierraRunnerError::Panicked(_) => write!(f, "Panicked"),
        }
    }
}

impl fmt::Debug for SierraRunnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl std::error::Error for SierraRunnerError {}

pub trait SierraRunner<T> {
    fn run_with_contracts_info(
        &self,
        name: &str,
        arguments: &Vec<Arg>,
        contracts_info: OrderedHashMap<Felt252, ContractInfo>,
    ) -> Result<Vec<Felt252>, SierraRunnerError>;
    fn run(&self, name: &str, arguments: &Vec<Arg>) -> Result<Vec<Felt252>, SierraRunnerError>;
}

pub struct Runner {
    progarm: Program,
}

impl Runner {
    pub fn new(progarm: Program) -> Self {
        Self { progarm }
    }
}

#[macro_export]
macro_rules! arg_vec {
    ($($a:expr),*) => {
        vec![$(Felt252::from($a)),*]
    };
}

#[macro_export]
macro_rules! arg_val {
    ($a:expr) => {
        Arg::Value(Felt252::from($a))
    };
}

pub use arg_val;

impl SierraRunner<Vec<Felt252>> for Runner {
    fn run_with_contracts_info(
        &self,
        name: &str,
        arguments: &Vec<Arg>,
        contracts_info: OrderedHashMap<Felt252, ContractInfo>,
    ) -> Result<Vec<Felt252>, SierraRunnerError> {
        let runner = match SierraCasmRunner::new(
            self.progarm.clone(),
            Some(Default::default()),
            contracts_info,
        ) {
            Ok(runner) => runner,
            Err(e) => {
                return Err(SierraRunnerError::FailedSettingUp(e.to_string()));
            }
        };
        let Ok(function) = runner.find_function(name) else {
            return Err(SierraRunnerError::FailedFindingFunction);
        };
        let result = match runner.run_function_with_starknet_context(
            function,
            arguments,
            Some(usize::MAX),
            StarknetState::default(),
        ) {
            Ok(r) => r,
            Err(e) => {
                println!("{:?}", e);
                return Err(SierraRunnerError::FailedRunning);
            }
        };
        match result.value {
            cairo_lang_runner::RunResultValue::Success(values) => Ok(values),
            cairo_lang_runner::RunResultValue::Panic(values) => {
                Err(SierraRunnerError::Panicked(values))
            }
        }
    }
    fn run(&self, name: &str, arguments: &Vec<Arg>) -> Result<Vec<Felt252>, SierraRunnerError> {
        self.run_with_contracts_info(name, arguments, OrderedHashMap::default())
    }
}
