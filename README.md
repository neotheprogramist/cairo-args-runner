# Cairo Args Runner

## Overview

**Cairo Args Runner** is a streamlined utility for executing Cairo 1 programs with command-line arguments.
It enhances the user experience by enabling direct argument specification in the command line,
thus simplifying the execution of Cairo programs.

## Configuration

Before using **Cairo Args Runner**, ensure your `Scarb.toml` configuration file is set up as follows:

```toml
[lib]
sierra-text = true
```

This step is crucial for the proper functioning of the utility.

## How to Use

### Basic Usage

To use **Cairo Args Runner**, you need to:

1. Navigate to the target directory of your Cairo program.
2. Build the program using `scarb build`.
3. Run the program with the desired arguments.

## Crate Installation via Cargo

You can also install this crate directly using the `cargo install` command.
To do this, run the following command in your terminal:

```bash
cargo install cairo-args-runner
```

### Example

Run a `main` function from the `complex` package located in `examples/complex`,
passing `[[1, 2, 4, 8, 16], [1, 2, 3, 4, 5, 6]]` as the argument:

```bash
cd examples/complex/
scarb build
echo "[[3, 3], [5, 5]]" | cargo run --release -- examples/complex/target/dev/complex.sierra
```

**Note:** There is a known bug in this example related to passing arrays as arguments.
For more details and updates on this issue, please visit
[Issue #7 on GitHub](https://github.com/neotheprogramist/cairo-args-runner/issues/7).

## Additional Examples

Here are more examples for running different Cairo programs:

### Fibonacci Sequence Example

```bash
cd examples/fib/
scarb build
echo "[10]" | cargo run --release -- examples/fib/target/dev/fib.sierra
```

### Structs Example

```bash
cd examples/structs/
scarb build
echo "[1, 2, 3, 4, 5]" | cargo run --release -- examples/structs/target/dev/structs.sierra
```

### Summation Example

```bash
cd examples/sum/
scarb build
echo "[[1, 3, 9, 27]]" | cargo run --release -- examples/sum/target/dev/sum.sierra
```

These examples demonstrate the versatility and ease of using **Cairo Args Runner** for different types of Cairo 1 programs.
