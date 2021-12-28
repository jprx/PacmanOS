FROM ubuntu:20.04

RUN DEBIAN_FRONTEND="noninteractive" apt-get update -y \
    && DEBIAN_FRONTEND="noninteractive" apt-get upgrade -y \
    && DEBIAN_FRONTEND="noninteractive" apt-get install -y build-essential gcc-aarch64-linux-gnu libc6-dev-arm64-cross gdb gdb-multiarch curl

# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
# ENV PATH="/root/.cargo/bin:${PATH}"
# RUN rustup update nightly
# RUN rustup override add nightly
# RUN rustup component add rust-src
# RUN rustup target add aarch64-unknown-linux-gnu

WORKDIR /pacman

CMD /bin/bash
