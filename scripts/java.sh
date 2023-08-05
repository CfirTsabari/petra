#!/bin/bash
set -e

# Save the current directory
original_dir=$(pwd)
git_root_dir=$(git rev-parse --show-toplevel)
cd "$git_root_dir"


# write petra schema
cat showcase/schema.petra | cargo run -p petra -- -b java > showcase/red_java/lib/src/main/java/red_java/Defs.java


cd showcase/red_java


./gradlew build 
./gradlew spotlessCheck
./gradlew test
./gradlew check 

echo "All passed" 

cd "$original_dir"