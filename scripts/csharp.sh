#!/bin/bash
set -e

# Save the current directory
original_dir=$(pwd)
git_root_dir=$(git rev-parse --show-toplevel)
cd "$git_root_dir"


# write petra schema

cat showcase/schema.petra | cargo run -p petra -- -b c# --no-gen-comment > showcase/red_csharp/RedCsharp/Defs.cs

cd showcase/red_csharp


dotnet restore
dotnet build --configuration Release --no-restore
# this doesn't work on gitpod
#dotnet test --no-restore --verbosity normal
dotnet format --verify-no-changes --no-restore 

echo "All passed" 

cd "$original_dir"