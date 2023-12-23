//! # Cairo Args Runner
//!
//! `cairo_args_runner` is a utility designed to execute Cairo 1 programs with arguments directly from the command line.
//! This tool simplifies the process of running Cairo programs by allowing you to specify arguments directly in the command line.
//!
//! ## Configuration
//!
//! Make sure your `Scarb.toml` file includes the following section:
//!
//! ```toml
//! [lib]
//! sierra-text = true
//! ```
//!
//! ## Examples
//!
//! ### Running a Complex Function
//! Run a complex function with an array of arguments:
//! ```rust
//! use cairo_args_runner::{run, Arg, Felt252};
//!
//! let target = "examples/complex/target/dev/complex.sierra";
//! let function = "main";
//! let args = vec![
//!     Arg::Array(vec![
//!         Felt252::new(1),
//!         Felt252::new(2),
//!         Felt252::new(4),
//!         Felt252::new(8),
//!         Felt252::new(16),
//!     ]),
//!     Arg::Array(vec![
//!         Felt252::new(1),
//!         Felt252::new(2),
//!         Felt252::new(3),
//!         Felt252::new(4),
//!         Felt252::new(5),
//!         Felt252::new(6),
//!     ]),
//! ];
//!
//! let result = run(target, function, &args);
//! assert_eq!(
//!     result.unwrap(),
//!     vec![
//!         Felt252::new(15),
//!         Felt252::new(31),
//!         Felt252::new(5),
//!         Felt252::new(6)
//!     ]
//! );
//! ```
//! **Note:** There is a known bug in this example related to passing arrays as arguments.
//! For more details and updates on this issue, please visit
//! [Issue #7 on GitHub](https://github.com/neotheprogramist/cairo-args-runner/issues/7).
//!
//! ### Fibonacci Sequence
//! Calculate the 10th number in the Fibonacci sequence:
//! ```rust
//! use cairo_args_runner::{run, Arg, Felt252};
//!
//! let target = "examples/fib/target/dev/fib.sierra";
//! let function = "main";
//! let args = vec![Arg::Value(Felt252::new(10))];
//!
//! let result = run(target, function, &args);
//! assert_eq!(result.unwrap(), vec![Felt252::new(55)]);
//! ```
//!
//! ### Working with Structs
//! Execute a function that works with multiple struct arguments:
//! ```rust
//! use cairo_args_runner::{run, Arg, Felt252};
//!
//! let target = "examples/structs/target/dev/structs.sierra";
//! let function = "main";
//! let args = vec![
//!     Arg::Value(Felt252::new(1)),
//!     Arg::Value(Felt252::new(2)),
//!     Arg::Value(Felt252::new(3)),
//!     Arg::Value(Felt252::new(4)),
//!     Arg::Value(Felt252::new(5)),
//! ];
//!
//! let result = run(target, function, &args);
//! assert_eq!(result.unwrap(), vec![Felt252::new(15)]);
//! ```
//!
//! ### Summation Example
//! Run a function to sum an array of numbers:
//! ```rust
//! use cairo_args_runner::{run, Arg, Felt252};
//!
//! let target = "examples/sum/target/dev/sum.sierra";
//! let function = "main";
//! let args = vec![Arg::Array(vec![
//!     Felt252::new(1),
//!     Felt252::new(3),
//!     Felt252::new(9),
//!     Felt252::new(27),
//! ])];
//!
//! let result = run(target, function, &args);
//! assert_eq!(result.unwrap(), vec![Felt252::new(40)]);
//! ```
//!
//! These examples demonstrate various ways to use `cairo_args_runner` to execute Cairo 1 programs with different types of arguments,
//! aiding users in understanding and utilizing the utility effectively.

use anyhow::Result;
pub use cairo_felt::Felt252;
pub use cairo_lang_runner::Arg;
use utils::parse::SingleFileParser;

pub use crate::utils::args::WrappedArg;
use crate::utils::{parse::SierraParser, run::SierraRunner};

mod utils;

/// Runs the specified function with the provided arguments.
///
/// # Arguments
///
/// * `file_name` - A string slice that holds the file name.
/// * `function` - A string slice that holds the function to run.
/// * `args` - A slice of `Arg` that holds the arguments to the function.
///
/// # Returns
///
/// * `Result<Vec<Felt252>>` - A Result containing a vector of `Felt252` if the function runs successfully, or an error if it fails.
///
/// # Errors
///
/// This function will return an error if:
///
/// * The file specified by `file_name` cannot be found or read.
/// * The function specified by `function` cannot be found in the file.
/// * The arguments provided in `args` are not valid for the function.
/// * The function execution fails for any reason.
pub fn run(file_name: &str, function: &str, args: &[Arg]) -> Result<Vec<Felt252>> {
    let parser = SingleFileParser::new(file_name);
    let runner = parser.parse()?;

    Ok(runner.run(format!("::{function}").as_str(), args)?)
}
