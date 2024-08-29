# Tetris

[![Build](https://github.com/Ramirisu/tetris/actions/workflows/build.yml/badge.svg)](https://github.com/Ramirisu/tetris/actions/workflows/build.yml)
[![dependency status](https://deps.rs/repo/github/Ramirisu/tetris/status.svg)](https://deps.rs/repo/github/Ramirisu/tetris)

A Classic Tetris (NES Tetris) clone written in bevy/rust.

## Features

- Game Play
  - [x] DAS (16 ticks / 6 ticks)
  - [x] Initial Entry Delay (96 ticks)
  - [x] Entry Delay (10 ~ 18 ticks)
  - [x] Line Clear Delay (18 ticks)
  - [x] 1H2R Random Generator for Next Piece
  - [x] Normal Color Palettes
  - [ ] Glitched Color Palettes
  - [x] Level 39 Super Kill Screen
  - [x] Statistics
  - [x] Sound Effects

- Platform
  - [x] High FPS Support
  - [x] Cross-platform (Web/Windows/Linux/MacOS)

## Keybindings

| Action                    | Keyboard | Controller (NES) |
| :------------------------ | :------: | :--------------: |
| Move Up                   |    ↑     |        ↑         |
| Move Down                 |    ↓     |        ↓         |
| Move Left                 |    ←     |        ←         |
| Move Right                |    →     |        →         |
| Rotate Clockwisely        |    X     |        A         |
| Rotate Counterclockwisely |    Z     |        B         |
| Start                     |  Enter   |      Start       |
| Soft Reset                |   Esc    |                  |

## License

[MIT License](https://opensource.org/license/MIT)
