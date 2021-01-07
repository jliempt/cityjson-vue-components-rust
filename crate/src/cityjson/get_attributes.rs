use lazy_static::lazy_static;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize, Deserializer};
use serde::de::{self, Visitor, MapAccess};
use serde_json::{Value, json};
use std::fmt;

use super::{WasmMemBuffer};
use super::to_bufferattributes;


#[derive(Serialize, Deserialize)]
struct CityObject {

    // In global variable CO_ID a CityObject ID is stored. The deserialization function will iterate over the CityObjects and find the one.

    #[serde(deserialize_with = "deserialize_single_cityobject")]
    #[serde(rename(deserialize = "CityObjects"))]
    attributes: serde_json::Value,

}

// Define a global mutable variable to store the ID of the clicked CityObject in.
// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
// It's very ugly, but it doesn't seem possible to pass variables to Serde deserializer.
// Here, it's done with a static value so it's not useful: https://github.com/serde-rs/serde/issues/1059
// serde_query looked nice but also can't take variables: https://docs.rs/serde-query/0.1.3/serde_query/
lazy_static! {
    static ref CO_ID: Mutex<String> = Mutex::new("".to_string());
}

// Similar to to_bufferattributes::deserialize_cityobjects()
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

#[wasm_bindgen]
pub fn get_attributes( buf: &WasmMemBuffer, selected_id: String ) -> wasm_bindgen::JsValue {

    log!("Getting attributes of {}...", selected_id);

    // Lock the global variable so that other processes can't access it, and take its value.
    let mut co_id = CO_ID.lock().unwrap();

    // Update it to the selected ID
    *co_id = selected_id;

    // Unlock it
    drop(co_id);

    // Retrieve selected CityObject
    let out: CityObject = serde_json::from_slice(&buf.buffer).expect("Error getting attributes");

    JsValue::from_serde(&out.attributes).expect("Could not convert serde_json::Value into JsValue (attributes)")

}

#[wasm_bindgen]
pub fn get_interval_and_id( query: u32 ) -> wasm_bindgen::JsValue {

    log!("Getting CityObject ID and triangle interval from clicked triangle index...");

    let intervals = &to_bufferattributes::INTERVALS.lock().unwrap();
    let ids = &to_bufferattributes::IDS.lock().unwrap();

    let mut position = intervals.len() / 2;
    let mut step = position / 2;

    loop {
        
        if step == 0 {

            if query >= intervals[ position ] && query < intervals[ position + 1 ] {

                let res = format!("{} {} {}", &ids[ position ], intervals[ position ], intervals[ position + 1 ] - 1 );

                return wasm_bindgen::JsValue::from_str( &res );

            }

            log!("ID corresponding to triangle index not found!");
            return wasm_bindgen::JsValue::from_str("");

        } else if query >= intervals[ position + 1 ] {

            position += step;

        } else if query < intervals[ position ] {

            position -= step;

        } else {

            let res = format!("{} {} {}", &ids[ position ], intervals[ position ], intervals[ position + 1 ] - 1 );

            return wasm_bindgen::JsValue::from_str( &res );

        }

        // Ceil division to make sure step does not get too low, but if it was 1 it needs to be floored, otherwise it will be 1 forever
        if step == 1 {

            step = 0;

        } else {

            step = (step as f32 / 2.0).ceil() as usize;

        }

    }

}