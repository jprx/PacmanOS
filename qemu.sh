#!/bin/bash
source config.sh
$PACMAN_QEMU -machine virt -cpu max -vga none -device ramfb -monitor stdio -m 4G -kernel build/PacmanOS.elf -s -S
