use anyhow::Result;
use cairo_args_runner::{run, WrappedArgs};
use clap::Parser;
use std::io::{self, Read};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to compiled sierra file
    target: String,

    /// Function name
    #[arg(long, short)]
    function: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut program_input = String::new();
    io::stdin().read_to_string(&mut program_input)?;

    let target = cli.target;
    let function = cli.function.unwrap_or_else(|| "main".to_string());
    let args: WrappedArgs = serde_json::from_str(&program_input).unwrap();

    let result = run(&target, &function, &args)?;
    println!("Result: {:?}", result);
    Ok(())
}
