#[macro_use]
extern crate cfg_if;

extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
//use web_sys::Blob;
extern crate js_sys;
use js_sys::{ ArrayBuffer, Uint8Array };
// use js_sys::JsString;
extern crate serde;
use serde::{Serialize, Deserialize, Deserializer};
extern crate serde_bytes;
use serde::de::{self, Visitor, SeqAccess};

use std::{cmp, fmt};
use std::marker::PhantomData;

use std::collections::HashMap;
use std::io;

extern crate bincode;
extern crate serde_json;

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

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn init() -> Result<(), JsValue> {
    // If the `console_error_panic_hook` feature is enabled this will set a panic hook, otherwise
    // it will do nothing.
    set_panic_hook();
    console_error_panic_hook::set_once();

    Ok(())
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {

    log!("add");

    return a + b

}


#[wasm_bindgen]
pub fn receive_blob(buf: ArrayBuffer) -> i32 {

    log!("hoi");

    // Convert JsValue::ArrayBuffer to vector
    let out: Vec<u8> = Uint8Array::new_with_byte_offset_and_length(
        &buf,
        0,
        buf.byte_length()
    ).to_vec();

    let e = serde_bytes::ByteBuf::from(out);

    let out2: Outer = serde_json::from_slice(&e).unwrap();

    log!("{}", out2.max_value);

    10

}


////////////////////////////////////////////
#[derive(Deserialize)]
struct Outer {

    // Deserialize this field by computing the maximum value of a sequence
    // (JSON array) of values.
    #[serde(deserialize_with = "deserialize_max")]
    // Despite the struct field being named `max_value`, it is going to come
    // from a JSON field called `values`.
    #[serde(rename(deserialize = "vertices"))]
    max_value: u64,
}

/// Deserialize the maximum of a sequence of values. The entire sequence
/// is not buffered into memory as it would be if we deserialize to Vec<T>
/// and then compute the maximum later.
///
/// This function is generic over T which can be any type that implements
/// Ord. Above, it is used with T=u64.
fn deserialize_max<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + Ord,
    D: Deserializer<'de>,
{
    struct MaxVisitor<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for MaxVisitor<T>
    where
        T: Deserialize<'de> + Ord,
    {
        /// Return type of this visitor. This visitor computes the max of a
        /// sequence of values of type T, so the type of the maximum is T.
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a nonempty sequence of numbers")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<T, S::Error>
        where
            S: SeqAccess<'de>,
        {
            // Start with max equal to the first value in the seq.
            let mut max = seq.next_element()?.ok_or_else(||
                // Cannot take the maximum of an empty seq.
                de::Error::custom("no values in seq when looking for maximum")
            )?;

            // Update the max while there are additional values.
            while let Some(value) = seq.next_element()? {
                max = cmp::max(max, value);
            }

            Ok(max)
        }
    }

    // Create the visitor and ask the deserializer to drive it. The
    // deserializer will call visitor.visit_seq() if a seq is present in
    // the input data.
    let visitor = MaxVisitor(PhantomData);
    deserializer.deserialize_seq(visitor)
}