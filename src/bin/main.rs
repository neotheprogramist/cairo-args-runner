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
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input json file
    #[arg(long, short)]
    file: String,
}

#[derive(Deserialize)]
struct Config {
    folder: String,
    package: String,
    function: String,
    args: WrappedArgs,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file = File::open(args.file)?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;

    println!("args: {:?}", config.args);

    let generator = LoggerGenerator::new(Generator::new(config.folder, config.package));
    let compiler = LoggerCompiler::new(generator.generate()?);
    let parser = LoggerParser::new(compiler.compile()?);
    let runner = parser.parse()?;

    let result = runner
        .run(format!("::{}", config.function).as_str(), &config.args)
        .unwrap();
    println!("Result: {:?}", result);
    Ok(())
}
