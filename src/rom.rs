use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Rom {
    name: String,
    data: Vec<u8>,
    pub size: usize,
}

#[wasm_bindgen]
impl Rom {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, data: Vec<u8>, size: usize) -> Self {
        Self { name, data, size }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Rom {
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Rom {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> JsValue {
        JsValue::from_str(&self.name)
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> js_sys::Uint8Array {
        unsafe { js_sys::Uint8Array::view(&self.data) }
    }
}
