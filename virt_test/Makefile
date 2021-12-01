
.PHONY: all
all: virt.elf

virt.elf: virt.o
	aarch64-linux-gnu-ld virt.o -o virt.elf

virt.o: virt.s
	aarch64-linux-gnu-as virt.s -o virt.o
