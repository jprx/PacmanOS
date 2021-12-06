#!/bin/bash
source config.sh
$PACMAN_QEMU -machine virt -machine virtualization=on -cpu max -vga none -device ramfb -monitor stdio -m 4G -kernel build/PacmanOS.elf -s -S
