#!/bin/bash
source config.sh

if [[ -z $1 ]];
then
	DEBUG_FLAGS=""
else
	DEBUG_FLAGS="-s -S"
	echo "Waiting for debugger..."
fi

$PACMAN_QEMU -machine virt -machine virtualization=on -cpu max -vga none -device ramfb -monitor none -serial stdio -m 4G -kernel build/PacmanOS.elf $DEBUG_FLAGS
