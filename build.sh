#!/bin/bash
# Builds the rust component of the project
cargo build -Zbuild-std=core -Zbuild-std-features=compiler-builtins-mem --target=aarch64-unknown-linux-gnu
docker-compose run devel make clean
docker-compose run devel make
