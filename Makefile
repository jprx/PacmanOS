TOOLCHAIN_PREFIX ?= aarch64-linux-gnu-

CC := $(TOOLCHAIN_PREFIX)gcc
AS := $(TOOLCHAIN_PREFIX)as
LD := $(TOOLCHAIN_PREFIX)ld
OBJCOPY := $(TOOLCHAIN_PREFIX)objcopy

CFLAGS := -ffreestanding -fpic -fno-pie
LDFLAGS := -T linker_qemu.ld -EL -maarch64elf -z notext -z nocopyreloc --gc-sections -static -pie --gc-sections

RUST_OBJECTS := \
	target/applem1-pacmanos-none/debug/libpacman_hypervisor.a

OBJECTS := \

BUILD_OBJS := $(patsubst %,build/%,$(OBJECTS)) \
	$(RUST_OBJECTS)

TARGET := PacmanOS

DEPDIR := build/.deps

.PHONY: all clean
all : build/$(TARGET).macho

clean:
	rm -rf build/*

# What's going on here:
# % is used for pattern matching- within this rule, % will always be the same thing (name of the .s file in this case)
# $@ = target of this rule
# $< = first prerequisite of the rule
# Prefixing a line with @ makes it silent
build/%.o: %.s
	@# Display what we are doing
	@echo "  AS    $<"

	@# Make the build directory if it doesn't exist
	@mkdir -p "$(dir $@)"

	@# Assemble the file
	@$(CC) -c $(CFLAGS) $< -o $@

# Same deal for C files
build/%.o: %.c
	@echo "  CC    $<"
	@mkdir -p "$(dir $@)"
	@$(CC) -c $(CFLAGS) $< -o $@

# Start by linking everything together into an ELF
# The first section of this ELF is the macho header :)
# See the .macho step for how this all comes together
build/$(TARGET).elf: $(BUILD_OBJS) linker.ld
	@echo "  LD    $@"
	@$(LD) $(LDFLAGS) -o $@ $(BUILD_OBJS)

# Idea here is from the m1n1 Makefile (https://github.com/AsahiLinux/m1n1)
# As we build the macho header by hand in the linker script, just using objcopy to
# copy the binary over will discard the ELF wrapper and leave us with a whole macho file
build/$(TARGET).macho: build/$(TARGET).elf
	@echo "  MACHO    $@"
	@$(OBJCOPY) -O binary --strip-debug $< $@
