FROM gitpod/workspace-full-vnc
RUN sudo apt update && sudo apt upgrade -y && sudo apt install clang lld cmake libssl-dev build-essential pkg-config libx11-dev libasound2-dev libudev-dev mesa-vulkan-drivers firefox -y
RUN curl -LO https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb && apt-get install -y ./google-chrome-stable_current_amd64.deb && rm google-chrome-stable_current_amd64.deb 
RUN rustup default nightly
RUN cargo install -f cargo-make && cargo install -f wasm-bindgen-cli --version 0.2.74 && cargo install -f basic-http-server
RUN rustup target install wasm32-unknown-unknown
RUN rustup component add rust-src
