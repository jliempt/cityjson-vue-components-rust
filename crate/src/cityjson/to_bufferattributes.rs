use phf::{phf_map, phf_set};
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize, Deserializer};
use serde::de::{self, Visitor, MapAccess, DeserializeSeed, SeqAccess};
use serde_json::{Value, json};
use std::fmt;
use std::marker::PhantomData;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::ops::{Index, IndexMut};
use std::collections::HashMap;
use super::{WasmMemBuffer};

lazy_static! {
    pub static ref IDS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

lazy_static! {
    pub static ref INTERVALS: Mutex<Vec<u32>> = Mutex::new(vec![0]);
}


#[wasm_bindgen]
pub fn receive_buf(buf: &WasmMemBuffer) -> wasm_bindgen::JsValue {

    log!("Rust: ArrayBuffer received");

    // Take the buffer and deserialize it into a CityJSONAttributes
    let mut res: CityJSONAttributes = serde_json::from_slice(&buf.buffer).expect("Error parsing CityJSON buffer");

    log!("Rust: CityObjects parsed");

    // Parse into JsValue to be able to return it to JS
    serde_wasm_bindgen::to_value( &res ).expect("Could not convert serde_json::Value into JsValue")

}

#[wasm_bindgen]
pub fn get_vertices(buf: &WasmMemBuffer) -> wasm_bindgen::JsValue {

    log!("Rust: getting vertices");

    let vertices: Vertices = serde_json::from_slice(&buf.buffer).expect("Error parsing CityJSON buffer");
    
    log!("Rust: vertices parsed");
    
    serde_wasm_bindgen::to_value(&vertices).expect("Could not convert serde_json::Value into JsValue")

}

///// Serde (JSON) streaming code, adapted from https://serde.rs/stream-array.html, https://serde.rs/deserialize-map.html, and https://serde.rs/deserialize-struct.html /////

// Default enables easy initialization (with CityObjects { ..Default::default() }; )
#[derive(Serialize, Deserialize, Default)]
struct CityObjects {

    Building: Vec<u32>,
    BuildingPart: Vec<u32>,
    BuildingInstallation: Vec<u32>,
    Bridge: Vec<u32>,
    BridgePart: Vec<u32>,
    BridgeInstallation: Vec<u32>,
    BridgeConstructionElement: Vec<u32>,
    CityObjectGroup: Vec<u32>,
    CityFurniture: Vec<u32>,
    GenericCityObject: Vec<u32>,
    LandUse: Vec<u32>,
    PlantCover: Vec<u32>,
    Railway: Vec<u32>,
    Road: Vec<u32>,
    SolitaryVegetationObject: Vec<u32>,
    TINRelief: Vec<u32>,
    TransportSquare: Vec<u32>,
    Tunnel: Vec<u32>,
    TunnelPart: Vec<u32>,
    TunnelInstallation: Vec<u32>,
    WaterBody: Vec<u32>,

}

#[derive(Serialize, Deserialize, Default)]
struct CityObjectsIDs {

    Building: Vec<String>,
    BuildingPart: Vec<String>,
    BuildingInstallation: Vec<String>,
    Bridge: Vec<String>,
    BridgePart: Vec<String>,
    BridgeInstallation: Vec<String>,
    BridgeConstructionElement: Vec<String>,
    CityObjectGroup: Vec<String>,
    CityFurniture: Vec<String>,
    GenericCityObject: Vec<String>,
    LandUse: Vec<String>,
    PlantCover: Vec<String>,
    Railway: Vec<String>,
    Road: Vec<String>,
    SolitaryVegetationObject: Vec<String>,
    TINRelief: Vec<String>,
    TransportSquare: Vec<String>,
    Tunnel: Vec<String>,
    TunnelPart: Vec<String>,
    TunnelInstallation: Vec<String>,
    WaterBody: Vec<String>,

}


impl Index<&'_ str> for CityObjects {
    type Output = Vec<u32>;
    fn index(&self, s: &str) -> &Vec<u32> {
        match s {
            "Building" => &self.Building,
            "BuildingPart" => &self.BuildingPart,
            "BuildingInstallation" => &self.BuildingInstallation,
            "Bridge" => &self.Bridge,
            "BridgePart" => &self.BridgePart,
            "BridgeInstallation" => &self.BridgeInstallation,
            "BridgeConstructionElement" => &self.BridgeConstructionElement,
            "CityObjectGroup" => &self.CityObjectGroup,
            "CityFurniture" => &self.CityFurniture,
            "GenericCityObject" => &self.GenericCityObject,
            "LandUse" => &self.LandUse,
            "PlantCover" => &self.PlantCover,
            "Railway" => &self.Railway,
            "Road" => &self.Road,
            "SolitaryVegetationObject" => &self.SolitaryVegetationObject,
            "TINRelief" => &self.TINRelief,
            "TransportSquare" => &self.TransportSquare,
            "Tunnel" => &self.Tunnel,
            "TunnelPart" => &self.TunnelPart,
            "TunnelInstallation" => &self.TunnelInstallation,
            "WaterBody" => &self.WaterBody,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl IndexMut<&'_ str> for CityObjects {

    fn index_mut(&mut self, s: &str) -> &mut Vec<u32> {
        match s {
            "Building" => &mut self.Building,
            "BuildingPart" => &mut self.BuildingPart,
            "BuildingInstallation" => &mut self.BuildingInstallation,
            "Bridge" => &mut self.Bridge,
            "BridgePart" => &mut self.BridgePart,
            "BridgeInstallation" => &mut self.BridgeInstallation,
            "BridgeConstructionElement" => &mut self.BridgeConstructionElement,
            "CityObjectGroup" => &mut self.CityObjectGroup,
            "CityFurniture" => &mut self.CityFurniture,
            "GenericCityObject" => &mut self.GenericCityObject,
            "LandUse" => &mut self.LandUse,
            "PlantCover" => &mut self.PlantCover,
            "Railway" => &mut self.Railway,
            "Road" => &mut self.Road,
            "SolitaryVegetationObject" => &mut self.SolitaryVegetationObject,
            "TINRelief" => &mut self.TINRelief,
            "TransportSquare" => &mut self.TransportSquare,
            "Tunnel" => &mut self.Tunnel,
            "TunnelPart" => &mut self.TunnelPart,
            "TunnelInstallation" => &mut self.TunnelInstallation,
            "WaterBody" => &mut self.WaterBody,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl Index<&'_ str> for CityObjectsIDs {
    type Output = Vec<String>;
    fn index(&self, s: &str) -> &Vec<String> {
        match s {
            "Building" => &self.Building,
            "BuildingPart" => &self.BuildingPart,
            "BuildingInstallation" => &self.BuildingInstallation,
            "Bridge" => &self.Bridge,
            "BridgePart" => &self.BridgePart,
            "BridgeInstallation" => &self.BridgeInstallation,
            "BridgeConstructionElement" => &self.BridgeConstructionElement,
            "CityObjectGroup" => &self.CityObjectGroup,
            "CityFurniture" => &self.CityFurniture,
            "GenericCityObject" => &self.GenericCityObject,
            "LandUse" => &self.LandUse,
            "PlantCover" => &self.PlantCover,
            "Railway" => &self.Railway,
            "Road" => &self.Road,
            "SolitaryVegetationObject" => &self.SolitaryVegetationObject,
            "TINRelief" => &self.TINRelief,
            "TransportSquare" => &self.TransportSquare,
            "Tunnel" => &self.Tunnel,
            "TunnelPart" => &self.TunnelPart,
            "TunnelInstallation" => &self.TunnelInstallation,
            "WaterBody" => &self.WaterBody,
            _ => panic!("unknown field: {}", s),
        }
    }
}

impl IndexMut<&'_ str> for CityObjectsIDs {

    fn index_mut(&mut self, s: &str) -> &mut Vec<String> {
        match s {
            "Building" => &mut self.Building,
            "BuildingPart" => &mut self.BuildingPart,
            "BuildingInstallation" => &mut self.BuildingInstallation,
            "Bridge" => &mut self.Bridge,
            "BridgePart" => &mut self.BridgePart,
            "BridgeInstallation" => &mut self.BridgeInstallation,
            "BridgeConstructionElement" => &mut self.BridgeConstructionElement,
            "CityObjectGroup" => &mut self.CityObjectGroup,
            "CityFurniture" => &mut self.CityFurniture,
            "GenericCityObject" => &mut self.GenericCityObject,
            "LandUse" => &mut self.LandUse,
            "PlantCover" => &mut self.PlantCover,
            "Railway" => &mut self.Railway,
            "Road" => &mut self.Road,
            "SolitaryVegetationObject" => &mut self.SolitaryVegetationObject,
            "TINRelief" => &mut self.TINRelief,
            "TransportSquare" => &mut self.TransportSquare,
            "Tunnel" => &mut self.Tunnel,
            "TunnelPart" => &mut self.TunnelPart,
            "TunnelInstallation" => &mut self.TunnelInstallation,
            "WaterBody" => &mut self.WaterBody,
            _ => panic!("unknown field: {}", s),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
struct TrianglesAndGroups {

    triangles: Vec<u32>,
    groups: HashMap<String, Vec<u32>>,

}

#[derive(Serialize, Deserialize)]
struct CityJSONAttributes {

    // Iterate over CityObjects and parse them into BufferAttributes
    #[serde(deserialize_with = "deserialize_cityobjects")]
    #[serde(rename(deserialize = "CityObjects"))]
    triangles: TrianglesAndGroups

}

/// Deserialize the CityObjects into vectors that can be used for Three.js BufferAttributes
fn deserialize_cityobjects<'de, D>(deserializer: D) -> Result<TrianglesAndGroups, D::Error>
where

    D: Deserializer<'de>,

{

    struct COVisitor;

    impl<'de> Visitor<'de> for COVisitor
    {
        /// Return type of this visitor
        type Value = TrianglesAndGroups;

        // Error message if data that is not of this type is encountered while deserializing
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a key/value entry")
        }

        // Traverse CityObjects
        fn visit_map<S>(self, mut map: S) -> Result<TrianglesAndGroups, S::Error>
        where
            S: MapAccess<'de>,
        {

            let mut i = 1;
        
            let mut triangle_groups = CityObjects { ..Default::default() };
            let mut interval_groups = CityObjects { ..Default::default() };
            let mut id_groups = CityObjectsIDs { ..Default::default() };

            let co_types = ["Building", "BuildingPart", "BuildingInstallation", "Bridge", "BridgePart", "BridgeInstallation", "BridgeConstructionElement", "CityObjectGroup", "CityFurniture", "GenericCityObject", "LandUse", "PlantCover", "Railway", "Road", "SolitaryVegetationObject", "TINRelief", "TransportSquare", "Tunnel", "TunnelPart", "TunnelInstallation", "WaterBody"];

            while let Some( ( key, value ) ) = map.next_entry::<String, serde_json::Value>()? {

                parse_cityobject( &key, &value, &mut triangle_groups );

                let co_type: &str = value[ "type" ].as_str().expect( "CityObject has no valid type" );

                let triangles_len = triangle_groups[ co_type ].len() as u32;

                // What happens if a CityObject did not have geometry? Maybe do something like this (but check if .last() == None)
                // if *interval_groups[ co_type ].last().unwrap() != triangles_len

                interval_groups[ co_type ].push( triangles_len / 3 );
                
                id_groups[ co_type ].push( key.to_string() );

                if i % 1000 == 0 {
                    log!("{} CityObjects parsed", i);
                }

                i += 1;

            }

            log!("COs done");

            let mut ids = IDS.lock().unwrap();
            let mut intervals = INTERVALS.lock().unwrap();

            let mut triangles_n = 0;

            // Count amount of triangles to be able to init vector with_capacity(n)
            for co_type in &co_types {

                triangles_n += triangle_groups[ &co_type.to_string() ].len();

            }

            let mut res = TrianglesAndGroups { triangles: Vec::with_capacity(triangles_n),
                                                groups: HashMap::new() };
            
            let triangles = &mut res.triangles;
            let groups = &mut res.groups;

            for co_type in &co_types {

                log!("{}", co_type);

                if triangle_groups[ &co_type.to_string() ].len() > 0 {

                    let start = triangles.len();

                    triangles.append( &mut triangle_groups[ &co_type.to_string() ] );

                    let count = triangles.len() - start;

                    groups.insert( co_type.to_string(), vec!(start as u32, count as u32) );

                    ids.append( &mut id_groups[ co_type ] );

                    // let group_intervals = *interval_groups[ co_type ].into_iter().map( |x| x + start as u32 ).collect();

                    interval_groups[ co_type ].iter_mut().for_each(|x| *x += (start as u32) );

                    intervals.append( &mut interval_groups[ co_type ] );

                }

            };

            log!("COs done (2)");

            //let res = json!( { "triangles": &triangles, "groups": &groups } );

            Ok( res )


        }
    }

    // Create the visitor and ask the deserializer to drive it. The
    // deserializer will call visitor.visit_map() if a map is present in
    // the input data.

    deserializer.deserialize_map(COVisitor)

}

fn parse_cityobject( id: &String, co: &serde_json::Value, triangles: &mut CityObjects ) {

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

                parse_shell( &boundaries[b_i], triangles, co_type, id );

            }

        }
        else if geom_type == "MultiSurface" || geom_type == "CompositeSurface" {

            parse_shell( &boundaries, triangles, co_type, id );

        }
        else if geom_type == "MultiSolid" || geom_type == "CompositeSolid" {

            for b_i in 0..boundaries_n {

                let boundaries_inner_n = boundaries[b_i].as_array().expect("CityObject something wrong with \"boundaries\"").len();

                for b_j in 0..boundaries_inner_n {

                    parse_shell( &boundaries[b_i][b_j], triangles, co_type, id );

                }

            }

        }

    }

}

fn parse_shell( boundaries: &serde_json::Value, triangles: &mut CityObjects, co_type: &str, id: &str ){

    let boundaries_n = boundaries.as_array().unwrap().len();

    for b_i in 0..boundaries_n {

        let boundary_n = boundaries[b_i][0].as_array().expect("CityObject something wrong with \"boundaries\"").len();

        // TODO: Investigate how to handle holes. Now I just take [0] from the boundaries.

        if boundary_n == 3 {

            let vs = [  boundaries[b_i][0][0].as_i64().unwrap() as u32,
                        boundaries[b_i][0][1].as_i64().unwrap() as u32,
                        boundaries[b_i][0][2].as_i64().unwrap() as u32 ];

            triangles[ co_type ].push( vs[ 0 ] );
            triangles[ co_type ].push( vs[ 1 ] );
            triangles[ co_type ].push( vs[ 2 ] );

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