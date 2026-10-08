# RustBoy - Game Boy Emulator

An Original Game Boy (DMG) emulator written in Rust with Winit+Pixels and Rodio.

## Features

- Accurate implementation of all CPU instructions (Passes [Blargg's](https://github.com/retrio/gb-test-roms) CPU instruction test suite)
- PPU implementation using Winit+Pixels to draw to a window
- APU implementation of all 4 channels
- Supports MBC0, 1, 3 and 5

### Limitations

- Audio glitches in channels 3 & 4 with notes going for longer that expected
- No RAM saving supported
- Some PPU timing issues

## Controls
| Button | Key |
| --- | --- |
| A | X | 
| B | Z |
| Start | S |
| Select | A |
| Up | ↑ |
| Down | ↓ | 
| Left | ← |
| Right | → |

## Usage 

Requires Rust and Cargo for building

### Running
No ROMs are provided
```bash
cargo run --release -- <path/to/game.gb>
```

## Specification

The Game Boy specification can be found in the [pandocs](https://gbdev.io/pandocs/Specifications.html)

