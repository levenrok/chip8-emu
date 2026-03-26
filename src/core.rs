use rand::RngExt;
use wasm_bindgen::prelude::*;

use crate::rom::Rom;

const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[wasm_bindgen]
pub struct Core {
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    opcode: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    key: [u8; 16],
    graphics: [u8; 64 * 32],
    delay_timer: u8,
    sound_timer: u8,
}

#[wasm_bindgen]
impl Core {
    #[wasm_bindgen(constructor)]
    pub fn init(rom: &Rom) -> Self {
        let mut memory = [0u8; 4096];

        let data = rom.data();
        let data_start = 0x0200;

        memory[..FONT_SET.len()].copy_from_slice(&FONT_SET);

        #[cfg(not(target_arch = "wasm32"))]
        {
            let data_len = data.len();
            memory[data_start..(data_start + data_len)].copy_from_slice(&data[..data_len]);
        }

        #[cfg(target_arch = "wasm32")]
        {
            let data_len = data.length() as usize;
            data.copy_to(&mut memory[data_start..(data_start + data_len)]);
        }

        Self {
            memory,
            v: [0; 16],
            i: 0,
            opcode: 0,
            pc: 0x0200,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            graphics: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    #[wasm_bindgen]
    pub fn cycle(&mut self) {
        let rand_byte = rand::rng().random_range(0..=255);

        self.opcode = (self.memory[self.pc as usize] as u16) << 8
            | self.memory[(self.pc + 1) as usize] as u16;
        println!("instruction: {:X}", self.opcode);

        let instruction = (self.opcode & 0xF000) >> 12;
        println!("opcode: {:X}", instruction);

        let nnn = self.opcode & 0x0FFF;
        let x = (self.opcode & 0x0F00) >> 8;
        let y = (self.opcode & 0x00F0) >> 4;
        let kk = self.opcode & 0x00FF;

        match instruction {
            0x0 => {
                if self.opcode == 0x00E0 {
                    for g in self.graphics.iter_mut() {
                        *g = 0;
                    }
                } else if self.opcode == 0x00EE {
                    self.pc = self.stack[self.sp as usize];
                    self.sp -= 1;
                }

                self.increment();
            }
            0x1 => self.pc = nnn,
            0x2 => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = nnn;
            }
            0x3 => {
                if self.v[x as usize] == kk as u8 {
                    self.increment();
                }
                self.increment();
            }
            0x4 => {
                if self.v[x as usize] != kk as u8 {
                    self.increment();
                }
                self.increment();
            }
            0x5 => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.increment();
                }
                self.increment();
            }
            0x6 => {
                self.v[x as usize] = kk as u8;
                self.increment();
            }
            0x7 => {
                self.v[x as usize] = self.v[x as usize].wrapping_add(kk as u8);
                self.increment();
            }
            0xA => {
                self.i = nnn;
                self.increment();
            }
            0xB => {
                self.pc = nnn + self.v[0] as u16;
            }
            0xC => {
                self.v[x as usize] = rand_byte & kk as u8;
                self.increment();
            }
            _ => unreachable!("unknown opcode: {:04X}", self.opcode),
        }
    }

    fn increment(&mut self) {
        self.pc += 2;
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Core {
    #[inline]
    pub fn graphics(&self) -> &[u8] {
        &self.graphics
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Core {
    #[wasm_bindgen(getter)]
    pub fn graphics(&self) -> js_sys::Uint8Array {
        unsafe { js_sys::Uint8Array::view(&self.graphics) }
    }
}
