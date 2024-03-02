#!/bin/bash
set -e

# Save the current directory
original_dir=$(pwd)
git_root_dir=$(git rev-parse --show-toplevel)
cd "$git_root_dir"


# write petra schema
cat showcase/schema.petra | cargo run -p petra -- -b ts > showcase/red_ts/src/defs.ts


cd showcase/red_ts


pnpm build 
pnpm test
pnpm check_fmt 
pnpm eslint

echo "All passed" 

cd "$original_dir"