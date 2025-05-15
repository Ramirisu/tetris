# Tetris

[![Build](https://github.com/Ramirisu/tetris/actions/workflows/build.yml/badge.svg)](https://github.com/Ramirisu/tetris/actions/workflows/build.yml)
[![dependency status](https://deps.rs/repo/github/Ramirisu/tetris/status.svg)](https://deps.rs/repo/github/Ramirisu/tetris)

Classic Tetris (NES Tetris) built using the Bevy engine.

## Features

- Game Play
  - [x] DAS
  - [x] 1H2R RNG for Next Piece
  - [X] Glitched Color Palettes
  - [x] Level 39 Super Kill Screen
  - [x] Game Statistics
  - [x] Piece Distribution
  - [x] Display Input
  - [x] Sound Effect
  - [x] Support English/繁體中文/简体中文

- Platform
  - [x] High FPS Support
  - [x] Cross-platform (Web/Windows/Linux/MacOS)

> It's recommended to run the DESKTOP version, which is much smoother than the WEB version.

## Game Options

**TRANSITION**

The *TRANSITION* option determines how the level advances after clearing lines.

| Options        | Description                                                                                                                                                                                          |
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Classic        | When the player clears `(startLevel × 10 + 10)` or `max(100, (startLevel × 10 - 50))` lines, whichever comes first, the level advances by 1. After this, the level advances by 1 for every 10 lines. |
| Fixed          | When the player clears `(startLevel × 10 + 10)` lines, the level advances by 1. After this, the level advances by 1 for every 10 lines.                                                              |
| Every 10 Lines | The level advances by 1 for every `10` lines.                                                                                                                                                        |
| Every 4 Lines  | The level advances by 1 for every `4` lines.                                                                                                                                                         |

**LINECAP**

The *LINECAP* option enables the end game mechanism for competitive matches. Players can keep going under *kill screen* due to the invention of the *rolling* technique. The end game mechanism can be used to avoid endless gameplay.

| Options        | Description                                                                                                                                   |
| -------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| Off            | The gravity is the same as level 29 for level 39 and beyond.                                                                                  |
| Kill Screen X2 | NTSC: The gravity becomes *200%* of level 29 starting from level 39.<br />PAL: The gravity becomes *200%* of level 19 starting from level 29. |
| Halt           | The game ends when it reaches level 39.                                                                                                       |

**LINECAP LEVEL**

The *LINECAP LEVEL* option controls the level at which the *LINECAP* feature is triggered. By default, this is level 39 for NTSC and level 29 for PAL.

**GRAVITY**

The *GRAVITY* option determines how the gravity will increase when the level advances.

| Options | Description                                    |
| ------- | ---------------------------------------------- |
| Level   | The gravity increases when the level advances. |
| Locked  | The gravity is locked at the `startLevel`.     |

**SEEDING**

The *SEEDING* option determines how to generate NEXT pieces. The 1H2R Randomizer is used to choose the pieces.

| Options | Description                                                                                           |
| ------- | ----------------------------------------------------------------------------------------------------- |
| System  | System-provided seeds are used and result in different sequences for each game.                       |
| Custom  | User-provided seed is used, generating a constant sequence. This mode is built for competitive match. |

**SEED**

The *SEED* option determines the seed for the random number generator. It is only available when **SEEDING** is set to `CUSTOM`.

- Press `Start` to enter/leave seed configuration.
- Press `Up` and `Down` to adjust the hex value.
- Press `Select` to generate a random seed.

**SCORING**

The *SCORING* option determines how the score is displayed.

| Options | Description                                                       |      123 | 1,234,567 | 3,704,567 | 39,504,567 |
| ------- | ----------------------------------------------------------------- | -------: | --------: | --------: | ---------: |
| Decimal | Display the score in decimal up to `2^64`.                        | `000123` | `1234567` | `3704567` | `39504567` |
| Classic | Display the score in decimal up to `999999`.                      | `000123` |  `999999` |  `999999` |   `999999` |
| Base36  | Apply `base36` encoding for the 6th digit and above of the score. | `000123` |  `C34567` | `1104567` |  `AZ04567` |

**LEVELING**

The *LEVELING* option determines how the level is displayed.

| Options | Description                                                                                                     |
| ------- | --------------------------------------------------------------------------------------------------------------- |
| Decimal | Levels are always displayed in decimal.                                                                         |
| Classic | Levels above 29 will be displayed in hexadecimal. For example: level 31 will be `0A` and level 33 will be `1E`. |

**TV SYSTEM**

The *TV SYSTEM* option determines which version of NES Tetris is used. This affects DAS, gravity, and the level of linecap.

| Options | Description                                                                                                                                            |
| ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| NTSC    | The NTSC version of NES Tetris, which is specified to run at 60 frames per second.                                                                     |
| PAL     | The PAL version of NES Tetris, which is specified to run at 50 frames per second. The game is rebalanced for the slower frame rate in the PAL release. |

> Reference: https://tetris.wiki/Tetris_(NES,_Nintendo)

**NEXT PIECE HINT**

The *NEXT PIECE HINT* option determines how many NEXT pieces are displayed.

| Options | Description           |
| ------- | --------------------- |
| Off     |                       |
| Classic | Show next piece only. |
| Modern  | Show next `5` pieces. |

**INVISIBLE**

The *INVISIBLE* option determines the visibility of the locked squares.

| Options | Description                            |
| ------- | -------------------------------------- |
| Off     | All squares are visible.               |
| On      | Squares that are locked are invisible. |

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

> A, B, X, and Y button mapping is in NES/SNES Controller layout.

## Build & Run

### Web

```sh
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir ./dist/tetris --out-name "tetris" ./target/wasm32-unknown-unknown/release/tetris.wasm
cp -r ./assets ./dist/assets

# Start a web server to serve the files under "./dist"
# ex: npx serve -l 8080 ./dist
```

### Linux

[Install Linux dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)

Ubuntu

```sh
WGPU_BACKEND=vulkan cargo run --release
```

> The OpenGL backend is broken; run with the Vulkan backend instead.

### Windows & MacOS

```sh
cargo run --release
```

## License

This project is free, open source, and permissively licensed! Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](https://github.com/Ramirisu/tetris/blob/main/LICENSE-MIT) or https://opensource.org/license/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/Ramirisu/tetris/blob/main/LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer!
