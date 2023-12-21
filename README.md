# Cairo Args Runner

Cairo Args Runner is a utility that enables you to execute Cairo 1 programs with arguments from the command line.

## Usage

To use Cairo Args Runner, you'll need to specify the folder, package, and function you wish to run, along with any values you want to pass as arguments.

For example, to run a function named `main` in the `fib` package located in the `examples/fib` folder, and pass `5` as an argument, you would use the following command:

```bash
cargo run --release -- --folder examples/fib --package fib --function main --values 5
