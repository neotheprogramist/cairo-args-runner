# Cairo Args Runner

**Cairo Args Runner** is a utility designed to execute Cairo 1 programs with arguments directly from the command line. This tool simplifies the process of running Cairo programs by allowing you to specify arguments directly in the command line.

## Usage

To use **Cairo Args Runner**, you need to specify the target directory and the arguments for the Cairo function you want to run.

For example, to run a `main` function in the `complex` package located in the `examples/complex` folder, and pass `[10, 2, 20, [1, 2, 4, 8, 16]]` as an argument, you would use the following command:

```bash
cargo run --release -- -t examples/complex -a "[10, 2, 20, [1, 2, 4, 8, 16]]"
