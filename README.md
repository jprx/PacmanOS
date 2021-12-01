# PacmanOS
An aarch64 operating system / hypervisor targetting the M1 chip, designed for microarchitecture research and also for fun

## Virtualization

I am also building support for the Qemu `virt` machine into PacmanOS so it can detect if it was booted under Qemu and act accordingly.

## Building

You'll need rust nightly and the `aarch64-unknown-linux-gnu` target

First install Rust and then run the following:

```
rustup update nightly
rustup override add nightly
cargo install xargo
rustup component add rust-src
rustup target add aarch64-unknown-linux-gnu
```

Then you should be able to build the rust component of PacmanOS with `build.sh`

## Linking

You can link the built Rust binary with the `linker.ld` linker script using the provided container and Makefile:

Link:

`docker-compose run devel make`

Cleanup:

`docker-compose run devel make clean`

If you're on Linux and have `aarch64-linux-gnu-ld` then you can just run `make` from the command line to link

# Credits

The linker script and infrastructure come from the [m1n1 project](https://github.com/AsahiLinux/m1n1)

Thanks the Asahi Linux team for their wonderful detailed reverse engineering of the M1 boot process!
