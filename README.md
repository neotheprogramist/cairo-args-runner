# Cairo Args Runner

## Overview

**Cairo Args Runner** is a streamlined utility for executing Cairo 1 programs with command-line arguments.
It enhances the user experience by enabling direct argument specification in the command line,
thus simplifying the execution of Cairo programs.

## Examples

You can run all examples with:

```bash
./run.sh
```

Here are examples for running different Cairo programs:

Firstly build examples with:

```bash
scarb build
```

And then build the runner with:

```bash
cargo build --release
```

### e1_struct_with_values

```bash
echo "[1, 2, 3]" | cargo run --release -- target/dev/e1_struct_with_values.sierra.json
```

The output should be `[1, 2, 3]` and it is OK.

### e2_arrays

```bash
echo "[[1, 9, 1], 7, []]" | cargo run --release -- target/dev/e2_arrays.sierra.json
```

The output should be `[11, 7, 0]` and it is OK.

### e3_struct_with_array

```bash
echo "[[1, 2, 3]]" | cargo run --release -- target/dev/e3_struct_with_array.sierra.json
```

The output should be `[6]` and it is OK.

### e4_struct_with_arrays

```bash
echo "[[1, 2, 3], [1, 3, 9]]" | cargo run --release -- target/dev/e4_struct_with_arrays.sierra.json
```

The output should be `[6, 13]` but the error is:

```
ArgumentsSizeMismatch { expected: 1, actual: 2 }
Error: RunError(FailedRunning)
```
