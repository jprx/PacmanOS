#!/bin/bash
# Builds the rust component of the project
cargo build -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem --target=applem1-pacmanos-none.json
docker-compose run devel make clean
docker-compose run devel make
