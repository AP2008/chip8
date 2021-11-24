# A CHIP-8 Emulator

![Demo](./pic.png)

## Features

* Built using `Rust`
* Statically links `SDL2` (no need to install `SDL2` for building)
* Can run all ROMS
* Has a Sound Timer

## Building
`cargo build --release`

## Running
`target/release/chip8 <path-to-rom>` (If no `rom` is provided, it will use the default rom)

## Resources
* [Cowgod's technical reference]
* [Sample Roms]

[Cowgod's technical reference]: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
[Sample Roms]: https://web.archive.org/web/20130702032522if_/http://www.chip8.com/downloads/Chip-8%20Pack.zip
