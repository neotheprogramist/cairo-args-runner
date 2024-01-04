# Cairo Args Runner

## Overview

**Cairo Args Runner** is a streamlined utility for executing Cairo 1 programs with command-line arguments.
It enhances the user experience by enabling direct argument specification in the command line,
thus simplifying the execution of Cairo programs.

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

**Note:** There is a known bug in this example related to passing arrays as arguments.
For more details and updates on this issue, please visit
[Issue #7 on GitHub](https://github.com/neotheprogramist/cairo-args-runner/issues/7).

## Examples

Here are examples for running different Cairo programs:

### Arrays2

```bash
cd examples/arrays2/
scarb build
echo "[[1, 1, 1], [2, 2, 2]]" | cargo run --release -- examples/arrays2/target/dev/arrays2.sierra.json
```

### Arrays3

```bash
cd examples/arrays3/
scarb build
echo "[[1, 1], [2, 2], [3, 3]]" | cargo run --release -- examples/arrays3/target/dev/arrays3.sierra.json
```

### Structs

```bash
cd examples/structs/
scarb build
echo "[1, 2, 10, 5, 9]" | cargo run --release -- examples/structs/target/dev/structs.sierra.json
```

### Serialization1

```bash
cd examples/serialization1/
scarb build
echo "[1, 2, 10, 5, 0]" | cargo run --release -- examples/serialization1/target/dev/serialization1.sierra.json
```

### Serialization2

```bash
cd examples/serialization2/
scarb build
echo "[1, 1, 1, 1]" | cargo run --release -- examples/serialization2/target/dev/serialization2.sierra.json
```

These examples demonstrate the versatility and ease of using **Cairo Args Runner** for different types of Cairo 1 programs.
