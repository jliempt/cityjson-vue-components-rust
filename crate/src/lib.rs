#![allow(warnings)]
#[macro_use]

extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate js_sys;
extern crate serde;
extern crate serde_bytes;
extern crate serde_json;
extern crate serde_wasm_bindgen;
extern crate phf;
extern crate serde_query;
extern crate lazy_static;

use lazy_static::lazy_static;
use std::sync::Mutex;

use phf::{phf_map, phf_set};

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

use serde_json::{Value, json};

use std::fmt::Display;

use serde_query::{DeserializeQuery, Query};


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


// Global map with RGB values for all CityObject types
static COLORS: phf::Map<&'static str, &'static [f32; 3]> = phf_map! {

    "Building"                      => &[ 0.45, 0.59, 0.87 ],
    "BuildingPart"                  => &[ 0.45, 0.59, 0.87 ],
    "BuildingInstallation"          => &[ 0.45, 0.59, 0.87 ],
    "Bridge"                        => &[ 0.6, 0.6, 0.6 ],
    "BridgePart"                    => &[ 0.6, 0.6, 0.6 ],
    "BridgeInstallation"            => &[ 0.6, 0.6, 0.6 ],
    "BridgeConstructionElement"     => &[ 0.6, 0.6, 0.6 ],
    "CityObjectGroup"               => &[ 1.0, 1.0, 0.70 ],
    "CityFurniture"                 => &[ 0.8, 0.0, 0.0 ],
    "GenericCityObject"             => &[ 0.8, 0.0, 0.0 ],
    "LandUse"                       => &[ 1.0, 1.0, 0.70 ],
    "PlantCover"                    => &[ 0.22, 0.67, 0.22 ],
    "Railway"                       => &[ 0.0, 0.0, 0.0 ],
    "Road"                          => &[ 0.6, 0.6, 0.6 ],
    "SolitaryVegetationObject"      => &[ 0.22, 0.67, 0.22 ],
    "TINRelief"                     => &[ 1.0, 0.86, 0.6 ],
    "TransportSquare"               => &[ 0.6, 0.6, 0.6 ],
    "Tunnel"                        => &[ 0.6, 0.6, 0.6 ],
    "TunnelPart"                    => &[ 0.6, 0.6, 0.6 ],
    "TunnelInstallation"            => &[ 0.6, 0.6, 0.6 ],
    "WaterBody"                     => &[ 0.3, 0.65, 1.0 ],

};


///// CityJSON processing starts here /////

#[wasm_bindgen]
pub fn receive_buf(buf: &WasmMemBuffer) -> wasm_bindgen::JsValue {

    // let mut arr = js_sys::Array::new_with_length(4);

    // arr.set(0, js_sys::Uint32Array::new());

    log!("Rust: ArrayBuffer received");

    // Take the buffer and deserialize it into a CityJSONAttributes
    let out: CityJSONAttributes = serde_json::from_slice(&buf.buffer).unwrap();

    log!("Rust: CityObjects and vertices parsed");

    // Parse into JsValue to be able to return it to JS
    serde_wasm_bindgen::to_value(&out).unwrap()

}

///// Serde (JSON) streaming code, adapted from https://serde.rs/stream-array.html, https://serde.rs/deserialize-map.html, and https://serde.rs/deserialize-struct.html /////

#[derive(Serialize, Deserialize)]
struct BufferAttributes {

    colors: Vec<f32>,
    triangles: Vec<u32>,
    vertices: Vec<u32>,
    ids: Vec<String>
    
}

#[derive(Serialize, Deserialize)]
struct CityJSONAttributes {

    // Iterate over CityObjects and parse them into BufferAttributes
    #[serde(deserialize_with = "deserialize_cityobjects")]
    #[serde(rename(deserialize = "CityObjects"))]
    attributes: BufferAttributes,

    // Retrieve vertices
    vertices: serde_json::Value,

}

#[derive(Deserialize)]
struct vertices {
    vertices: serde_json::Value
}

fn deserialize_vertices<'de, D>(deserializer: D) -> Result<Vec<u32>, D::Error>
where

    D: Deserializer<'de>,

{

    Ok( Vec::new() )

}

/// Deserialize the CityObjects into vectors that can be used for Three.js BufferAttributes
fn deserialize_cityobjects<'de, D>(deserializer: D) -> Result<BufferAttributes, D::Error>
where

    D: Deserializer<'de>,

{

    struct COVisitor;

    impl<'de> Visitor<'de> for COVisitor
    {
        /// Return type of this visitor
        type Value = BufferAttributes;

        // Error message if data that is not of this type is encountered while deserializing
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a key/value entry")
        }

        // Traverse CityObjects
        fn visit_map<S>(self, mut map: S) -> Result<BufferAttributes, S::Error>
        where
            S: MapAccess<'de>,
        {

            let mut i = 1;

            let mut colors: Vec<f32> = Vec::new();
            let mut triangles: Vec<u32> = Vec::new();
            let mut vertices: Vec<u32> = Vec::new();
            let mut ids: Vec<String> = Vec::new();
        
            let mut ba = BufferAttributes {
                colors: colors,
                triangles: triangles,
                vertices: vertices,
                ids: ids
            };

            while let Some( ( key, value ) ) = map.next_entry::<String, serde_json::Value>()? {

                parse_cityobject( &key, &value, &mut ba );

                if i % 1000 == 0 {
                    log!("{}", i);
                }

                i += 1;

            }

            Ok( ba )


        }
    }

    // Create the visitor and ask the deserializer to drive it. The
    // deserializer will call visitor.visit_map() if a map is present in
    // the input data.

    deserializer.deserialize_map(COVisitor)

}

fn parse_cityobject( id: &String, co: &serde_json::Value, ba: &mut BufferAttributes ) {

    let co_type: &str = co["type"].as_str().unwrap();

    let mut geom = co.get("geometry");

    // Return early if the CityObject has no geometry
    if geom.is_none() || geom.unwrap().as_array().unwrap().len() == 0 {

        return;

    }

    let mut geom = geom.unwrap();

    let geom_n = geom.as_array().unwrap().len();

    for g_i in 0..geom_n {

        let geom_type = &geom[g_i]["type"];

        let boundaries = &geom[g_i]["boundaries"];
        let boundaries_n = boundaries.as_array().unwrap().len();

        
        if geom_type == "Solid" {

            for b_i in 0..boundaries_n {

                parse_shell( &boundaries[b_i], ba, co_type, id );

            }

        }
        else if geom_type == "MultiSurface" || geom_type == "CompositeSurface" {

            parse_shell( &boundaries, ba, co_type, id );

        }
        else if geom_type == "MultiSolid" || geom_type == "CompositeSolid" {

            for b_i in 0..boundaries_n {

                let boundaries_inner_n = boundaries[b_i].as_array().unwrap().len();

                for b_j in 0..boundaries_inner_n {

                    parse_shell( &boundaries[b_i][b_j], ba, co_type, id );

                }

            }

        }

    }

}

fn parse_shell( boundaries: &serde_json::Value, ba: &mut BufferAttributes, co_type: &str, id: &str ){

    let boundaries_n = boundaries.as_array().unwrap().len();

    let color = COLORS.get( co_type ).unwrap();

    for b_i in 0..boundaries_n {

        let boundary_n = boundaries[b_i][0].as_array().unwrap().len();

        // TODO: Investigate how to handle holes. Now I just take [0] from the boundaries.

        if boundary_n == 3 {

            let v0: u32 = boundaries[b_i][0][0].as_i64().unwrap() as u32;
            let v1: u32 = boundaries[b_i][0][1].as_i64().unwrap() as u32;
            let v2: u32 = boundaries[b_i][0][2].as_i64().unwrap() as u32;

            ba.triangles.push( v0 );
            ba.triangles.push( v1 );
            ba.triangles.push( v2 );

            ba.colors.push( color[ 0 ] );
            ba.colors.push( color[ 1 ] );
            ba.colors.push( color[ 2 ] );

            ba.ids.push( id.to_string() )

        }
        
    }

}


#[derive(Serialize, Deserialize)]
struct CityObject {

    // In global variable CO_ID a CityObject ID is stored. The deserialization function will iterate over the CityObjects and find the one.

    #[serde(deserialize_with = "deserialize_single_cityobject")]
    #[serde(rename(deserialize = "CityObjects"))]
    attributes: serde_json::Value,

}

// Similar to fn deserialize_cityobjects()
fn deserialize_single_cityobject<'de, D>(deserializer: D) -> Result<serde_json::Value, D::Error>
where

    D: Deserializer<'de>,

{

    struct COVisitor;

    impl<'de> Visitor<'de> for COVisitor
    {
        /// Return type of this visitor
        type Value = serde_json::Value;

        // Error message if data that is not of this type is encountered while deserializing
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a key/value entry")
        }

        // Traverse CityObjects
        fn visit_map<S>(self, mut map: S) -> Result<serde_json::Value, S::Error>
        where
            S: MapAccess<'de>,
        {

            // Get selected CityObject ID from global variable (since it's seemingly not possible to pass variables to Serde)
            let id = CO_ID.lock().unwrap();

            // Init output
            let mut out = json!({});

            while let Some( ( key, value ) ) = map.next_entry::<String, serde_json::Value>()? {

                if key == *id {

                    out = value;

                    // This would cause an error, why?
                    // break;

                }

            }

            Ok( out )


        }
    }

    deserializer.deserialize_map(COVisitor)

}

///// Retrieving CityObjects from a WasmMemBuffer /////

// Define a global mutable variable to store the ID of the clicked CityObject in.
// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
// It's very ugly, but it doesn't seem possible to pass variables to Serde deserializer.
// Here, it's done with a static value so it's not useful: https://github.com/serde-rs/serde/issues/1059
// serde_query looked nice but also can't take variables: https://docs.rs/serde-query/0.1.3/serde_query/
lazy_static! {
    static ref CO_ID: Mutex<String> = Mutex::new("".to_string());
}


#[wasm_bindgen]
pub fn get_attributes( buf: &WasmMemBuffer, selected_id: String ) -> wasm_bindgen::JsValue {

    // Lock the global variable so that other processes can't access it, and take its value.
    let mut co_id = CO_ID.lock().unwrap();

    // Update it to the selected ID
    *co_id = selected_id;

    // Unlock it
    drop(co_id);

    // Retrieve selected CityObject
    let out: CityObject = serde_json::from_slice(&buf.buffer).unwrap();

    JsValue::from_serde(&out.attributes).unwrap()

}
