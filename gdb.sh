#!/bin/bash
source config.sh
gdb-multiarch build/PacmanOS.elf -ex "set confirm off" -ex "target remote $DEBUG_HOST"
