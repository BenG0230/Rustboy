# RustBoy
Ongoing

A Rust-based Gameboy emulator. Passes [Blargg's](https://github.com/retrio/gb-test-roms) CPU and Instruction Timing test ROMs.
currently only emulates the CPU printing serial output to stdout.

### Usage
```
# Build the emulator
cargo build

# Run a test ROM
cargo run -- <path-to-rom>
```
