#!/bin/bash
set -e

# Save the current directory
original_dir=$(pwd)
git_root_dir=$(git rev-parse --show-toplevel)
cd "$git_root_dir"


# write petra schema
cat showcase/schema.petra | cargo run -p petra -- -b cpp > showcase/red_cpp/include/defs.h


cd showcase/red_cpp


cmake -S . -B build
cmake --build build
build/test/test_main
cmake --build build --target main-clangformat-check

echo "All passed" 

cd "$original_dir"