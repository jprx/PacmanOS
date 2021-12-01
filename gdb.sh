#!/bin/bash
# Wow, I am surprised I remember this off the top of my head
# This only works under a docker container on Docker Desktop (not Linux)
gdb-multiarch build/PacmanOS.elf -ex "target remote host.docker.internal:1234"
