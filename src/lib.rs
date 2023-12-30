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
//! use cairo_args_runner::{arg_array, felt_vec, run};
//!
//! let target = "examples/complex/target/dev/complex.sierra.json";
//! let function = "main";
//! let args = arg_array![5, 1, 2, 4, 8, 16, 6, 1, 2, 3, 4, 5, 6];
//!
//! let result = run(target, function, &[args]);
//! assert_eq!(result.unwrap(), felt_vec![31, 21, 5, 6]);
//! ```
//! **Note:** There is a known bug in this example related to passing arrays as arguments.
//! For more details and updates on this issue, please visit
//! [Issue #7 on GitHub](https://github.com/neotheprogramist/cairo-args-runner/issues/7).
//!
//! ### Fibonacci Sequence
//! Calculate the 10th number in the Fibonacci sequence:
//! ```rust
//! use cairo_args_runner::{arg_array, felt_vec, run};
//!
//! let target = "examples/fib/target/dev/fib.sierra.json";
//! let function = "main";
//! let args = arg_array![10];
//!
//! let result = run(target, function, &[args]);
//! assert_eq!(result.unwrap(), felt_vec![55]);
//! ```
//!
//! ### Working with Structs
//! Execute a function that works with multiple struct arguments:
//! ```rust
//! use cairo_args_runner::{arg_array, felt_vec, run};
//!
//! let target = "examples/structs/target/dev/structs.sierra.json";
//! let function = "main";
//! let args = arg_array![1, 2, 10, 5, 9, 3, 1, 2, 3];
//!
//! let result = run(target, function, &[args]);
//! assert_eq!(result.unwrap(), felt_vec![33]);
//! ```
//!
//! ### Summation Example
//! Run a function to sum an array of numbers:
//! ```rust
//! use cairo_args_runner::{arg_array, felt_vec, run};
//!
//! let target = "examples/sum/target/dev/sum.sierra.json";
//! let function = "main";
//! let args = arg_array![4, 1, 3, 9, 27];
//!
//! let result = run(target, function, &[args]);
//! assert_eq!(result.unwrap(), felt_vec![40]);
//! ```
//!
//! These examples demonstrate various ways to use `cairo_args_runner` to execute Cairo 1 programs with different types of arguments,
//! aiding users in understanding and utilizing the utility effectively.

pub use cairo_felt::Felt252;
pub use cairo_lang_runner::Arg;
use errors::SierraRunnerError;
use utils::parse::SingleFileParser;

pub use crate::utils::args::ArgsArray;
use crate::utils::{parse::SierraParser, run::SierraRunner};

pub mod errors;
mod macros;
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
pub fn run(
    file_name: &str,
    function: &str,
    args: &[Arg],
) -> Result<Vec<Felt252>, SierraRunnerError> {
    let parser = SingleFileParser::new(file_name);
    let runner = parser.parse()?;

    runner.run(format!("::{function}").as_str(), args)
}
