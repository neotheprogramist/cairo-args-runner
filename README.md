# Cairo Args Runner

**Cairo Args Runner** is a utility designed to execute Cairo 1 programs with arguments directly from the command line. This tool simplifies the process of running Cairo programs by allowing you to specify arguments in a JSON file.

## Usage

To use **Cairo Args Runner**, you need to create a JSON file that contains the configuration and arguments for the Cairo function you want to run.

For example, let's say you have a JSON file named `input.json` with your configuration and arguments. You can run your Cairo function with these arguments using the following command:

```bash
cargo run --release -- -f input.json
