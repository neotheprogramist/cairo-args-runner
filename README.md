# Cairo Args Runner

**Cairo Args Runner** is a utility designed to execute Cairo 1 programs with arguments directly from the command line. This tool simplifies the process of running Cairo programs by allowing you to specify arguments directly in the command line.

## Configuration

Make sure your `Scarb.toml` file includes the following section:

```toml
[lib]
sierra-text = true
```

## Usage

To use **Cairo Args Runner**, you need to specify the target directory and the arguments for the Cairo function you want to run.

For example, to run a `main` function in the `complex` package located in the `examples/complex` folder, and pass `[[1, 2, 4, 8, 16], [1, 2, 3, 4, 5, 6]]` as an argument, you would use the following command:

```bash
cd examples/complex/
scarb build
echo "[[1, 2, 4, 8, 16], [1, 2, 3, 4, 5, 6]]" | cargo run --release -- examples/complex/target/dev/complex.sierra
```

## More examples

```bash
cd examples/fib/
scarb build
echo "[10]" | cargo run --release -- examples/fib/target/dev/fib.sierra
```

```bash
cd examples/structs/
scarb build
echo "[1, 2, 3, 4, 5]" | cargo run --release -- examples/structs/target/dev/structs.sierra
```

```bash
cd examples/sum/
scarb build
echo "[[1, 3, 9, 27]]" | cargo run --release -- examples/sum/target/dev/sum.sierra
```
