#!/bin/bash
# qemu-system-aarch64 -machine versatilepb -cpu cortex-a57 -kernel build/PacmanOS.elf -s -S
qemu-system-aarch64 -machine virt -vga std -cpu cortex-a57 -kernel build/PacmanOS.elf -monitor stdio -s -S
