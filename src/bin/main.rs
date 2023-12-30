use std::io::{self, Read};

use cairo_lang_runner::Arg;
use clap::Parser;
use thiserror::Error;

use cairo_args_runner::{errors::SierraRunnerError, run, ArgsArray};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("failed to read string from stdin")]
    ReadError(#[from] std::io::Error),

    #[error("run function failed")]
    RunError(#[from] SierraRunnerError),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to compiled sierra file
    target: String,

    /// Function name
    #[arg(long, short)]
    function: Option<String>,
}

fn main() -> Result<(), AppError> {
    let cli = Cli::parse();
    let mut program_input = String::new();
    io::stdin().read_to_string(&mut program_input)?;

    let target = cli.target;
    let function = cli.function.unwrap_or_else(|| "main".to_string());
    let args: ArgsArray = serde_json::from_str(&program_input).unwrap();

    let result = run(&target, &function, &[Arg::Array(args.to_vec())])?;

    println!("{args:?}");
    println!("{result:?}");
    Ok(())
}
