#!/bin/bash

# Get name of current kernel
unameVal="$(uname -s)"

# Docker Desktop for Mac uses a different port name than Linux does for GDB
# Just detect host type and pass appropriate port into devel container
if [[ $unameVal == *"Darwin"* ]]; then
    DEBUG_HOST="host.docker.internal:1234"
elif [[ $unameVal == *"Linux"* ]]; then
    DEBUG_HOST="localhost:1234"
else
    # @TODO: Windows debugging?
    echo "Unsupported OS for debugging, debug might not work"
    DEBUG_HOST="localhost:1234"
fi

gdb-multiarch build/PacmanOS.elf -ex "set confirm off" -ex "target remote $DEBUG_HOST"
