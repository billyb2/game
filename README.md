# game

Please follow the instructions below in order, while it seems complicated, it makes having builds for web and native PC's very easy.

## Pre-build Instructions
### Firstly, install [Rust](https://www.rust-lang.org/tools/install)

### Windows
- Please install [VS2019 Build Tools](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)
- [Install CMake](https://cmake.org/download/)
- [Install OpenSSL](https://slproweb.com/download/Win64OpenSSL-1_1_1k.msi)
- Add the *OPENSSL_DIR* system environment variable, with the value being the *exact* path you installed OpenSSL to (default is C:\Program Files\OpenSSL-Win64 , but please double check to make sure)
- `cargo install -f cargo-binutils && cargo install -f wasm-bindgen-cli --version 0.2.74 && cargo install -f basic-http-server`
- `rustup component add llvm-tools-preview`
- `rustup toolchain install nightly`
- `cargo install -f cargo-make`
- ### Ubuntu/Debian Linux
- `sudo apt-get install cmake clang lld libx11 pkgconf alsa-lib openssl`
- `rustup toolchain install nightly`
- `cargo install -f cargo-make && cargo install -f wasm-bindgen-cli --version 0.2.74 && cargo install -f basic-http-server`
### Arch/Manjaro Linux
- `sudo pacman -Syu cmake clang lld libx11 pkgconf alsa-lib openssl --needed`
- `rustup toolchain install nightly`
- `cargo install -f cargo-make && cargo install -f wasm-bindgen-cli --version 0.2.74 && cargo install -f basic-http-server`
### MacOS
MacOS, of course, does not have a working LLD linker (thanks Apple), though the ZLD linker is still faster than the default
- [Install CMake](https://cmake.org/download/)
- `brew install michaeleisel/zld/zld`
- `rustup toolchain install nightly`
- `cargo install -f cargo-make && cargo install -f wasm-bindgen-cli --version 0.2.74 && cargo install -f basic-http-server`

Occasionally, please run `rustup update` to update to the latest version of the nightly compiler.
The nightly compiler allows us to use unstable Rust optimizations for both faster build times and faster bineries.

`cargo-make` allows us to use custom build configurations for the web and for native builds.

## Instructions for running / building

To run the game in development mode (with very very fast build times), run:
`cargo make run`

To run it in release mode:
`cargo make run-release`

To buld for release mode:
`cargo make build-release`

Thanks to `bevy_webgl2`, this game can run on the web! When running the game, expect to wait a few seconds on the page with a completely blank, or a canvas with black lines. This is because web builds are typically slower, since they're single threaded. Because of this, WASM builds will be release by default.

I'd recommend using Chrome (or Chromium based browsers like Brave) for testing WASM builds. The performance on Firefox is pretty bad (30 fps) without messing with flags, and even then it's still slightly worse than Chrome.

To build for WASM (Web ASseMbly), run:
`cargo make serve`

To make a very optimized build for WASM (slow build times, fast run times), run:
`cargo make serve-release-simd`

To run a debug WASM build (not recommended since the performance is horrible), run:
`cargo make serve-fast`

I only recommend debug builds if you want to very quickly check a change, rather than playtesting the game

## How the map editor works
Currently, the game uses a custom binary format that I made designed to take up very little space (maps used to take up a few MB, now they're a few hundred bytes), and is also future proofed to be easy to add custom assets (since the final map is compressed with LZ4, multiple files could be added and then compressed into a single archive). Anyway, enough patting myself on the back, how do you set up the map editor?

### What makes the maps look the way they do
The map itself is stored as the map1.tmx file. Each object that you see in the game is a thing in Tiled called an object. These objects can be created relatively easily, though I recommend copying and pasting an already existing object, since each object has certain custom attributes added to it. As of writing, these attributes are the RGBA (the colors) of said objects, whether the object can be a player spawn, and whether it's possible to collide with aid objet. For example, the current map1 that we use has 3 player spawns, each with an alpha value of 0 (meaning they're completely transparent), they're set to not be player collidable, and they're set to be player spawns. All other objects in the game are either red or blue, are not player spawns, and are player collidable. The map itself also has some metadata associated with it, including the size of the map and the background color.

Now this information might seem useless, but it's very important to understand these as you make your own lovely maps. If a map object, for example, doesn't include all of the attributes it's supposed to, the map might not build correctly, the map file could be corrupted, and more!

### How does the map format actually work?
Now this part isn't necessarily important unless you want to add to the map format. If you don't really care, just skip to the section below. 

The entire map1.custom file uses binary, in order to be very efficient to load and interpret, though this has the disadvantage of being basically unreadable without knowing the map format.

At the beginning of the file, the metadata of the map is stored (currently the width, height, and background color of the map), which takes up 11 bytes.

Then, for every map object, the x, y, z (how objects are stacked onto each other) coordinates and w (rotation) (4 bytes each for a total of 16), the width and height (4 bytes each for a total of 8 bytes) of said object, whether a player can spawn in that object (1 byte), and whether the player can collide with that object (1 byte) are stored, as well as the health of the object (1 byte) and the color of the object (4 bytes total). Generally for the boolean data, 0 is false and 255 is true, and for the health of the map object, 0 is considered indestructible, and then anything above 0 is the actual health of the map object. So in total, the every object takes up a relatively small 32 bytes.

Then, the map has a built in checksum (a way of knowing if a map file has been corrupted or not), that uses the CRC32 algorithm (since it's very fast and does the job well). The CRC32 takes up 32 more bytes, and is appended after the null map object. The CRC32 is also useful for uniquely identifying maps, so that when a playe rtries to join another player's game, the client can check to see if the checksums match, and if not, request to download the map.

Finally, the entire map is compressed using the LZ4 compression algorithm, since it has relatoively slow compression times, but very very fast decompression times. The custom map extension is used since it's not anything like JSON or XML, it's an entirely new binary format I made just for this game (though eventually we should probably use a real file format name so people can recognize it).

To view how all of this works, please see the custom.js file. I apologize for how much code there is in the first line, that's a WASM version of the original C-lang LZ4 compression algorithm, since the JavaScript ones I used before were too slow for my liking.

### Setting up and using Tiled
Alright now, time to actually set up Tiled (map editor):

1. First, install tiled:
- Arch/Manjaro: `sudo pacman -Syu tiled`
Many many many distributions do not have Tiled in their software repositories, so unless you want to compile Tiled yourself, I'd recommend using Flatpak
- Other OS: First, [install Flatpak](https://www.flatpak.org/setup/) (included by default on Zorin), second, `flatpak install flathub org.mapeditor.Tiled`
2. Secondly, create a symlink between the `custom.js` file and `$HOME/.config/tiled/extensions/`
This is as easy as running `mkdir -p $HOME/.config/tiled/extensions/ && ln -s /path/to/game/repository/tiled/custom.js $HOME/.config/tiled/extensions/`, replacing /path/to/game/repository with the actual path of the repository of course.
3. When you've finally created your glorious map (using the map1.tmx as an example of course), go to File->Export As, and change `File type` to `Custom map format (*.custom)`. Then, the `custom.js` file will do all the magic of properly encoding the file into something the game can parse!


## Before pushing your Git commit

Please try running `cargo make serve-fast` and verifying that WASM builds and runs correctly. I know WASM builds take a long time, but if a commit slips through that doesn't work with WASM, it makes it far more difficult to debug why native builds work and why WASM builds don't later.
