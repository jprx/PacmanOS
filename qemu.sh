#!/bin/bash
# qemu-system-aarch64 -machine versatilepb -cpu cortex-a57 -kernel build/PacmanOS.elf -s -S
# qemu-system-aarch64 -machine virt -vga std -cpu cortex-a57 -kernel build/PacmanOS.elf -monitor stdio -s -S
# ~/Documents/qemu/build/qemu-system-aarch64 -machine virt -cpu cortex-a57 -vga none -device ramfb -monitor stdio -m 4G -kernel build/PacmanOS.elf -s -S
~/Documents/qemu/build/qemu-system-aarch64 -machine virt -cpu max -vga none -device ramfb -monitor stdio -m 4G -kernel build/PacmanOS.elf -s -S
