use anyhow::Result;
use cairo_args_runner::utils::compile::ScarbProjectCompiler;
use cairo_args_runner::utils::generate::{Generator, ScarbProjectGenerator};
use cairo_args_runner::utils::logger::{LoggerCompiler, LoggerGenerator, LoggerParser};
use cairo_args_runner::utils::parse::SierraParser;
use cairo_args_runner::utils::run::SierraRunner;
use cairo_felt::Felt252;
use cairo_lang_runner::Arg;

fn main() -> Result<()> {
    let generator = LoggerGenerator::new(Generator::new("examples/fib", "fib"));
    let compiler = LoggerCompiler::new(generator.generate()?);
    let parser = LoggerParser::new(compiler.compile()?);
    let runner = parser.parse()?;
    let arguments = vec![Arg::Value(Felt252::new(10))];
    let result = runner.run("::main", &arguments).unwrap();
    println!("Result: {:?}", result);
    Ok(())
}
