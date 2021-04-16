# game

Please instal the LLD linker (faster linker). Due to the way the .cargo/config.toml file works, you absolutely need to follow those steps (but you'll thank me later), but it results in much much faster build times.

## Instructions
### Windows
- Please install [VS2019 Build Tools](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)
- `cargo install -f cargo-binutils`
- `rustup component add llvm-tools-preview`
- `rustup toolchain install nightly`
- `cargo install -f cargo-make`
### Arch/Manjaro Linux
- `sudo pacman -Syu lld libx11 pkgconf alsa-lib`
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
`cargo make run --features bevy/dynamic`

To run it in release mode:
`cargo make run --release`

Thanks th `bevy_webgl2`, this game can run on the web! When running the game, expect to wait a few seconds on the page with a completely blank, or a canvas with black lines. This is because web builds are typically slower, since they're single threaded. Because of this, WASM builds will be release by default.

To build for the WASM (Web ASseMbly), run:
`cargo make serve`

