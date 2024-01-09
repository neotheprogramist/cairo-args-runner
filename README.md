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

## Examples

Here are examples for running different Cairo programs:

### e1_struct_with_values

```bash
cd examples/e1_struct_with_values/
scarb build
echo "[1, 2, 3]" | cargo run --release -- examples/e1_struct_with_values/target/dev/e1_struct_with_values.sierra.json
```

The output should be `[1, 2, 3]` and it is OK.

### e2_arrays

```bash
cd examples/e2_arrays/
scarb build
echo "[[1, 9, 1], 7, []]" | cargo run --release -- examples/e2_arrays/target/dev/e2_arrays.sierra.json
```

The output should be `[11, 7, 0]` and it is OK.

### e3_struct_with_array

```bash
cd examples/e3_struct_with_array/
scarb build
echo "[[1, 2, 3]]" | cargo run --release -- examples/e3_struct_with_array/target/dev/e3_struct_with_array.sierra.json
```

The output should be `[6]` and it is OK.

### e4_struct_with_arrays

```bash
cd examples/e4_struct_with_arrays/
scarb build
echo "[[1, 2, 3], [1, 3, 9]]" | cargo run --release -- examples/e4_struct_with_arrays/target/dev/e4_struct_with_arrays.sierra.json
```

The output should be `[6, 13]` but the error is:

```
ArgumentsSizeMismatch { expected: 1, actual: 2 }
Error: RunError(FailedRunning)
```
