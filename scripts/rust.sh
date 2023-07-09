#!/bin/bash

# Save the current directory
original_dir=$(pwd)
git_root_dir=$(git rev-parse --show-toplevel)
cd "$git_root_dir"


# write petra schema
cat showcase/schema.petra | cargo run -p petra -- -b rs > showcase/red_rust/src/model.rs


cd showcase/red_rust


cargo build
cargo test
cargo fmt -- --check
cargo clippy --tests --all-targets --all-features -- -D warnings

echo "All passed" 

cd "$original_dir"