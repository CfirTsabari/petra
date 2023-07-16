#!/bin/bash
set -e

# Save the current directory
original_dir=$(pwd)
git_root_dir=$(git rev-parse --show-toplevel)
cd "$git_root_dir"


# write petra schema
cat showcase/schema.petra | cargo run -p petra -- -b py > showcase/red_python/red_python/model.py


cd showcase/red_python


poetry run black --check red_python tests
poetry run pytest tests
poetry run ruff tests red_python
poetry run mypy red_python tests

echo "All passed" 

cd "$original_dir"