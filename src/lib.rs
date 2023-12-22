use anyhow::Result;
use cairo_felt::Felt252;
use cairo_lang_runner::Arg;

use crate::utils::{
    compile::ScarbProjectCompiler,
    generate::{Generator, ScarbProjectGenerator},
    logger::{LoggerCompiler, LoggerGenerator, LoggerParser},
    parse::SierraParser,
    run::SierraRunner,
};

pub mod utils;

pub fn run(target: &str, package: &str, function: &str, args: &[Arg]) -> Result<Vec<Felt252>> {
    let generator = LoggerGenerator::new(Generator::new(target, package));
    let compiler = LoggerCompiler::new(generator.generate()?);
    let parser = LoggerParser::new(compiler.compile()?);
    let runner = parser.parse()?;

    Ok(runner.run(format!("::{}", function).as_str(), args)?)
}
