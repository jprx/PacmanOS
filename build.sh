#!/bin/bash
# Builds the rust component of the project
RUST_TARGET_PATH=$(pwd) cargo build --target=applem1-pacmanos-none.json -Z build-std=core,alloc
docker-compose run devel make clean
docker-compose run devel make
