use cairo_felt::Felt252;
use cairo_lang_runner::{Arg, SierraCasmRunner, StarknetState};
use cairo_lang_sierra::program::Program;
use cairo_lang_starknet::contract::ContractInfo;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;

use crate::errors::SierraRunnerError;

pub trait SierraRunner<T> {
    fn run_with_contracts_info(
        &self,
        name: &str,
        arguments: &[Arg],
        contracts_info: OrderedHashMap<Felt252, ContractInfo>,
    ) -> Result<Vec<Felt252>, SierraRunnerError>;
    fn run(&self, name: &str, arguments: &[Arg]) -> Result<Vec<Felt252>, SierraRunnerError>;
}

pub struct Runner {
    progarm: Program,
}

impl Runner {
    pub fn new(progarm: Program) -> Self {
        Self { progarm }
    }
}

impl SierraRunner<Vec<Felt252>> for Runner {
    fn run_with_contracts_info(
        &self,
        name: &str,
        arguments: &[Arg],
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
                println!("{e:?}");
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
    fn run(&self, name: &str, arguments: &[Arg]) -> Result<Vec<Felt252>, SierraRunnerError> {
        self.run_with_contracts_info(name, arguments, OrderedHashMap::default())
    }
}
