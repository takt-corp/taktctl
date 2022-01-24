FROM ubuntu:focal

ENV CROSS_DOCKER_IN_DOCKER=true
ENV DEBIAN_FRONTEND="noninteractive"

RUN apt-get update \
  && apt-get upgrade -y \
  && apt-get install -y -q pkg-config openssl libssl-dev build-essential curl \
  && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

ENV OPENSSL_DIR=/usr/lib/ssl
ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl
ENV OPENSSL_LIB_DIR=/usr/lib/ssl
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install -f cross

RUN rustup target add x86_64-unknown-linux-gnu \
  && rustup target add x86_64-pc-windows-gnu \
  && rustup target add x86_64-apple-darwin \
  && rustup target add aarch64-unknown-linux-gnu \
  && rustup target add armv7-unknown-linux-gnueabihf

WORKDIR /app

ENTRYPOINT [ "cargo" ]
