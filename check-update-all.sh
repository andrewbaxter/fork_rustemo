#!/usr/bin/env sh
set -e -u

export RUST_BACKTRACE=1
export CARGO_INCREMENTAL=1
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# We must first test and install compiler itself
cargo nextest run -p rustemo-compiler
cargo install --path rustemo-compiler --debug

cd docs/src/tutorials/calculator/
for i in {1..5}; do
    rcomp calculator$i/src/calculator.rustemo;
done

cd -

# Default test
cargo nextest run

# Test with array-based table generator
cargo nextest run --features arrays

cargo clippy --all --all-targets -- -D warnings
cargo fmt --all
