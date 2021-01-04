use phf::{phf_map, phf_set};
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize, Deserializer};
use serde::de::{self, Visitor, MapAccess, DeserializeSeed, SeqAccess};
use serde_json::{Value, json};
use std::fmt;
use std::marker::PhantomData;
use super::{WasmMemBuffer};


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

#[wasm_bindgen]
pub fn receive_buf(buf: &WasmMemBuffer) -> wasm_bindgen::JsValue {

    log!("Rust: ArrayBuffer received");

    // Take the buffer and deserialize it into a CityJSONAttributes
    let mut out: CityJSONAttributes = serde_json::from_slice(&buf.buffer).expect("Error parsing CityJSON buffer");
    // let vertices: Vertices = serde_json::from_slice(&buf.buffer).expect("Error parsing CityJSON buffer");

    // out.attributes.vertices = vertices.vertices;

    log!("Rust: CityObjects and vertices parsed");

    // Parse into JsValue to be able to return it to JS
    serde_wasm_bindgen::to_value(&out).expect("Could not convert serde_json::Value into JsValue")

}

#[wasm_bindgen]
pub fn get_vertices(buf: &WasmMemBuffer) -> wasm_bindgen::JsValue {

    let vertices: Vertices = serde_json::from_slice(&buf.buffer).expect("Error parsing CityJSON buffer");
    
    log!("Vertices parsed");
    
    serde_wasm_bindgen::to_value(&vertices).expect("Could not convert serde_json::Value into JsValue")

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
                    log!("{} CityObjects parsed", i);
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

    let co_type: &str = co["type"].as_str().expect("CityObject has no valid type");

    let mut geom = co.get("geometry");

    // Return early if the CityObject has no geometry
    if geom.is_none() || geom.unwrap().as_array().unwrap().len() == 0 {

        return;

    }

    let mut geom = geom.expect("CityObject does not have \"geometry\"");

    let geom_n = geom.as_array().unwrap().len();

    for g_i in 0..geom_n {

        let geom_type = &geom[g_i]["type"];

        let boundaries = &geom[g_i]["boundaries"];
        let boundaries_n = boundaries.as_array().expect("CityObject does not have \"boundaries\"").len();

        
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

                let boundaries_inner_n = boundaries[b_i].as_array().expect("CityObject something wrong with \"boundaries\"").len();

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

        let boundary_n = boundaries[b_i][0].as_array().expect("CityObject something wrong with \"boundaries\"").len();

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

            // a.ids.push( id.to_string() )

        }
        
    }

}

#[derive(Serialize, Deserialize)]
struct Vertices {
    
    #[serde(deserialize_with = "deserialize_vertices")]
    vertices: Vec<u32>,
    
}


// From https://docs.serde.rs/serde/de/trait.DeserializeSeed.html
fn deserialize_vertices<'de, D>(deserializer: D) -> Result<Vec<u32>, D::Error>
    where

    D: Deserializer<'de>,

{

    // A DeserializeSeed implementation that uses stateful deserialization to
    // append array elements onto the end of an existing vector. The preexisting
    // state ("seed") in this case is the Vec<T>. The `deserialize` method of
    // `ExtendVec` will be traversing the inner arrays of the JSON input and
    // appending each integer into the existing Vec.
    struct ExtendVec<'a, T: 'a>(&'a mut Vec<T>);

    impl<'de, 'a, T> DeserializeSeed<'de> for ExtendVec<'a, T>
    where
        T: Deserialize<'de>,
    {
        // The return type of the `deserialize` method. This implementation
        // appends onto an existing vector but does not create any new data
        // structure, so the return type is ().
        type Value = ();

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            // Visitor implementation that will walk an inner array of the JSON
            // input.
            struct ExtendVecVisitor<'a, T: 'a>(&'a mut Vec<T>);

            impl<'de, 'a, T> Visitor<'de> for ExtendVecVisitor<'a, T>
            where
                T: Deserialize<'de>,
            {
                type Value = ();

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    write!(formatter, "an array of integers")
                }

                fn visit_seq<A>(self, mut seq: A) -> Result<(), A::Error>
                where
                    A: SeqAccess<'de>,
                {
                    // Visit each element in the inner array and push it onto
                    // the existing vector.
                    while let Some(elem) = seq.next_element()? {
                        self.0.push(elem);
                    }
                    Ok(())
                }
            }

            deserializer.deserialize_seq(ExtendVecVisitor(self.0))
        }
    }

    // Visitor implementation that will walk the outer array of the JSON input.
    struct FlattenedVecVisitor<T>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for FlattenedVecVisitor<T>
    where
        T: Deserialize<'de>,
    {
        // This Visitor constructs a single Vec<T> to hold the flattened
        // contents of the inner arrays.
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "an array of arrays")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Vec<T>, A::Error>
        where
            A: SeqAccess<'de>,
        {
            // Create a single Vec to hold the flattened contents.
            let mut vec = Vec::new();

            // Each iteration through this loop is one inner array.
            while let Some(()) = seq.next_element_seed(ExtendVec(&mut vec))? {
                // Nothing to do; inner array has been appended into `vec`.
            }

            // Return the finished vec.
            Ok(vec)
        }
    }

    let visitor = FlattenedVecVisitor(PhantomData);
    let flattened: Vec<u32> = deserializer.deserialize_seq(visitor)?;

    Ok(flattened)

}