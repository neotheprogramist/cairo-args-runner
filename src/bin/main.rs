use anyhow::Result;
use cairo_args_runner::utils::{
    args::WrappedArgs,
    compile::ScarbProjectCompiler,
    generate::{Generator, ScarbProjectGenerator},
    logger::{LoggerCompiler, LoggerGenerator, LoggerParser},
    parse::SierraParser,
    run::SierraRunner,
};
use clap::Parser;
use std::{
    io::{self, Read},
    path::Path,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to Scarb workspace
    target: String,

    /// Package name
    #[arg(long, short)]
    package: Option<String>,

    /// Function name
    #[arg(long, short)]
    function: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut program_input = String::new();
    io::stdin().read_to_string(&mut program_input)?;

    let target = cli.target;
    let package = cli.package.unwrap_or_else(|| {
        Path::new(&target)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    });
    let function = cli.function.unwrap_or_else(|| "main".to_string());
    let args: WrappedArgs = serde_json::from_str(&program_input).unwrap();

    let generator = LoggerGenerator::new(Generator::new(target, package));
    let compiler = LoggerCompiler::new(generator.generate()?);
    let parser = LoggerParser::new(compiler.compile()?);
    let runner = parser.parse()?;

    let result = runner
        .run(format!("::{}", function).as_str(), &args)
        .unwrap();
    println!("Result: {:?}", result);
    Ok(())
}
