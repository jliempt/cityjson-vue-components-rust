<template>
  <div
    id="viewer"
    class="col-12 px-0 h-100"
  />
</template>

<script>
import $ from 'jquery';
import * as THREE from 'three';
import OrbitControls from 'three-orbitcontrols';
// Needed for async/await functionality (I think parcel needs it)
import 'regenerator-runtime/runtime';
// Import Rust functions
import rust from '../crate/Cargo.toml';
rust.init();

const filePath = "45bz1.json"

export default {
	name: 'ThreeJsViewer',
	props: {
		citymodel: Object,
		selected_objid: String,
		object_colors: {
			type: Object,
			default: function () {

			return {
					"Building": 0x7497df,
					"BuildingPart": 0x7497df,
					"BuildingInstallation": 0x7497df,
					"Bridge": 0x999999,
					"BridgePart": 0x999999,
					"BridgeInstallation": 0x999999,
					"BridgeConstructionElement": 0x999999,
					"CityObjectGroup": 0xffffb3,
					"CityFurniture": 0xcc0000,
					"GenericCityObject": 0xcc0000,
					"LandUse": 0xffffb3,
					"PlantCover": 0x39ac39,
					"Railway": 0x000000,
					"Road": 0x999999,
					"SolitaryVegetationObject": 0x39ac39,
					"TINRelief": 0xffdb99,
					"TransportSquare": 0x999999,
					"Tunnel": 0x999999,
					"TunnelPart": 0x999999,
					"TunnelInstallation": 0x999999,
					"WaterBody": 0x4da6ff
				};

			}
		},

		background_color: {
			type: Number,
			default: 0xd9eefc
		}

	},
	data() {

		return {
			camera_init: false
		};

	},

	watch: {

		background_color: function ( newVal, ) {

			this.renderer.setClearColor( newVal );

			this.renderer.render( this.scene, this.camera );

		},

		object_colors: {
			handler: function ( newVal, ) {

				for ( var i = 0; i < this.meshes.length; i ++ )
					this.meshes[ i ].material.color.setHex( newVal[ this.citymodel.CityObjects[ this.meshes[ i ].name ].type ] );

				this.renderer.render( this.scene, this.camera );

			},
			deep: true

		},

		selected_objid: function ( newID, oldID ) {

			var split = newID.split(" ");
			var newID = { id: split[ 0 ], start: split[ 1 ], end: split[ 2 ] }


			if ( oldID != null ) {

				split = oldID.split(" ");
				oldID = { id: split[ 0 ], start: split[ 1 ], end: split[ 2 ] };

			}

			this.select_co( newID, oldID );

		}

	},
	beforeCreate() {

		this.scene = null;
		this.camera = null;
		this.renderer = null;
		this.controls = null;
		this.raycaster = null;
		this.mouse = null;
		this.geometry = new THREE.BufferGeometry();
		this.mesh = null;
		this.buffer;
		this.selectedCOColor;

	},

	async mounted() {

		this.$emit( 'rendering', true );

		setTimeout( async () => {

			var self = this;

			this.initScene();

			// Already render before streaming has finished, so that the background is shown in the meantime.
			this.renderer.render( this.scene, this.camera );

			await fetch( filePath )
			.then( r => r.arrayBuffer() )
			.then( function( buf ) {

				var arr = new Uint8Array( buf );

				// See https://github.com/rustwasm/wasm-bindgen/issues/1079, https://github.com/rustwasm/wasm-bindgen/issues/1643
				// Code has been sourced from there.
				self.buffer = new rust.WasmMemBuffer(arr.length, array => {
				// "array" wraps a piece of wasm memory. Fill it with some values.
				array.set( arr )
				})

				return rust.receive_buf( self.buffer );

			})
			.then( function( res ) {

				console.log("JS: COs parsed");

				// TODO: set vertices in another then
				let vs = rust.get_vertices( self.buffer );

				self.createGeometry(res.triangles, vs.vertices );

			});


			$( "#viewer" ).dblclick( function ( eventData ) {

				if ( eventData.button == 0 ) { //leftClick

					self.handleClick();

				}

			} );

			this.$emit( 'rendering', false );

		}, 25 );

	},

	methods: {

		select_co( newID, oldID ) {

			var attr = rust.get_attributes( this.buffer, newID.id );
			console.log(attr);

			var color;

			if ( oldID != null ) {

				color = this.selectedCOColor;
				// this.updateCOColor( color, oldID );

			}

			if ( newID != null ) {

			let coType = attr.type;
			color = [ 255, 255, 255 ];
			this.selectedCOColor = this.object_colors[ coType ];

			// this.updateCOColor( color, newID );

			}

			this.renderer.render( this.scene, this.camera );

		},

		findIDFaces( coId ) {



		},

		updateCOColor( color, coID ) {

			let firstFaceID = coID.start;
			let lastFaceID = coID.end;

			//console.log(firstFaceID, lastFaceID, color);

			for ( var i = firstFaceID; i <= lastFaceID; i ++ ) {

				var vertices = [];

				vertices.push( this.mesh.geometry.index.array[ i * 3 ] );
				vertices.push( this.mesh.geometry.index.array[ i * 3 + 1 ] );
				vertices.push( this.mesh.geometry.index.array[ i * 3 + 2 ] );

				// console.log(vertices);

				for ( var v = 0; v < vertices.length; v ++ ) {

					this.mesh.geometry.attributes.color.array[ vertices[ v ] * 3 ] = color[0];
					this.mesh.geometry.attributes.color.array[ vertices[ v ] * 3 + 1 ] = color[1];
					this.mesh.geometry.attributes.color.array[ vertices[ v ] * 3 + 2 ] = color[2];

				}

			}

			this.mesh.geometry.colorsNeedUpdate = true;
			this.mesh.geometry.attributes.color.needsUpdate = true;

		},

		handleClick() {

			var rect = this.renderer.domElement.getBoundingClientRect();
			this.mouse.x = ( ( event.clientX - rect.left ) / this.renderer.domElement.clientWidth ) * 2 - 1;
			this.mouse.y = - ( ( event.clientY - rect.top ) / this.renderer.domElement.clientHeight ) * 2 + 1;

			this.raycaster.setFromCamera( this.mouse, this.camera );
			var intersects = this.raycaster.intersectObject( this.mesh );

			if ( intersects.length == 0 ) {

				this.$emit( 'object_clicked', null );
				return;

			}

			var cityObjId = rust.get_interval_and_id( intersects[ 0 ].faceIndex );

			this.$emit( 'object_clicked', cityObjId );

		},

		initScene() {

			this.scene = new THREE.Scene();
			var ratio = $( "#viewer" ).width() / $( "#viewer" ).height();
			this.camera = new THREE.PerspectiveCamera( 60, ratio, 0.001, 1000 );
			this.camera.up.set( 0, 0, 1 );
			this.camera.position.set( 0, 0, 2 );
			this.camera.lookAt( 0, 0, 0 );

			this.renderer = new THREE.WebGLRenderer( { antialias: true } );
			var viewer = document.getElementById( "viewer" );
			viewer.appendChild( this.renderer.domElement );
			this.renderer.setSize( $( "#viewer" ).width(), $( "#viewer" ).height() );
			this.renderer.setClearColor( this.background_color );
			this.renderer.shadowMap.enabled = true;
			this.renderer.shadowMap.type = THREE.PCFSoftShadowMap;

			this.raycaster = new THREE.Raycaster();
			this.mouse = new THREE.Vector2();

			var ambLight = new THREE.AmbientLight( 0xFFFFFF, 0.7 );
			ambLight.name = "ambLight";
			var spotLight = new THREE.SpotLight( 0xDDDDDD, 0.4 );
			spotLight.name = "spotLight";
			spotLight.position.set( 0, - 1, 1 );
			spotLight.target = this.scene;
			spotLight.castShadow = true;
			this.scene.add( spotLight, ambLight );

			let self = this;
			this.controls = new OrbitControls( this.camera, this.renderer.domElement );
			this.controls.addEventListener( 'change', function () {

				self.renderer.render( self.scene, self.camera );

			} );

			this.controls.screenSpacePanning = true;

		},

		clearScene() {

			for ( var i = this.scene.children.length - 1; i >= 0; i -- ) {

				if ( this.scene.children[ i ].name != "ambLight" && this.scene.children[ i ].name != "spotLight" ) {

					this.scene.remove( this.scene.children[ i ] );

				}

			}

			// TODO: properly reinitialise all properties and test if this function works well.
			this.mesh = null;
			this.geometry = new THREE.BufferGeometry();

		},

		async createGeometry( triangles, vertices ) {

			console.log(triangles.groups);

			// Set triangles and vertices
			this.geometry.setIndex( triangles.triangles );
			this.geometry.setAttribute( 'position', new THREE.Float32BufferAttribute( vertices, 3 ) );

			// Create geometry groups (for every CityObject type)
			var materials = [];
			for ( const [ coType, triangleIndices ] of triangles.groups.entries() ) {

				var material = new THREE.MeshLambertMaterial();
				material.color = new THREE.Color( this.object_colors[ coType ] );
				materials.push( material );

				console.log(triangleIndices[ 0 ], triangleIndices[ 1 ]);

				// triangleIndices[ 0 ] = start index, triangleIndices[ 1 ] = triangle count
				this.geometry.addGroup( triangleIndices[ 0 ], triangleIndices[ 1 ], materials.length - 1 )


			}

			// var material = new THREE.MeshLambertMaterial();
			// material.color = new THREE.Color( this.object_colors[ "Building" ] );

			this.mesh = new THREE.Mesh( this.geometry, materials );
			this.mesh.castShadow = true;
			this.mesh.receiveShadow = true;

			// Normalize coordinates
			// TODO: normalise vertices before loading into buffer?
			this.geometry.computeBoundingSphere();

			const center = this.geometry.boundingSphere.center;
			const radius = this.geometry.boundingSphere.radius;

			const s = radius === 0 ? 1 : 1.0 / radius;

			const matrix = new THREE.Matrix4();
			matrix.set(
				s, 0, 0, - s * center.x,
				0, s, 0, - s * center.y,
				0, 0, s, - s * center.z,
				0, 0, 0, 1
			);

			this.geometry.applyMatrix4( matrix );
			this.geometry.computeVertexNormals();

			// this.geometry.setDrawRange( 0, 100000 );

			this.scene.add( this.mesh );

			console.log("Geometry added to scene");

			this.renderer.render( this.scene, this.camera );

		},

	}
};
</script>
