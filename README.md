# Tetris

[![Build](https://github.com/Ramirisu/tetris/actions/workflows/build.yml/badge.svg)](https://github.com/Ramirisu/tetris/actions/workflows/build.yml)
[![dependency status](https://deps.rs/repo/github/Ramirisu/tetris/status.svg)](https://deps.rs/repo/github/Ramirisu/tetris)

A Classic Tetris (NES Tetris) clone written in BEVY/RUST.

![splash](https://github.com/Ramirisu/tetris/blob/main/docs/splash.png)
![game_play](https://github.com/Ramirisu/tetris/blob/main/docs/game_play.png)

## Features

- Game Play
  - [x] DAS (16 ticks / 6 ticks)
  - [x] Initial Entry Delay (96 ticks)
  - [x] Entry Delay (10 ~ 18 ticks)
  - [x] Line Clear Delay (18 ticks)
  - [x] 1H2R RNG for Next Piece
  - [X] Glitched Color Palettes
  - [x] Level 39 Super Kill Screen
  - [x] Piece Distribution
  - [x] Statistics
  - [x] Sound Effects
  - [ ] Musics

- Platform
  - [x] High FPS Support
  - [x] Cross-platform (Web/Windows/Linux/MacOS)

## Keybindings

| Menu       | In Game                   |     Keyboard     | Controller (NES) |
| :--------- | :------------------------ | :--------------: | :--------------: |
| Move Up    |                           |        ↑         |        ↑         |
| Move Down  | Soft Drop                 |        ↓         |        ↓         |
| Move Left  | Move Left                 |        ←         |        ←         |
| Move Right | Move Right                |        →         |        →         |
|            | Rotate Clockwisely        |        X         |        A         |
| Back       | Rotate Counterclockwisely |        Z         |        B         |
| Start      | Pause/Resume              |      Enter       |      Start       |
| Soft Reset | Soft Reset                | Left Shift / Esc |      Select      |

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

[MIT License](https://opensource.org/license/MIT)