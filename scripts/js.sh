#!/bin/bash
set -e

# Save the current directory
original_dir=$(pwd)
git_root_dir=$(git rev-parse --show-toplevel)
cd "$git_root_dir"


# write petra schema
cat showcase/schema.petra | cargo run -p petra -- -b js > showcase/red_js/src/defs.js


cd showcase/red_js


pnpm test
pnpm check_fmt 
pnpm eslint

echo "All passed" 

cd "$original_dir"