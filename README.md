# game

Please instal the LLD linker (faster linker). Due to the way the .cargo/config.toml file works, you absolutely need to follow those steps (but you'll thank me later), but it results in much much faster build times.

## Instructions
### Windows
- Please install [VS2019 Build Tools](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)
- `cargo install -f cargo-binutils`
- `rustup component add llvm-tools-preview`
- `rustup toolchain install nightly`
### Arch/Manjaro Linux
- `sudo pacman -Syu lld libx11 pkgconf alsa-lib`
- `rustup toolchain install nightly`
### MacOS
MacOS, of course, does not have a working LLD linker (thanks Apple), though the ZLD linker is still faster than the default
- `brew install michaeleisel/zld/zld`
- `rustup toolchain install nightly`


Occasionally, please run `rustup update` to update to the latest version of the nightly compiler.
The nightly compiler allows us to use unstable Rust optimizations for both faster build times and faster bineries


To run the game in development mode (with very very fast build times), run 
`cargo run --features bevy/dynamic`

To run it in release mode:
`cargo run --release`
