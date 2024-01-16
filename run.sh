#!/usr/bin/env bash

cargo clean && \
scarb clean && \
cargo build --release && \
scarb build

echo "[1, 2, 3]" | cargo run --release -- target/dev/e1_struct_with_values.sierra.json
echo "[[1, 9, 1], 7, []]" | cargo run --release -- target/dev/e2_arrays.sierra.json
echo "[[1, 2, 3]]" | cargo run --release -- target/dev/e3_struct_with_array.sierra.json
echo "[[1, 2, 3], [1, 3, 9]]" | cargo run --release -- target/dev/e4_struct_with_arrays.sierra.json
