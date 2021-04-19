# game

Please follow the instructions below in order, while it seems complicated, it makes having builds for web and native PC's very easy.

## Instructions
### Windows
- Please install [VS2019 Build Tools](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)
- `cargo install -f cargo-binutils`
- `rustup component add llvm-tools-preview`
- `rustup toolchain install nightly`
- `cargo install -f cargo-make`
### Arch/Manjaro Linux
- `sudo pacman -Syu clang lld libx11 pkgconf alsa-lib`
- `rustup toolchain install nightly`
- `cargo install -f cargo-make`
### MacOS
MacOS, of course, does not have a working LLD linker (thanks Apple), though the ZLD linker is still faster than the default
- `brew install michaeleisel/zld/zld`
- `rustup toolchain install nightly`
- `cargo install -f cargo-make`


Occasionally, please run `rustup update` to update to the latest version of the nightly compiler.
The nightly compiler allows us to use unstable Rust optimizations for both faster build times and faster bineries.

`cargo-make` allows us to use custom build configurations for the web and for native builds.


To run the game in development mode (with very very fast build times), run:
`cargo make run`

To run it in release mode:
`cargo make run-release`

To buld for release mode:
`cargo make build-release`

Thanks to `bevy_webgl2`, this game can run on the web! When running the game, expect to wait a few seconds on the page with a completely blank, or a canvas with black lines. This is because web builds are typically slower, since they're single threaded. Because of this, WASM builds will be release by default.

I'd recommend using Chrome (or Chromium based browsers like Brave) for testing WASM builds. The performance on Firefox is pretty bad (30 fps) without messing with flags, and even then it's still slightly worse than Chrome.

To build for the WASM (Web ASseMbly), run:
`cargo make serve`

To run a debug WASM build (not recommended since the performance is horrible), run:
`cargo make serve-debug`

