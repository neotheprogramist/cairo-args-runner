use anyhow::Result;
use cairo_args_runner::utils::compile::ScarbProjectCompiler;
use cairo_args_runner::utils::generate::{Generator, ScarbProjectGenerator};
use cairo_args_runner::utils::logger::{LoggerCompiler, LoggerGenerator, LoggerParser};
use cairo_args_runner::utils::parse::SierraParser;
use cairo_args_runner::utils::run::SierraRunner;
use cairo_felt::Felt252;
use cairo_lang_runner::Arg as CairoRunnerArg;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the folder of the Scarb project
    #[arg(long)]
    folder: String,

    /// Package name of the Scarb project
    #[arg(long)]
    package: String,

    /// Function to run
    #[arg(long)]
    function: String,

    /// Numeric arguments to pass to the function
    #[arg(long)]
    values: Vec<u128>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let generator = LoggerGenerator::new(Generator::new(args.folder, args.package));
    let compiler = LoggerCompiler::new(generator.generate()?);
    let parser = LoggerParser::new(compiler.compile()?);
    let runner = parser.parse()?;

    let arguments = args
        .values
        .into_iter()
        .map(|num| CairoRunnerArg::Value(Felt252::new(num)))
        .collect::<Vec<_>>();

    let result = runner
        .run(format!("::{}", args.function).as_str(), &arguments)
        .unwrap();
    println!("Result: {:?}", result);
    Ok(())
}
