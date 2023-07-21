#!/bin/bash
set -e

# Save the current directory
original_dir=$(pwd)
git_root_dir=$(git rev-parse --show-toplevel)
cd "$git_root_dir"


# write petra schema
cat showcase/schema.petra | cargo run -p petra -- -b go > showcase/red_golang/pkg/defs/defs.go

cd showcase/red_golang

go build ./...
go test ./...

if [ -n "$(goimports -d .)" ]; then
    exit 1
fi

staticcheck ./...
/workspace/go/bin/golangci-lint run

echo "All passed" 

cd "$original_dir"