use cairo_felt::Felt252;
use cairo_lang_runner::{Arg, RunResultStarknet, RunResultValue, SierraCasmRunner, StarknetState};
use cairo_lang_sierra::program::ProgramArtifact;
use cairo_lang_starknet::contract::ContractInfo;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;

use crate::errors::SierraRunnerError;

pub struct SuccessfulRun {
    pub value: Vec<Felt252>,
    pub memory: Vec<Option<Felt252>>,
}

pub trait SierraRunner<T> {
    fn run_with_contracts_info(
        &self,
        name: &str,
        arguments: &[Arg],
        contracts_info: OrderedHashMap<Felt252, ContractInfo>,
    ) -> Result<RunResultStarknet, SierraRunnerError>;
    fn run(&self, name: &str, arguments: &[Arg]) -> Result<SuccessfulRun, SierraRunnerError>;
}

pub struct Runner {
    program: ProgramArtifact,
}

impl Runner {
    pub fn new(program: ProgramArtifact) -> Self {
        Self { program }
    }
}

impl SierraRunner<Vec<Felt252>> for Runner {
    fn run_with_contracts_info(
        &self,
        name: &str,
        arguments: &[Arg],
        contracts_info: OrderedHashMap<Felt252, ContractInfo>,
    ) -> Result<RunResultStarknet, SierraRunnerError> {
        let runner = match SierraCasmRunner::new(
            self.program.program.clone(),
            Some(Default::default()),
            contracts_info,
            false,
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
        Ok(result)
    }
    fn run(&self, name: &str, arguments: &[Arg]) -> Result<SuccessfulRun, SierraRunnerError> {
        let result = self.run_with_contracts_info(name, arguments, OrderedHashMap::default())?;
        match result.value {
            RunResultValue::Success(values) => Ok(SuccessfulRun {
                value: values,
                memory: result.memory,
            }),
            RunResultValue::Panic(values) => Err(SierraRunnerError::Panicked(values)),
        }
    }
}
