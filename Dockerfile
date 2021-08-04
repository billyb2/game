FROM gitpod/workspace-full
RUN sudo apt update && sudo apt upgrade -y && sudo apt install clang lld cmake libssl-dev build-essential pkg-config libx11-dev libasound2-dev libudev-dev mesa-vulkan-drivers -y
RUN rustup default nightly
RUN cargo install -f cargo-make
RUN rustup target install wasm32-unknown-unknown
RUN rustup component add rust-src
RUN cargo make build-web-fast-simd
