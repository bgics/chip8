use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

use crate::error::{Chip8Error, Result};
use crate::instruction::Instruction;
use crate::memory::{FONT_START_ADDR, Memory, ROM_START_ADDR};
use crate::{FrameBuffer, KeyMatrix, Message};

pub struct Cpu {
    v: [u8; 16],
    i: u16,
    pc: u16,
    sp: u8,
    dt: u8,
    st: u8,
    stack: [u16; 16],
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            v: [0u8; 16],
            stack: [0u16; 16],
            i: 0,
            pc: ROM_START_ADDR,
            sp: 0,
            dt: 0,
            st: 0,
        }
    }

    pub fn tick_60hz(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            self.st -= 1;
        }
    }

    pub fn tick(
        &mut self,
        memory: &mut Memory,
        frame_buffer: Arc<Mutex<FrameBuffer>>,
        key_matrix: Arc<Mutex<KeyMatrix>>,
        sender: Sender<Message>,
        receiver: &Receiver<Message>,
    ) -> Result<()> {
        let instruction = self.get_next_instruction(memory)?;
        self.execute(
            instruction,
            memory,
            frame_buffer,
            key_matrix,
            sender,
            receiver,
        )
    }

    fn execute(
        &mut self,
        instruction: Instruction,
        memory: &mut Memory,
        frame_buffer: Arc<Mutex<FrameBuffer>>,
        key_matrix: Arc<Mutex<KeyMatrix>>,
        sender: Sender<Message>,
        receiver: &Receiver<Message>,
    ) -> Result<()> {
        match instruction {
            Instruction::Cls => {
                {
                    let mut frame_buffer = frame_buffer.lock().unwrap();
                    *frame_buffer = FrameBuffer::new();
                }

                let _ = sender.send(Message::Draw);
            }
            Instruction::Ret => {
                if self.sp == 0 {
                    return Err(Chip8Error::StackUnderflow);
                }
                self.sp -= 1;

                match self.stack.get(self.sp as usize) {
                    Some(value) => self.pc = *value,
                    None => return Err(Chip8Error::StackOverflow),
                }
            }
            Instruction::Call { addr } => {
                match self.stack.get_mut(self.sp as usize) {
                    Some(value) => *value = self.pc,
                    None => return Err(Chip8Error::StackOverflow),
                }
                self.sp += 1;
                self.pc = addr;
            }
            Instruction::Jp { addr } => self.pc = addr,
            Instruction::JpV0 { addr } => {
                let v0_val = self.v[0];
                self.pc = addr + v0_val as u16;
            }
            Instruction::LdI { addr } => self.i = addr,
            Instruction::LdByte { vx, byte } => self.v[vx as usize] = byte,
            Instruction::AddByte { vx, byte } => {
                self.v[vx as usize] = self.v[vx as usize].wrapping_add(byte)
            }
            Instruction::Drw { vx, vy, nibble } => {
                let mut sprite_buffer = Vec::with_capacity(nibble as usize);

                for offset in 0..nibble {
                    let addr = self.i + offset as u16;
                    sprite_buffer.push(memory.read(addr)?);
                }

                let x = self.v[vx as usize] % 64;
                let y = self.v[vy as usize] % 32;

                self.v[0xF] = 0;

                for cy in y..(y + nibble).min(32) {
                    for cx in x..(x + 8).min(64) {
                        let byte_index = cy - y;
                        let bit_offset = cx - x;

                        let turned_off = frame_buffer.lock().unwrap().xor(
                            cx as usize,
                            cy as usize,
                            ((sprite_buffer[byte_index as usize] >> (7 - bit_offset)) & 1) == 1,
                        );

                        if turned_off {
                            self.v[0xF] = 1;
                        }
                    }
                }

                let _ = sender.send(Message::Draw);
            }
            Instruction::SeByte { vx, byte } => {
                if self.v[vx as usize] == byte {
                    self.pc += 2;
                }
            }
            Instruction::SeReg { vx, vy } => {
                if self.v[vx as usize] == self.v[vy as usize] {
                    self.pc += 2;
                }
            }
            Instruction::SneByte { vx, byte } => {
                if self.v[vx as usize] != byte {
                    self.pc += 2;
                }
            }
            Instruction::SneReg { vx, vy } => {
                if self.v[vx as usize] != self.v[vy as usize] {
                    self.pc += 2;
                }
            }
            Instruction::LdReg { vx, vy } => self.v[vx as usize] = self.v[vy as usize],
            Instruction::OrReg { vx, vy } => self.v[vx as usize] |= self.v[vy as usize],
            Instruction::AndReg { vx, vy } => self.v[vx as usize] &= self.v[vy as usize],
            Instruction::XorReg { vx, vy } => self.v[vx as usize] ^= self.v[vy as usize],
            Instruction::AddRegCarry { vx, vy } => {
                let (result, overflow) = self.v[vx as usize].overflowing_add(self.v[vy as usize]);

                self.v[vx as usize] = result;

                if overflow {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
            }
            Instruction::SubReg { vx, vy } => {
                let (result, borrow) = self.v[vx as usize].overflowing_sub(self.v[vy as usize]);

                self.v[vx as usize] = result;

                if !borrow {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
            }
            Instruction::SubNReg { vx, vy } => {
                let (result, borrow) = self.v[vy as usize].overflowing_sub(self.v[vx as usize]);

                self.v[vx as usize] = result;

                if !borrow {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
            }
            Instruction::Shr { vx } => {
                let vx_val = self.v[vx as usize];

                self.v[vx as usize] = vx_val >> 1;

                if vx_val & 0x1 == 0x1 {
                    self.v[0xF] = 1
                } else {
                    self.v[0xF] = 0
                }
            }
            Instruction::Shl { vx } => {
                let vx_val = self.v[vx as usize];

                self.v[vx as usize] = vx_val << 1;

                if vx_val & 0x80 == 0x80 {
                    self.v[0xF] = 1
                } else {
                    self.v[0xF] = 0
                }
            }
            Instruction::KeyWait { vx } => {
                if let Ok(Message::KeyPressed(val)) = receiver.recv() {
                    self.v[vx as usize] = val
                }
            }
            Instruction::Store { vx } => {
                let i = self.i;

                for offset in 0..=vx {
                    memory.write(i + offset as u16, self.v[offset as usize])?;
                }
            }
            Instruction::Read { vx } => {
                let i = self.i;

                for offset in 0..=vx {
                    self.v[offset as usize] = memory.read(i + offset as u16)?;
                }
            }
            Instruction::StoreBcd { vx } => {
                let mut vx_val = self.v[vx as usize];

                let hundreds_digit = vx_val / 100;
                vx_val -= hundreds_digit * 100;

                let tens_digit = vx_val / 10;
                vx_val -= tens_digit * 10;

                memory.write(self.i, hundreds_digit)?;
                memory.write(self.i + 1, tens_digit)?;
                memory.write(self.i + 2, vx_val)?;
            }
            Instruction::LdRegDt { vx } => self.v[vx as usize] = self.dt,
            Instruction::LdFont { vx } => self.i = FONT_START_ADDR + self.v[vx as usize] as u16 * 5,
            Instruction::LdDt { vx } => self.dt = self.v[vx as usize],
            Instruction::LdSt { vx } => self.st = self.v[vx as usize],
            Instruction::Rnd { vx, byte } => {
                let rand_val = rand::random::<u8>();
                self.v[vx as usize] = rand_val & byte;
            }
            Instruction::Skp { vx } => {
                let vx_val = self.v[vx as usize];
                if key_matrix.lock().unwrap().is_pressed(vx_val as usize) {
                    self.pc += 2
                }
            }
            Instruction::Sknp { vx } => {
                let vx_val = self.v[vx as usize];
                if !key_matrix.lock().unwrap().is_pressed(vx_val as usize) {
                    self.pc += 2
                }
            }
            Instruction::AddI { vx } => self.i += self.v[vx as usize] as u16,
            Instruction::Unknown { instruction } => {
                println!("unknown instruction: 0x{instruction:4X}");
            }
        }
        Ok(())
    }

    fn get_next_instruction(&mut self, memory: &Memory) -> Result<Instruction> {
        let msb = memory.read(self.pc)?;
        let lsb = memory.read(self.pc + 1)?;

        self.pc += 2;

        let opcode = ((msb as u16) << 8) | lsb as u16;

        Ok(opcode.into())
    }
}
