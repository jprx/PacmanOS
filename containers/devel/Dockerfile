FROM ubuntu:20.04

RUN DEBIAN_FRONTEND="noninteractive" apt-get update -y \
    && DEBIAN_FRONTEND="noninteractive" apt-get upgrade -y \
    && DEBIAN_FRONTEND="noninteractive" apt-get install -y build-essential gcc-aarch64-linux-gnu libc6-dev-arm64-cross

WORKDIR /pacman

CMD /bin/bash
