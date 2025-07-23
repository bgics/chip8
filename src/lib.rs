use std::{
    fs::File,
    io::{self, Read},
};

enum Instruction {
    Halt,
    Cls,
    Ret,
    Jp { addr: u16 },
    LdI { addr: u16 },
    LdByte { vx: u8, byte: u8 },
    LdReg { vx: u8, vy: u8 },
    LdRegDt { vx: u8 },
    LdDt { vx: u8 },
    LdFont { vx: u8 },
    AddByte { vx: u8, byte: u8 },
    AndReg { vx: u8, vy: u8 },
    XorReg { vx: u8, vy: u8 },
    AddRegCarry { vx: u8, vy: u8 },
    SubReg { vx: u8, vy: u8 },
    Shr { vx: u8 },
    Shl { vx: u8 },
    OrReg { vx: u8, vy: u8 },
    SeByte { vx: u8, byte: u8 },
    SeReg { vx: u8, vy: u8 },
    SneByte { vx: u8, byte: u8 },
    SneReg { vx: u8, vy: u8 },
    Store { vx: u8 },
    StoreBcd { vx: u8 },
    Read { vx: u8 },
    Call { addr: u16 },
    Drw { vx: u8, vy: u8, nibble: u8 },
    Unknown { instruction: u16 },
}

type FrameBuffer = [[bool; 64]; 32];

type DrawCallback = Option<Box<dyn Fn(&FrameBuffer) + Send>>;

pub struct Chip8 {
    cpu: Cpu,
    mem: Memory,
    frame_buffer: FrameBuffer,
    draw_callback: DrawCallback,
    halted: bool,
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Self {
            cpu: Cpu::new(),
            mem: Memory::new(),
            frame_buffer: [[false; 64]; 32],
            draw_callback: None,
            halted: false,
        }
    }

    pub fn set_draw_callback<F>(&mut self, callback: F)
    where
        F: Fn(&FrameBuffer) + 'static + Send,
    {
        self.draw_callback = Some(Box::new(callback))
    }

    pub fn load_rom(&mut self, file_name: &str) -> io::Result<()> {
        let mut file = File::open(file_name)?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        self.mem.load_rom(&buffer);

        Ok(())
    }

    pub fn update_60hz(&mut self) {
        if self.cpu.dt > 0 {
            self.cpu.dt -= 1
        }
    }

    pub fn tick(&mut self) {
        if self.halted {
            return;
        }

        let instruction = self.fetch();
        let decoded_instruction = Self::decode(instruction);
        self.execute(decoded_instruction);
    }

    fn fetch(&mut self) -> u16 {
        let pc = self.cpu.pc;

        assert!(pc < 4094);

        let msb = self.mem.get_byte(pc);
        let lsb = self.mem.get_byte(pc + 1);

        self.cpu.pc += 2;

        ((msb as u16) << 8) | lsb as u16
    }

    fn decode(instruction: u16) -> Instruction {
        if instruction == 0x0000 {
            Instruction::Halt
        } else if instruction == 0x00E0 {
            Instruction::Cls
        } else if instruction == 0x00EE {
            Instruction::Ret
        } else if instruction >> 12 == 0x1 {
            let addr = instruction & 0xFFF;

            Instruction::Jp { addr }
        } else if instruction >> 12 == 0x2 {
            let addr = instruction & 0xFFF;

            Instruction::Call { addr }
        } else if instruction >> 12 == 0x6 {
            let vx = ((instruction >> 8) & 0x0F) as u8;
            let byte = (instruction & 0xFF) as u8;

            Instruction::LdByte { vx, byte }
        } else if instruction >> 12 == 0x7 {
            let vx = ((instruction >> 8) & 0x0F) as u8;
            let byte = (instruction & 0xFF) as u8;

            Instruction::AddByte { vx, byte }
        } else if instruction >> 12 == 0xA {
            let addr = instruction & 0xFFF;

            Instruction::LdI { addr }
        } else if instruction >> 12 == 0xD {
            let vx = ((instruction >> 8) & 0x0F) as u8;
            let vy = ((instruction >> 4) & 0x0F) as u8;
            let nibble = (instruction & 0x0F) as u8;

            Instruction::Drw { vx, vy, nibble }
        } else if instruction >> 12 == 0x3 {
            let vx = ((instruction >> 8) & 0x0F) as u8;
            let byte = (instruction & 0xFF) as u8;

            Instruction::SeByte { vx, byte }
        } else if instruction >> 12 == 0x4 {
            let vx = ((instruction >> 8) & 0x0F) as u8;
            let byte = (instruction & 0xFF) as u8;

            Instruction::SneByte { vx, byte }
        } else if instruction >> 12 == 0x5 && instruction & 0x0F == 0x0 {
            let vx = ((instruction >> 8) & 0x0F) as u8;
            let vy = ((instruction >> 4) & 0x0F) as u8;

            Instruction::SeReg { vx, vy }
        } else if instruction >> 12 == 0x9 && instruction & 0x0F == 0x0 {
            let vx = ((instruction >> 8) & 0x0F) as u8;
            let vy = ((instruction >> 4) & 0x0F) as u8;

            Instruction::SneReg { vx, vy }
        } else if instruction >> 12 == 0x8 && instruction & 0x0F == 0x0 {
            let vx = ((instruction >> 8) & 0x0F) as u8;
            let vy = ((instruction >> 4) & 0x0F) as u8;

            Instruction::LdReg { vx, vy }
        } else if instruction >> 12 == 0x8 && instruction & 0x0F == 0x1 {
            let vx = ((instruction >> 8) & 0x0F) as u8;
            let vy = ((instruction >> 4) & 0x0F) as u8;

            Instruction::OrReg { vx, vy }
        } else if instruction >> 12 == 0x8 && instruction & 0x0F == 0x2 {
            let vx = ((instruction >> 8) & 0x0F) as u8;
            let vy = ((instruction >> 4) & 0x0F) as u8;

            Instruction::AndReg { vx, vy }
        } else if instruction >> 12 == 0x8 && instruction & 0x0F == 0x3 {
            let vx = ((instruction >> 8) & 0x0F) as u8;
            let vy = ((instruction >> 4) & 0x0F) as u8;

            Instruction::XorReg { vx, vy }
        } else if instruction >> 12 == 0x8 && instruction & 0x0F == 0x4 {
            let vx = ((instruction >> 8) & 0x0F) as u8;
            let vy = ((instruction >> 4) & 0x0F) as u8;

            Instruction::AddRegCarry { vx, vy }
        } else if instruction >> 12 == 0x8 && instruction & 0x0F == 0x5 {
            let vx = ((instruction >> 8) & 0x0F) as u8;
            let vy = ((instruction >> 4) & 0x0F) as u8;

            Instruction::SubReg { vx, vy }
        } else if instruction >> 12 == 0x8 && instruction & 0x0F == 0x6 {
            let vx = ((instruction >> 8) & 0x0F) as u8;

            Instruction::Shr { vx }
        } else if instruction >> 12 == 0x8 && instruction & 0x0F == 0xE {
            let vx = ((instruction >> 8) & 0x0F) as u8;

            Instruction::Shl { vx }
        } else if instruction >> 12 == 0xF && instruction & 0xFF == 0x55 {
            let vx = ((instruction >> 8) & 0x0F) as u8;

            Instruction::Store { vx }
        } else if instruction >> 12 == 0xF && instruction & 0xFF == 0x65 {
            let vx = ((instruction >> 8) & 0x0F) as u8;

            Instruction::Read { vx }
        } else if instruction >> 12 == 0xF && instruction & 0xFF == 0x33 {
            let vx = ((instruction >> 8) & 0x0F) as u8;

            Instruction::StoreBcd { vx }
        } else if instruction >> 12 == 0xF && instruction & 0xFF == 0x07 {
            let vx = ((instruction >> 8) & 0x0F) as u8;

            Instruction::LdRegDt { vx }
        } else if instruction >> 12 == 0xF && instruction & 0xFF == 0x29 {
            let vx = ((instruction >> 8) & 0x0F) as u8;

            Instruction::LdFont { vx }
        } else if instruction >> 12 == 0xF && instruction & 0xFF == 0x15 {
            let vx = ((instruction >> 8) & 0x0F) as u8;

            Instruction::LdDt { vx }
        } else {
            Instruction::Unknown { instruction }
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Halt => self.halted = true,
            Instruction::Cls => {
                self.frame_buffer = [[false; 64]; 32];

                if let Some(ref draw_callback) = self.draw_callback {
                    draw_callback(&self.frame_buffer);
                }
            }
            Instruction::Ret => {
                assert!(self.cpu.sp > 0);
                self.cpu.sp = self.cpu.sp.saturating_sub(1);
                self.cpu.pc = self.cpu.stack[self.cpu.sp as usize];
            }
            Instruction::Call { addr } => {
                self.cpu.stack[self.cpu.sp as usize] = self.cpu.pc;
                self.cpu.sp += 1;
                self.cpu.pc = addr;
            }
            Instruction::Jp { addr } => self.cpu.pc = addr,
            Instruction::LdI { addr } => self.cpu.i = addr,
            Instruction::LdByte { vx, byte } => self.cpu.v[vx as usize] = byte,
            Instruction::AddByte { vx, byte } => {
                self.cpu.v[vx as usize] = self.cpu.v[vx as usize].wrapping_add(byte)
            }
            Instruction::Drw { vx, vy, nibble } => {
                let mut sprite_buffer = Vec::with_capacity(nibble as usize);

                for offset in 0..nibble {
                    let addr = self.cpu.i + offset as u16;
                    sprite_buffer.push(self.mem.get_byte(addr));
                }

                let x = self.cpu.v[vx as usize];
                let y = self.cpu.v[vy as usize];

                for (cy, byte) in sprite_buffer
                    .iter()
                    .enumerate()
                    .map(|(i, b)| (y as usize + i, b))
                {
                    for bit_index in 0..8 {
                        let fx = ((x + bit_index) % 64) as usize;
                        let fy = (cy) % 32;

                        self.frame_buffer[fy][fx] ^= ((byte >> (7 - bit_index)) & 1) == 1;
                    }
                }

                if let Some(ref draw_callback) = self.draw_callback {
                    draw_callback(&self.frame_buffer);
                }
            }
            Instruction::SeByte { vx, byte } => {
                if self.cpu.v[vx as usize] == byte {
                    assert!(self.cpu.pc < 4094);
                    self.cpu.pc += 2;
                }
            }
            Instruction::SeReg { vx, vy } => {
                if self.cpu.v[vx as usize] == self.cpu.v[vy as usize] {
                    assert!(self.cpu.pc < 4094);
                    self.cpu.pc += 2;
                }
            }
            Instruction::SneByte { vx, byte } => {
                if self.cpu.v[vx as usize] != byte {
                    assert!(self.cpu.pc < 4094);
                    self.cpu.pc += 2;
                }
            }
            Instruction::SneReg { vx, vy } => {
                if self.cpu.v[vx as usize] != self.cpu.v[vy as usize] {
                    assert!(self.cpu.pc < 4094);
                    self.cpu.pc += 2;
                }
            }
            Instruction::LdReg { vx, vy } => self.cpu.v[vx as usize] = self.cpu.v[vy as usize],
            Instruction::OrReg { vx, vy } => self.cpu.v[vx as usize] |= self.cpu.v[vy as usize],
            Instruction::AndReg { vx, vy } => self.cpu.v[vx as usize] &= self.cpu.v[vy as usize],
            Instruction::XorReg { vx, vy } => self.cpu.v[vx as usize] ^= self.cpu.v[vy as usize],
            Instruction::AddRegCarry { vx, vy } => {
                let (result, overflow) =
                    self.cpu.v[vx as usize].overflowing_add(self.cpu.v[vy as usize]);

                if overflow {
                    self.cpu.v[0xF] = 1;
                } else {
                    self.cpu.v[0xF] = 0;
                }

                self.cpu.v[vx as usize] = result;
            }
            Instruction::SubReg { vx, vy } => {
                let (result, borrow) =
                    self.cpu.v[vx as usize].overflowing_sub(self.cpu.v[vy as usize]);

                if !borrow {
                    self.cpu.v[0xF] = 1;
                } else {
                    self.cpu.v[0xF] = 0;
                }

                self.cpu.v[vx as usize] = result;
            }
            Instruction::Shr { vx } => {
                let vx_val = self.cpu.v[vx as usize];

                if vx_val & 0x1 == 0x1 {
                    self.cpu.v[0xF] = 1
                } else {
                    self.cpu.v[0xF] = 0
                }

                self.cpu.v[vx as usize] = vx_val >> 1;
            }
            Instruction::Shl { vx } => {
                let vx_val = self.cpu.v[vx as usize];

                if vx_val & 0x80 == 0x80 {
                    self.cpu.v[0xF] = 1
                } else {
                    self.cpu.v[0xF] = 0
                }

                self.cpu.v[vx as usize] = vx_val << 1;
            }
            Instruction::Store { vx } => {
                let i = self.cpu.i;

                for offset in 0..=vx {
                    self.mem.set(i + offset as u16, self.cpu.v[offset as usize]);
                }
            }
            Instruction::Read { vx } => {
                let i = self.cpu.i;

                for offset in 0..=vx {
                    self.cpu.v[offset as usize] = self.mem.get_byte(i + offset as u16);
                }
            }
            Instruction::StoreBcd { vx } => {
                let mut vx_val = self.cpu.v[vx as usize];

                let hundreds_digit = vx_val / 100;
                vx_val -= hundreds_digit * 100;

                let tens_digit = vx_val / 10;
                vx_val -= tens_digit * 10;

                self.mem.set(self.cpu.i, hundreds_digit);
                self.mem.set(self.cpu.i + 1, tens_digit);
                self.mem.set(self.cpu.i + 2, vx_val);
            }
            Instruction::LdRegDt { vx } => self.cpu.v[vx as usize] = self.cpu.dt,
            Instruction::LdFont { vx } => {
                self.cpu.i = FONT_START_ADDR + self.cpu.v[vx as usize] as u16 * 5
            }
            Instruction::LdDt { vx } => self.cpu.dt = self.cpu.v[vx as usize],
            Instruction::Unknown { instruction } => {
                println!("unknown instruction: 0x{instruction:X}");
            }
        }
    }
}

struct Memory([u8; 4096]);

impl Memory {
    fn new() -> Memory {
        let mut mem = Memory([0u8; 4096]);
        mem.load_font();
        mem
    }

    fn get_byte(&self, addr: u16) -> u8 {
        assert!(addr < 4096);
        self.0[addr as usize]
    }

    fn set(&mut self, addr: u16, byte: u8) {
        assert!(addr < 4096);
        self.0[addr as usize] = byte;
    }

    fn load_rom(&mut self, buffer: &[u8]) {
        let start_addr: usize = 0x200;
        for (i, &byte) in buffer.iter().enumerate().take(3584) {
            self.0[start_addr + i] = byte;
        }
    }

    fn load_font(&mut self) {
        for (offset, &byte) in FONT_BIN.iter().enumerate() {
            self.set(offset as u16 + FONT_START_ADDR, byte);
        }
    }
}

struct Cpu {
    v: [u8; 16],
    i: u16,
    pc: u16,
    sp: u8,
    dt: u8,
    stack: [u16; 16],
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            v: [0u8; 16],
            stack: [0u16; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            dt: 0,
        }
    }
}

const FONT_START_ADDR: u16 = 0x50;

#[rustfmt::skip]
const FONT_BIN: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0,
    0x20, 0x60, 0x20, 0x20, 0x70,
    0xF0, 0x10, 0xF0, 0x80, 0xF0,
    0xF0, 0x10, 0xF0, 0x10, 0xF0,
    0x90, 0x90, 0xF0, 0x10, 0x10,
    0xF0, 0x80, 0xF0, 0x10, 0xF0,
    0xF0, 0x80, 0xF0, 0x90, 0xF0,
    0xF0, 0x10, 0x20, 0x40, 0x40,
    0xF0, 0x90, 0xF0, 0x90, 0xF0,
    0xF0, 0x90, 0xF0, 0x10, 0xF0,
    0xF0, 0x90, 0xF0, 0x90, 0x90,
    0xE0, 0x90, 0xE0, 0x90, 0xE0,
    0xF0, 0x80, 0x80, 0x80, 0xF0,
    0xE0, 0x90, 0x90, 0x90, 0xE0,
    0xF0, 0x80, 0xF0, 0x80, 0xF0,
    0xF0, 0x80, 0xF0, 0x80, 0x80,
];
