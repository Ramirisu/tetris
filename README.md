# Tetris

[![Build](https://github.com/Ramirisu/tetris/actions/workflows/build.yml/badge.svg)](https://github.com/Ramirisu/tetris/actions/workflows/build.yml)
[![dependency status](https://deps.rs/repo/github/Ramirisu/tetris/status.svg)](https://deps.rs/repo/github/Ramirisu/tetris)

Classic Tetris (NES Tetris) written in BEVY/RUST.

![gameplay](https://github.com/Ramirisu/tetris/blob/main/docs/gameplay.png)

## Features

- Game Play
  - [x] DAS
  - [x] 1H2R RNG for Next Piece
  - [X] Glitched Color Palettes
  - [x] Level 39 Super Kill Screen
  - [x] Piece Distribution
  - [x] Statistics
  - [x] Display Inputs
  - [x] Sound Effects
  - [ ] Musics

- Platform
  - [x] High FPS Support
  - [x] Cross-platform (Web/Windows/Linux/MacOS)


## Game Options

**TRANSITION**

|  Options |                                                                                                                                                                                                       |
| -------: | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|  Classic | When the player line clear `(startLevel × 10 + 10) or max(100, (startLevel × 10 - 50))` lines, whatever comes first, the level advances by 1. After this, the level advances by 1 for every 10 lines. |
|    Fixed | When the player line clear `(startLevel x 10 + 10)` lines, the level advances by 1. After this, the level advances by 1 for every 10 lines.                                                           |
| 10 Lines | The level advacnes by 1 for every `10` lines.                                                                                                                                                         |
|  4 Lines | The level advacnes by 1 for every `4` lines.                                                                                                                                                          |

**LINECAP**

| Options |                                                                                                         |
| ------: | :------------------------------------------------------------------------------------------------------ |
|     Off | The drop speed is the same as Level 29 for Level 39 and beyond.                                         |
|      On | The drop speed is *200%* of Level 29 starting from Level 39. This is also known as *Super Kill Screen*. |

**DROPSPEED**

| Options |                                                   |
| ------: | :------------------------------------------------ |
|   Level | The drop speed increases when the level advances. |
|  Locked | The drop speed is locked at the `startLevel`.     |

**TV SYSTEM**

| Options |                                                                                                                                                           |
| ------: | :-------------------------------------------------------------------------------------------------------------------------------------------------------- |
|    NTSC | The NTSC version of NES Tetris, which is specified to run at 60 frames per second.                                                                        |
|     PAL | The PAL version of NES Tetris, which is specified to run at 50 frames per second. The game is rebalanced for the slower frames per second in PAL release. |

> Reference: https://tetris.wiki/Tetris_(NES,_Nintendo)

**NEXT PIECE HINT**

| Options |                       |
| ------: | :-------------------- |
|     Off |                       |
| Classic | Show next piece only. |
|  Modern | Show next `5` pieces. |


## Keybindings

| Menu       | In Game                 | Keyboard | Controller: Mapping A  | Controller: Mapping B  |
| :--------- | :---------------------- | :------: | :--------------------: | :--------------------: |
| Move Up    |                         |    ↑     |           ↑            |           ↑            |
| Move Down  | Soft Drop               |    ↓     |           ↓            |           ↓            |
| Move Left  | Move Left               |    ←     |           ←            |           ←            |
| Move Right | Move Right              |    →     |           →            |           →            |
|            | Rotate Clockwise        |    X     |         A (→)          |         B (↓)          |
| Back       | Rotate Counterclockwise |    Z     |         B (↓)          |         Y (←)          |
| Start      | Pause/Resume            |  Enter   |         Start          |         Start          |
| Soft Reset | Soft Reset              |   Esc    | Select + Start + A + B | Select + Start + B + Y |

> A, B, X and Y button mapping is in NES/SNES/NSwitch Controller layout.

## Build & Run

#### Web

```sh

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./dist/tetris --out-name "tetris" ./target/wasm32-unknown-unknown/release/tetris.wasm
cp -r ./assets ./dist/assets

# Start a web server to serve the files under "./dist"

```

#### Linux

Ubuntu

```sh

apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
cargo run --release

```

#### Windows

```sh

cargo run --release

```

#### MacOS

```sh

cargo run --release

```

## License

This project is free, open source and permissively licensed! Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](https://github.com/Ramirisu/tetris/blob/main/LICENSE-MIT) or https://opensource.org/license/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/Ramirisu/tetris/blob/main/LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer!
