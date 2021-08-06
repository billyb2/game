FROM gitpod/workspace-full-vnc
RUN sudo apt update && sudo apt upgrade -y && sudo apt install clang lld cmake libssl-dev build-essential pkg-config libx11-dev libasound2-dev libudev-dev mesa-vulkan-drivers firefox -y
RUN rustup default nightly
RUN cargo install -f cargo-make && cargo install -f wasm-bindgen-cli --version 0.2.74
RUN rustup target install wasm32-unknown-unknown
RUN rustup component add rust-src
