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
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to Scarb workspace
    #[arg(long, short)]
    target: String,

    /// Package name
    #[arg(long, short)]
    package: Option<String>,

    /// Function name
    #[arg(long, short)]
    function: Option<String>,

    /// Serialized arguments
    #[arg(long, short)]
    args: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let target = args.target;
    let package = args.package.unwrap_or_else(|| {
        Path::new(&target)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    });
    let function = args.function.unwrap_or_else(|| "main".to_string());
    let args: WrappedArgs = serde_json::from_str(&args.args).unwrap();

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
