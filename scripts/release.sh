#!/usr/bin/env bash
set -e

# BUILDER_IMAGE_NAME=taktctl/builder:latest

# compile_target() {
#   echo "Compiling $1 in a docker container...."

#   docker container run -it --rm \
#     -v /var/run/docker.sock:/var/run/docker.sock \
#     -v $(pwd):/app \
#     $BUILDER_IMAGE_NAME \
#     build --release --target $1
# }

# echo "Build the builder image..."
# docker image build -t $BUILDER_IMAGE_NAME .

# echo "Compiling Binaries..."
# compile_target x86_64-unknown-linux-gnu
# compile_target x86_64-pc-windows-gnu
# compile_target x86_64-apple-darwin

rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-unknown-linux-gnu
rustup target add armv7-unknown-linux-gnueabihf

cargo build --target x86_64-unknown-linux-gnu
cargo build --target x86_64-pc-windows-gnu
