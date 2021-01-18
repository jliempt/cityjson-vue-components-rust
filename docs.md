There are four optimisations that can be implemented individually:
1. Parsing the CityJSON geometries into an indexed BufferGeometry with groups
1. Being able to assume that a file is triangulated
1. Reading the JSON in a stream rather than fully loading it into memory
1. Doing the CityJSON -> Three.js mesh parsing in Rust


1. BufferGeometry  
In the current version of cityjson-vue-components, every CityObject is parsed into an individual Mesh with a Geometry. Now, they are parsed into one indexed (meaning that vertices are reused) BufferGeometry, divided into Groups per CityObject type (stored as the index of the first triangle that belongs to it and the amount of triangles that it consists of).  
Having one geometry greatly improves the visualisation performance (and I think also helps with memory). Dividing the geometry into groups is mainly done so that it's possible to set one color per group - without groups you get color bleeding (when using an indexed BufferGeometry, because colors are stored per vertex rather than per triangle) or you have to use a non-indexed BufferGeometry and store a color for every vertex, which is highly memory-inefficient.

1. Triangulation assumption  
Performance (both speed and memory) benefits a lot from being able to assume that a CityJSON file is triangulated. It means that the vertices can stay as they are (besides having to be flattened), since there won't be new vertices created by triangulation on the fly. This is fast, and it also makes it unnecessary to load the vertices into memory while parsing the CityObjects.  
It is already proposed to add a "triangulated" flag to CityJSON v1.1.

1. JSON stream  
Loading JSON data into memory is very expensive compared to just having a binary data buffer (ArrayBuffer in JS). It is however possible to read the binary data as a stream, and thus traverse JSON data without having it in memory. In the optimisation, CityObjects are read and parsed one-by-one. This is probably slower, but very memory-efficient.  
In JS it can be done with JSONStream or Oboe.js. I tried the former, maybe the latter is nicer.
In Rust, it is done with Serde.

1. Rust  
Parcel.js is used as a bundler, because it makes it very easy to combine JS/Vue with Rust/WASM. It enables importing Rust files and crates into JS in the same way as modules are normally imported. Parcel handles the compilation of Rust into WASM.  
wasm-bindgen is used for its functionalities for interoperability between JS and Rust. It helps with converting JS and Rust data types into each other, so that they can be shared.    
It currently works as follows:  
 * It currently only supports fully triangulated CityJSON files (non-triangulated geometries are skipped).
 1. The JSON file is loaded as an ArrayBuffer in JS.
 1. The buffer is stored in WASM memory, and JS keeps a pointer to it. If you simply pass data from JS to WASM, it will be duplicated I believe. So in this way, it is only in memory once. In hindsight we probably need to keep the CityJSON ArrayBuffer in JS anyway, so maybe this overcomplicates things.
 1. First, the CityObjects are parsed by calling the Rust function for it. It returns an array of triangles and an array of geometry groups.
Serde reads over "CityObjects" in a stream and takes stores all triangles in a vector per CityObject type. At the end, these are merged together, while storing the needed information for the geometry groups (start index and amount of triangles). The triangles and the groups are returned to JS, which are to be used for the creation of a BufferGeometry.  
In the same process in a similar way (first in groups, then merged), for every CityObject the indices of the triangles that belong to it are stored in a vector (in intervals: start and end index), along with an aligned vector in which the IDs are stored. These are used for being able to click on CityObjects. Three.js raycaster returns the index of the triangle that is clicked. With a binary search, the interval to which the triangle index belongs is found. And with the index of that interval, the corresponding CityObject ID can be retrieved. These two vectors are currently (globally) stored in Rust/WASM, and the binary search is also implemented there. But it's likely better to just keep this in JS.
 1. Secondly, the vertices are retrieved and flattened by calling the Rust function for it. Because we assume triangulation, the vertices stay unchanged.
 1. The reason to do this separately is that wasm-bindgen seems to only support wasm-unknown-unknown for compilation of Rust to WASM, which is currently still limited to 2GB of memory use, contrasting to browser's 4GB per tab. I think this will be solved in the future, looks like with Emscripten you can already use 4GB. Anyway, by separating the previous two steps, you split the memory use of these two tasks.

Future work and ideas:
* Integration with ninja
* CO coloring after picking
* Implement triangulation in Rust
* Better/faster attributes retrieval in Rust (or do it in JS)
* Look into using 4GB memory (in WASM) by compiling WASM with Emscripten
