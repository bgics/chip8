# CHIP-8 Emulator

A fully-featured CHIP-8 emulator written in Rust, featuring a graphical user interface built with [egui](https://github.com/emilk/egui).

## Project Overview

CHIP-8 is an interpreted programming language developed in the mid-1970s for the COSMAC VIP and Telmac 1800 microcomputers. It was designed to simplify game development for early 8-bit systems. This emulator faithfully recreates the CHIP-8 virtual machine, allowing you to run classic CHIP-8 programs and games on modern hardware.

### Why This Project?

- Learn about emulator development and Rust

## Features

### Supported Instructions

| Opcode | Instruction | Description |
|--------|-------------|-------------|
| `00E0` | `CLS` | Clear the display |
| `00EE` | `RET` | Return from subroutine |
| `1nnn` | `JP addr` | Jump to address nnn |
| `2nnn` | `CALL addr` | Call subroutine at nnn |
| `3xkk` | `SE Vx, byte` | Skip next instruction if Vx == kk |
| `4xkk` | `SNE Vx, byte` | Skip next instruction if Vx != kk |
| `5xy0` | `SE Vx, Vy` | Skip next instruction if Vx == Vy |
| `6xkk` | `LD Vx, byte` | Set Vx = kk |
| `7xkk` | `ADD Vx, byte` | Set Vx = Vx + kk |
| `8xy0` | `LD Vx, Vy` | Set Vx = Vy |
| `8xy1` | `OR Vx, Vy` | Set Vx = Vx OR Vy |
| `8xy2` | `AND Vx, Vy` | Set Vx = Vx AND Vy |
| `8xy3` | `XOR Vx, Vy` | Set Vx = Vx XOR Vy |
| `8xy4` | `ADD Vx, Vy` | Set Vx = Vx + Vy, VF = carry |
| `8xy5` | `SUB Vx, Vy` | Set Vx = Vx - Vy, VF = NOT borrow |
| `8xy6` | `SHR Vx` | Set Vx = Vx >> 1, VF = LSB |
| `8xy7` | `SUBN Vx, Vy` | Set Vx = Vy - Vx, VF = NOT borrow |
| `8xyE` | `SHL Vx` | Set Vx = Vx << 1, VF = MSB |
| `9xy0` | `SNE Vx, Vy` | Skip next instruction if Vx != Vy |
| `Annn` | `LD I, addr` | Set I = nnn |
| `Bnnn` | `JP V0, addr` | Jump to address nnn + V0 |
| `Cxkk` | `RND Vx, byte` | Set Vx = random byte AND kk |
| `Dxyn` | `DRW Vx, Vy, n` | Draw n-byte sprite at (Vx, Vy), VF = collision |
| `Ex9E` | `SKP Vx` | Skip next instruction if key Vx is pressed |
| `ExA1` | `SKNP Vx` | Skip next instruction if key Vx is not pressed |
| `Fx07` | `LD Vx, DT` | Set Vx = delay timer |
| `Fx0A` | `LD Vx, K` | Wait for key press, store in Vx |
| `Fx15` | `LD DT, Vx` | Set delay timer = Vx |
| `Fx18` | `LD ST, Vx` | Set sound timer = Vx |
| `Fx1E` | `ADD I, Vx` | Set I = I + Vx |
| `Fx29` | `LD F, Vx` | Set I = location of sprite for digit Vx |
| `Fx33` | `LD B, Vx` | Store BCD representation of Vx at I, I+1, I+2 |
| `Fx55` | `LD [I], Vx` | Store V0 to Vx in memory starting at I |
| `Fx65` | `LD Vx, [I]` | Read V0 to Vx from memory starting at I |

### Display

- **Resolution**: 64×32 monochrome pixels
- **Rendering**: XOR sprite drawing with collision detection
- **Configurable Colors**: Customize ON/OFF pixel colors via GUI

### Timers

- **Delay Timer (DT)**: Decrements at 60Hz, can be read/written by programs
- **Sound Timer (ST)**: Decrements at 60Hz, plays tone when non-zero (audio not yet implemented)

### Input

- **16-key hexadecimal keypad** (0-9, A-F)
- **Remappable controls** via in-app GUI
- **Default keyboard mapping**:

```
CHIP-8 Keypad    →    Keyboard
┌───┬───┬───┬───┐    ┌───┬───┬───┬───┐
│ 1 │ 2 │ 3 │ C │    │ 1 │ 2 │ 3 │ 4 │
├───┼───┼───┼───┤    ├───┼───┼───┼───┤
│ 4 │ 5 │ 6 │ D │ →  │ Q │ W │ E │ R │
├───┼───┼───┼───┤    ├───┼───┼───┼───┤
│ 7 │ 8 │ 9 │ E │    │ A │ S │ D │ F │
├───┼───┼───┼───┤    ├───┼───┼───┼───┤
│ A │ 0 │ B │ F │    │ Z │ X │ C │ V │
└───┴───┴───┴───┘    └───┴───┴───┴───┘
```

### Save States

- Save and load emulator state at any time
- States are serialized using bincode format (`.sav` files)

## Demo / Screenshots

*Screenshots coming soon.*

## Getting Started

Make sure you have the Rust toolchain installed, then run:

```bash
cargo run
```

## Usage

### Loading ROMs

1. Launch the emulator with `cargo run`
2. Go to **File → Load ROM**
3. Select a `.ch8` ROM file

The `rom/` directory contains several sample ROMs for testing:
- `pong.ch8` - Classic Pong game
- `tetris.ch8` - Tetris clone
- `space_invaders.ch8` - Space Invaders clone
- `breakout.ch8` - Breakout game
- `ibm_logo.ch8` - IBM logo test ROM
- `corax_test.ch8` - Instruction test suite
- And more...

### File Formats

| Extension | Description |
|-----------|-------------|
| `.ch8` | CHIP-8 ROM file (raw binary) |
| `.sav` | Save state file (bincode serialized) |

### Saving and Loading States

- **Save State**: File → Save State → Choose location
- **Load State**: File → Load State → Select `.sav` file

### Remapping Keys

1. Go to **Edit → Remap Keys**
2. Click **Edit** next to the key you want to remap
3. Press the desired keyboard key
4. Press **Enter** to confirm

To reset to default mappings: **Edit → Reset keymapping**

### Color Configuration

1. Go to **Config → Color Config**
2. Adjust RGB values for OFF (background) and ON (foreground) colors

## Project Structure

```
chip8/
├── src/
│   ├── main.rs          # Application entry point
│   ├── lib.rs           # Library exports and message types
│   ├── app.rs           # GUI application (egui-based)
│   ├── chip8.rs         # Main emulator orchestration
│   ├── cpu.rs           # CPU emulation (registers, instruction execution)
│   ├── instruction.rs   # Opcode decoding and instruction types
│   ├── memory.rs        # 4KB memory with font data
│   ├── frame_buffer.rs  # 64×32 display buffer
│   ├── key_matrix.rs    # 16-key input state
│   ├── key_mapping.rs   # Keyboard-to-CHIP8 key mapping
│   ├── handle.rs        # Thread management for emulator loop
│   ├── channel.rs       # Message passing between threads
│   ├── chip8_state.rs   # Save state serialization
│   ├── file_picker.rs   # File dialog handling
│   ├── remap.rs         # Key remapping UI state
│   └── error.rs         # Error types
├── rom/                  # Sample ROM files
├── Cargo.toml           # Rust dependencies
└── README.md            # This file
```

## Roadmap / TODO

- [ ] Audio support (beep when sound timer > 0)
- [ ] CPU speed control (adjustable clock rate)
- [ ] Disassembler view
- [ ] Comprehensive test coverage

## Dependencies

- [eframe/egui](https://github.com/emilk/egui) - Immediate-mode GUI framework
- [rand](https://crates.io/crates/rand) - Random number generation
- [rfd](https://crates.io/crates/rfd) - Native file dialogs
- [bincode](https://crates.io/crates/bincode) - Binary serialization
- [serde](https://crates.io/crates/serde) - Serialization framework

## License

This project is licensed under the MIT License - see below for details.

```
MIT License

Copyright (c) 2024

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## Acknowledgments

- [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [CHIP-8 Wikipedia Article](https://en.wikipedia.org/wiki/CHIP-8)
- [Timendus CHIP-8 Test Suite](https://github.com/Timendus/chip8-test-suite)
