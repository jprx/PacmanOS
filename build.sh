#!/bin/bash
# Temporary build script before I create a Makefile

clang -arch arm64e -ffreestanding -fno-builtin bringup.s -c
clang -arch arm64e -ffreestanding -fno-builtin hypermain.c -c

clang -arch arm64e -ffreestanding -fno-builtin bringup.o hypermain.o -o kernel
