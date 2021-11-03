#![feature(stmt_expr_attributes)]
#![feature(slice_as_chunks)]
#![feature(option_result_unwrap_unchecked)]
#![feature(control_flow_enum)]

#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

use std::cell::RefCell;
use std::io::{Read, Write};
use std::rc::Rc;
use std::ops::ControlFlow;

use bevy::prelude::*;
use bevy::math::const_vec2;

use game_types::{ColliderHandleWrapper, GameRelated, RigidBodyHandleWrapper, Size};

use helper_functions::*;

use crc32fast::Hasher;
use lz4_flex::frame::{FrameEncoder, FrameDecoder};

use rapier2d::prelude::*;
use rapier2d::na::Vector2;

use rustc_hash::FxHashMap;
use single_byte_hashmap::*;

#[cfg(feature = "parallel")]
use rayon::join;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

// The identifier for the map
pub struct MapCRC32(pub u32);

#[derive(Component)]
pub struct WallMarker;

#[derive(Bundle, Clone, PartialEq)]
pub struct MapObject {
    pub coords: MapObjectCoords,
    pub size: Size,
    pub sprite: MapColor,
    pub collidable: Bool,
    pub player_spawn: Bool,
    pub using_image: Bool,
    pub health: MapHealth,
}

pub struct Map {
    pub name: String,
    pub objects: Vec<MapObject>,
    pub background_color: Color,
    pub size: Vec2,
    pub crc32: u32,
    pub spawn_points: Vec<Vec2>,
}

pub struct MapAssets(pub HashMap<u8, Handle<ColorMaterial>>);

pub struct Maps(pub FxHashMap<u32, Map>);

impl MapObject {
    // Convert the map object to a bin array
    pub fn to_bin(&self) -> [u8; 32] {
        let bool_to_byte = 
        #[inline(always)]
        |boolean: bool| -> u8 {
            match boolean {
                true => 255,
                false => 0,
            }
        };

        const MAP_OBJECT_LEN: usize = 32;

        let mut bytes: [u8; MAP_OBJECT_LEN] = [0; MAP_OBJECT_LEN];

        // The line below makes sure that the unsafe code below is sound
        debug_assert!(MAP_OBJECT_LEN % 4 == 0);
        let byte_chunks = unsafe { bytes.as_chunks_unchecked_mut::<4>() };

        byte_chunks[0] = ((self.coords.0.x - (self.size.0.x / 2.0))).to_le_bytes();
        byte_chunks[1] = ((-self.coords.0.y - (self.size.0.y / 2.0))).to_le_bytes();
        byte_chunks[2] = (self.coords.0.z).to_le_bytes();
        byte_chunks[3] = (self.coords.0.w).to_le_bytes();

        byte_chunks[4] = (self.size.0.x).to_le_bytes();
        byte_chunks[5] = (self.size.0.y).to_le_bytes();

        // Arrays neeed to be the exact same size in order to be concactenated for some reason
        byte_chunks[6] = [
            bool_to_byte(self.player_spawn.0),
            bool_to_byte(self.collidable.0),
            bool_to_byte(self.using_image.0),
            self.sprite.r,
        ];

        byte_chunks[7] = [
            self.sprite.g,
            self.sprite.b,
            self.sprite.a,

            match self.health.0 {
                Some(health) => health as u8,
                None => 0,
            }
        ];


        debug_assert_eq!(bytes.len(), 32);
        // If the entire map object byte is null, that means either it's a very very weird map object (will almost never happen naturally), or the function is bugged for some reason
        debug_assert_ne!(bytes, [0; 32]);

        bytes
    }

    pub fn from_bin(chunk: &[u8]) -> MapObject {
        let x = slice_to_f32(&chunk[0..=(3)]);
        let y = slice_to_f32(&chunk[(4)..=(7)]);
        let z = slice_to_f32(&chunk[(8)..=(11)]);

        let rotation = slice_to_f32(&chunk[(12)..=(15)]);

        let width = slice_to_f32(&chunk[(16)..=(19)]);
        let height = slice_to_f32(&chunk[(20)..=(23)]);

        MapObject {
            // Gotta adjust for Bevy's coordinate system center being at (0, 0)
            coords: MapObjectCoords(Vec4::new(x + width / 2.0, -y -height / 2.0, z, rotation)),
            size: Size::new(width, height),
            player_spawn: Bool(matches!(&chunk[(24)], 255)),
            collidable: Bool(matches!(&chunk[(25)], 255)),

            using_image: Bool(matches!(&chunk[26], 255)),

            sprite: MapColor::new(
                chunk[27].into(),
                chunk[28].into(),
                chunk[29].into(),
                chunk[30].into(),
            ),

            health: MapHealth(match chunk[31] {
                0 => None,
                health => Some(health as f32),
            }),
        }
    }

    pub const fn default() -> MapObject {
        MapObject {
            coords: MapObjectCoords(Vec4::ZERO),
            size: Size(const_vec2!([0.0; 2])),
            sprite: MapColor::default(),
            collidable: Bool(false),
            using_image: Bool(false),
            player_spawn: Bool(false),
            health: MapHealth(None),
        }
    }

}


impl Map {
    pub fn new(
        name: String,
        objects: Vec<MapObject>,
        size: [f32; 2],
        background_color: Color,
        crc32: u32,
        spawn_points: Vec<Vec2>
    ) -> Map {
        Map {
            name,
            objects,
            size: Vec2::new(size[0], size[1]),
            background_color,
            crc32,
            spawn_points,
        }
    }

    pub fn from_bin(compressed_bytes: &[u8]) -> Map {
        //Decompress the map
        let mut bytes: Vec<u8> = Vec::with_capacity(500);

        let mut decoder = FrameDecoder::new(compressed_bytes);

        decoder.read_to_end(&mut bytes).unwrap();

        // Just dropping the FrameDecoder to save a little bit of memory
        std::mem::drop(decoder);

        //Unallocates all the extra capacity
        bytes.shrink_to_fit();

        // The first few bytes of the map are metadata, like the dimensions of the map, its background color, etc.
        let map_width = slice_to_f32(&bytes[0..=3]);
        let map_height = slice_to_f32(&bytes[4..=7]);

        let background_color = Color::rgb_u8(bytes[8], bytes[9], bytes[10]);

        let map_name = bytes[11..].iter().try_fold(String::with_capacity(10), |mut o_map_name, &byte_char| {
            match byte_char != 0 {
                true => {
                    o_map_name.push(byte_char as char);
                    ControlFlow::Continue(o_map_name)
                },
                false => ControlFlow::Break(o_map_name),

            }

        }).break_value().unwrap();

        let start_of_map = 11 + map_name.len();

        let mut objects: Vec<MapObject> = Vec::new();

        // The map metadata length is 11 bytes
        // Splits the map into chunks each the size of a single map object
        #[cfg(feature = "parallel")]
        let chunks = (&bytes[start_of_map..]).par_chunks_exact(32);

        #[cfg(not(feature = "parallel"))]
        let chunks = (&bytes[start_of_map..]).chunks_exact(32);

        // Since the CRC32 is 4 bytes, it will be the final remainder of the map
        let crc32: u32 = slice_to_u32(chunks.remainder());
        let add_map_objects = 
        #[inline(always)]
        || {
            // Iterates through the entire map, adding a map object for each chunk
            objects = chunks.map(MapObject::from_bin).collect();
            objects.shrink_to_fit();
            
        };

        let calculate_crc32 = 
        #[inline(always)]
        || {
            // Performs a CRC32 hash of the file, and compares it to the CRC32 given
            let mut hasher = Hasher::new();
            hasher.update(&bytes[0..(bytes.len() - 4)]);

            let checksum: u32 = hasher.finalize();

            match checksum == crc32 {
                true => println!("Verified map checksum!"),
                false => panic!("The map file is corrupted! (Checksums don't match)"),
            }

        };

        // Calculates the CRC32 and adds map objects at the same time
        #[cfg(feature = "parallel")]
        join(add_map_objects, calculate_crc32);

        #[cfg(not(feature = "parallel"))]
        {
            add_map_objects();
            calculate_crc32();
        }

        let find_spawn_points = 
        #[inline]
        |map_object: &MapObject| {
            match map_object.player_spawn.0 {
                true => Some(map_object.coords.0.truncate().truncate()),
                false => None,
            }
        };

        #[cfg(feature = "parallel")]
        let objects_iter = objects.par_iter();

        #[cfg(not(feature = "parallel"))]
        let objects_iter = objects.iter();

        let spawn_points = objects_iter.filter_map(find_spawn_points).collect();

        let map = Map::new(map_name, objects, [map_width, map_height], background_color, crc32, spawn_points);

        // Quick check to make sure the to_bin function is working
        // It isn't working at the moment, please fix :(
        //debug_assert_eq!(bytes[..], map_to_bin(&map, false));

        map
    }

}

// This system just iterates through the map and draws each MapObject
pub fn draw_map(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, maps: Res<Maps>, map_crc32: Res<MapCRC32>, mut map_assets: ResMut<MapAssets>, asset_server: Res<AssetServer>, mut collider_set: ResMut<ColliderSet>, mut rigid_body_set: ResMut<RigidBodySet>) {
    let map = maps.0.get(&map_crc32.0).unwrap();

    // Set the background color to the map's specified color
    commands.insert_resource(ClearColor(map.background_color));

    map.objects.iter().for_each(|object| {
        let map_coords = &object.coords;
        let map_object_size = &object.size;

        let map_asset_int = slice_to_u32(&[
            object.sprite.r,
            object.sprite.g,
            object.sprite.b,
            object.sprite.a,
        ]) as u8;

        let color_handle = match object.using_image.0 {
            true => match map_assets.0.get(&map_asset_int) {
                Some(asset) => asset.clone(),
                None => {
                    let path_string = &*format!("map_assets/{}.png", map_asset_int);
                    let asset = asset_server.load(path_string);
                    let asset = materials.add(asset.into());

                    map_assets.0.insert(map_asset_int, asset.clone());

                    asset
                }
            },
            false => materials.add(
                Color::rgba_u8(
                    object.sprite.r,
                    object.sprite.g,
                    object.sprite.b,
                    object.sprite.a,
                )
                .into(),
            ),
        };


        // Only do physics calcs on an object if it's collidable
        let physics_handles = if object.collidable.0 {
            let half_extents = map_object_size.0 / bevy::math::const_vec2!([500.0; 2]);

            let rigid_body = RigidBodyBuilder::new(RigidBodyType::Static)
                .translation(Vector2::new(map_coords.0.x, map_coords.0.y).component_div(&Vector2::new(250.0, 250.0))) 
                .gravity_scale(0.0)
                .build();


            let collider = ColliderBuilder::cuboid(half_extents.x, half_extents.y)
                .collision_groups(InteractionGroups::new(0b0100, 0b1010))
                .friction(0.5)
                .build();

            let rigid_body_handle = rigid_body_set.insert(rigid_body);
            let collider_handle = collider_set.insert_with_parent(collider, rigid_body_handle, &mut rigid_body_set);

            Some((RigidBodyHandleWrapper(rigid_body_handle), ColliderHandleWrapper(collider_handle)))

        } else {
            None

        };

        // Spawn a new map sprite
        let mut entity = commands
            .spawn_bundle(SpriteBundle {
                material: color_handle,
                sprite: Sprite::new(map_object_size.0),
                transform: Transform {
                    translation: map_coords.0.truncate(),
                    rotation: Quat::from_rotation_z(map_coords.0.w),

                    ..Default::default()

                },
                ..Default::default()
            });

        entity
            .insert(WallMarker)
            .insert(GameRelated);

        if let Some((rigid_body_handle, collider_handle)) = physics_handles {
            entity
                .insert(rigid_body_handle)
                .insert(collider_handle);
        }

    });
}

//TODO: Change this whole fn to use a map (Iterator)?
//TODO: Fix this function
#[allow(dead_code)]
fn map_to_bin(map: &Map, should_compress: bool) -> Vec<u8> {
    let map_bytes: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::with_capacity(900)));

    // Need to use a reference counter or the compiler complains about moving map_bytes
    let push_to_map = 
    #[inline]
    |b: &u8| map_bytes.borrow_mut().push(*b);

    map_bytes.borrow_mut().extend_from_slice(&(map.size.x.to_le_bytes()));
    map_bytes.borrow_mut().extend_from_slice(&(map.size.y.to_le_bytes()));

    push_to_map(&((map.background_color.r() * u8::MAX as f32).round() as u8));
    push_to_map(&((map.background_color.g() * u8::MAX as f32).round() as u8));
    push_to_map(&((map.background_color.b() * u8::MAX as f32).round() as u8));

    map_bytes.borrow_mut().extend_from_slice(map.name.as_bytes());

    #[cfg(feature = "parallel")]
    let objects_iter = map.objects.par_iter();

    #[cfg(not(feature = "parallel"))]
    let objects_iter = map.objects.iter();

    let flat_object_map = objects_iter.map(|object| object.to_bin()).flatten();

    #[cfg(feature = "parallel")]
    map_bytes.borrow_mut().par_extend(flat_object_map);

    #[cfg(not(feature = "parallel"))]
    map_bytes.borrow_mut().extend(flat_object_map);

    let crc32 = {
        let mut hasher = Hasher::new();
        hasher.update(&map_bytes.borrow());
        hasher.finalize()

    };

    map_bytes.borrow_mut().extend_from_slice(&crc32.to_be_bytes());

    if should_compress {
        let mut compressed_bytes: Vec<u8> = Vec::with_capacity(500);

        let mut compressor = FrameEncoder::new(&mut compressed_bytes);

        compressor.write_all(&map_bytes.borrow()).unwrap();
        compressor.finish().unwrap();

        compressed_bytes

    } else {
        Rc::try_unwrap(map_bytes).unwrap().into_inner()

    }
}

#[derive(Component, Clone, PartialEq)]
pub struct MapObjectCoords(pub Vec4);

#[derive(Component, Clone, PartialEq)]
pub struct MapColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,

}

impl MapColor {
    const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r,
            g,
            b,
            a,
        }

    }

    const fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}

#[derive(Component, Clone, PartialEq)]
pub struct MapHealth(pub Option<f32>);

#[derive(Component, Copy, Clone, PartialEq)]
pub struct Bool(pub bool);