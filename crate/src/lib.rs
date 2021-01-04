#[macro_use]

extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate js_sys;
extern crate serde;
extern crate serde_json;
extern crate serde_wasm_bindgen;
extern crate phf;
extern crate lazy_static;

use wasm_bindgen::prelude::*;


///// Boilerplate code, from https://github.com/rustwasm/rust-parcel-template and I think the wasm-pack-template /////

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// Only import module after macros have been defined
mod cityjson;

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn init() -> Result<(), JsValue> {
    // If the `console_error_panic_hook` feature is enabled this will set a panic hook, otherwise
    // it will do nothing.
    set_panic_hook();
    console_error_panic_hook::set_once();

    Ok(())
}



// This allows the creation and direct use of buffers in WASM memory. It avoids having to copy an ArrayBuffer from JS to Rust, which would mean that the data is in memory twice.
// See https://github.com/rustwasm/wasm-bindgen/issues/1079, https://github.com/rustwasm/wasm-bindgen/issues/1643
// Code has been sourced from there.
#[wasm_bindgen]
pub struct WasmMemBuffer {
    buffer: Vec<u8>,
}

#[wasm_bindgen]
impl WasmMemBuffer {
    #[wasm_bindgen(constructor)]
    pub fn new(byte_length: u32, f: &js_sys::Function) -> Self {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.reserve(byte_length as usize);
        unsafe {
            let array =
                js_sys::Uint8Array::view_mut_raw(buffer.as_mut_ptr(),
                                                 byte_length as usize);
            f.call1(&JsValue::NULL, &JsValue::from(array))
                .expect("The callback function should not throw");
            buffer.set_len(byte_length as usize);
        }
        Self { buffer }
    }
}
