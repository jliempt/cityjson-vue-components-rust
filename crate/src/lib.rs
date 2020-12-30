#![allow(warnings)]
#[macro_use]

extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate js_sys;
extern crate serde;
extern crate serde_bytes;
extern crate serde_json;

use wasm_bindgen::prelude::*;
//use web_sys::Blob;

use js_sys::{ ArrayBuffer, Uint8Array };
// use js_sys::JsString;

use serde::{Serialize, Deserialize, Deserializer};

use serde::de::{self, Visitor, SeqAccess, MapAccess};

use std::{cmp, fmt};
use std::marker::PhantomData;

use std::collections::HashMap;
use std::io;

use serde_json::Value;

use std::fmt::Display;


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


///// CityJSON processing starts here /////

#[wasm_bindgen]
pub fn receive_buf(buf: ArrayBuffer) {

    // Convert JsValue::ArrayBuffer to vector
    let bufVec: Vec<u8> = Uint8Array::new_with_byte_offset_and_length(
        &buf,
        0,
        buf.byte_length()
    ).to_vec();

    // Convert vector into ByteBuf for Serde
    let bufSerde = serde_bytes::ByteBuf::from(bufVec);

    // Serialize the CityJSON into vectors for Three.js BufferAttributes
    let out: Outer = serde_json::from_slice(&bufSerde).unwrap();

}

// TODO: triangulation checker? (immediately count amount of triangles and vertices)

///// Serde (JSON) streaming code, adapted from https://serde.rs/stream-array.html and https://serde.rs/deserialize-map.html /////

#[derive(Deserialize)]
struct Outer {

    // Deserialize this field with this function, and specifify the key of the CityJSON data that needs to be deserialized

    #[serde(deserialize_with = "deserialize_cityobjects")]
    #[serde(rename(deserialize = "CityObjects"))]
    max_value: HashMap<String, serde_json::Value>,

    // triangles:
    // colors:
}

/// Deserialize the CityObjects into vectors that can be used for Three.js BufferAttributes
fn deserialize_cityobjects<'de, K: Display, V: Display, D>(deserializer: D) -> Result<HashMap<K, V>, D::Error>
where

    K: Deserialize<'de>,
    V: Deserialize<'de>,
    D: Deserializer<'de>,

{

    struct COVisitor<K: Display, V: Display>(PhantomData<fn() -> K>, PhantomData<fn() -> V>);

    impl<'de, K: Display, V: Display> Visitor<'de> for COVisitor<K, V>
    where
        K: Deserialize<'de>,
        V: Deserialize<'de>,
    {
        /// Return type of this visitor (a HashMap with keys and values)
        type Value = HashMap<K, V>;

        // Error message if data that is not of this type is encountered while deserializing
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a nonempty sequence of numbers")
        }

        // Traverse CityObjects
        fn visit_map<S>(self, mut seq: S) -> Result<HashMap<K, V>, S::Error>
        where
            S: MapAccess<'de>,
        {

            while let Some((K,V)) = seq.next_entry::<K,V>()? {

                log!("{}", K);
                log!("{}", V);

            }

            Ok( HashMap::new() )

        }
    }

    // Create the visitor and ask the deserializer to drive it. The
    // deserializer will call visitor.visit_map() if a map is present in
    // the input data.
    let visitor = COVisitor(PhantomData, PhantomData);
    deserializer.deserialize_map(visitor)

}