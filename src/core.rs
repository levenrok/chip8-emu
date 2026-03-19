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

        memory[..FONT_SET.len()].copy_from_slice(&FONT_SET);

        #[cfg(not(target_arch = "wasm32"))]
        {
            let data = rom.data();
            let len = data.len();

            for i in 0..len {
                memory[i + 0x0200] = data[i];
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let data = rom.data();
            let len = data.length() as usize;

            data.copy_to(&mut memory[0x0200..0x0200 + len]);
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
