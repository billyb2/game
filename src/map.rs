#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

use std::io::Read;

use bevy::math::prelude::*;
use bevy::sprite::prelude::*;
use bevy::transform::prelude::*;
use bevy::ecs::prelude::*;
use bevy::render::prelude::*;
use bevy::asset::prelude::*;

use crate::{GameRelated, Health};
use crate::components::WallMarker;

use crate::helper_functions::*;

use crc32fast::Hasher;
use lz4_flex::frame::FrameDecoder;

use single_byte_hashmap::*;
use hashbrown::HashMap as HashBrownMap;

#[cfg(feature = "parallel")]
use rayon::prelude::*;
#[cfg(feature = "parallel")]
use rayon::join;

#[derive(Bundle)]
pub struct MapObject {
    pub coords: Vec3,
    pub size: Vec2,
    pub sprite: UVec4,
    pub collidable: bool,
    pub player_spawn: bool,
    pub using_image: bool,
    pub health: Option<f32>,

}

pub struct Map {
    pub name: String,
    pub objects: Vec<MapObject>,
    pub background_color: Color,
    pub size: Vec2,

}

pub struct MapAssets(pub HashMap<u8, Handle<ColorMaterial>>);

pub struct Maps(pub HashBrownMap<String, Map>);

impl MapObject {
    fn collision(&self, other_object_coords: Vec2, other_object_size: Vec2, distance: f32, angle: f32) -> bool {
        //Just runs a simple rectangle - rectangle collision function, if the given map object can be collided with
        self.collidable && collide(other_object_coords, other_object_size, self.coords.truncate(), self.size, distance, angle)

    }

}

impl Map {
    pub fn new(name: String, objects: Vec<MapObject>, size: [f32; 2], background_color: Color) -> Map {
        Map {
            name,
            objects,
            size: Vec2::new(size[0], size[1]),
            background_color,

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
        let map_width = slice_to_u32(&bytes[0..=3]) * 6;
        let map_height = slice_to_u32(&bytes[4..=7]) * 6;
        let background_color = Color::rgb_u8(bytes[8], bytes[9], bytes[10]);

        let mut start_of_map = 11;

        let mut map_char_vec = Vec::with_capacity(10);

        for byte in &bytes[11..] {
            if *byte == 0 {

                break;
            } 

            map_char_vec.push(*byte as char);

            start_of_map += 1;

        }

        let map_name: String = map_char_vec.into_iter().collect();

        let mut objects: Vec<MapObject> = Vec::with_capacity(0);

        // The map metadata length is 11 bytes
        // Splits the map into chunks each the size of a single map object
        #[cfg(feature = "parallel")]
        let chunks = (&bytes[start_of_map..]).par_chunks_exact(24);

        #[cfg(not(feature = "parallel"))]
        let chunks = (&bytes[start_of_map..]).chunks_exact(24);

        // Since the CRC32 is 4 bytes, it will be the final remainder of the map
        let crc32: u32 = slice_to_u32(chunks.remainder());

        let add_map_objects = || { 
            // Iterates through the entire map, adding a map object for each chunk
            objects = chunks.map(|chunk| {
                let x = (slice_to_u32(&chunk[0..=(3)])) as f32;
                let y = (slice_to_u32(&chunk[(4)..=(7)])) as f32;
                let width = (slice_to_u32(&chunk[(8)..=(11)])) as f32;
                let height = (slice_to_u32(&chunk[(12)..=(15)])) as f32;

                MapObject {
                    // Gotta adjust for Bevy's coordinate system center being at (0, 0)
                    coords: Vec3::new(x, -y, 0.0) + Vec3::new(width / 2.0, -height / 2.0, 1.0),
                    size: Vec2::new(width, height),
                    player_spawn: matches!(&chunk[(16)], 255),
                    collidable: matches!(&chunk[(17)], 255),

                    sprite: UVec4::new(chunk[19].into(), chunk[20].into(), chunk[21].into(), chunk[22].into()),
                    using_image: matches!(&chunk[18], 255),

                    health: match chunk[23] {
                        0 => None,
                        health => Some(health as f32),
                    },
                }

            }).collect();

        };

        let calculate_crc32 = || {
            // Performs a CRC32 hash of the file, and compares it to the CRC32 given
            let mut hasher = Hasher::new();
            hasher.update(&bytes[0..(bytes.len() - 4)]);

            let checksum: u32 = hasher.finalize();

            if checksum == crc32 {
                println!("Verified map checksum!");

            } else {
                panic!("The map file is corrupted! (Checksums don't match)");

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

        Map::new(map_name, objects, [map_width as f32, map_height as f32], background_color)

    }

    // Returns the health of a wall if they have health
    pub fn collision(&mut self, other_object_coords: Vec2, other_object_size: Vec2, damage: f32, distance: f32, angle: f32) -> (bool, Option<(f32, Vec2)>) {
        let map_collision = |index: &usize| {
            self.objects[*index].collision(other_object_coords, other_object_size, distance, angle)

        };

        // The collision function just iterates throuhg each map object within the map, and runs the collide function within
        #[cfg(feature = "parallel")]
        let index = (0..self.objects.len()).into_par_iter().find_any(map_collision);

        #[cfg(not(feature = "parallel"))]
        let index = (0..self.objects.len()).into_iter().find(map_collision);


        let health_and_coords = match index {
            Some(index) => { 
                 if let Some(mut health) = &mut self.objects[index].health {
                    // Damagable objects take damage
                    if health as i16 - damage as i16 <= 0 {
                        health = 0.0;

                    } else {
                        health -= damage;

                    }


                    if health == 0.0 {
                        self.objects.remove(index);

                    }

                    Some((health,  self.objects[index].coords.truncate())) 

                } else {
                    None

                }

            },
            None => None,

        };

        (index.is_some(), health_and_coords)

     }

    // Identical to collision, except it's a non-mutable reference so it's safe to use in an iterator
    pub fn collision_no_damage(&self, other_object_coords: Vec2, other_object_size: Vec2, distance: f32, angle: f32) -> bool {
        let map_collision = |index: usize| {
            self.objects[index].collision(other_object_coords, other_object_size, distance, angle)

        };

        // The collision function just iterates through each map object within the map, and runs the collide function within
        // Since this function is only used in par_for_each loops, we don't need extra parallelism
        #[cfg(not(feature = "parallel"))]
        let collision = (0..self.objects.len()).into_iter().any(map_collision);

        #[cfg(feature = "parallel")]
        let collision = (0..self.objects.len()).into_par_iter().any(map_collision);

        collision

     }

}

// This system just iterates through the map and draws each MapObject
pub fn draw_map(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, maps: Res<Maps>, mut map_assets: ResMut<MapAssets>, asset_server: Res<AssetServer>) {

    let map = maps.0.get(&String::from("default")).unwrap();

    // Set the background color to the map's specified color
    commands.insert_resource(ClearColor(map.background_color));

    map.objects.iter().for_each(|object| {
        let map_coords = object.coords;
        let map_object_size =  object.size;

        let map_asset_int = slice_to_u32(&[object.sprite.x as u8, object.sprite.y as u8, object.sprite.z as u8, object.sprite.w as u8]) as u8;

        let color_handle = match object.using_image {
            true => match map_assets.0.get(&map_asset_int) {
                Some(asset) => asset.clone(),
                None => {
                    let path_string = &*format!("map_assets/{}.png", map_asset_int);
                    let asset = asset_server.load(path_string);
                    let asset = materials.add(asset.into());

                    map_assets.0.insert(map_asset_int, asset.clone());

                    asset.clone()
                }
            },
            false => materials.add(Color::rgba_u8(object.sprite.x as u8, object.sprite.y as u8, object.sprite.z as u8, object.sprite.w as u8).into()),

        };

        // Spawn a new map sprite
        commands
            .spawn_bundle(SpriteBundle {
                material: color_handle,
                sprite: Sprite::new(map_object_size),
                transform: Transform::from_translation(map_coords),
                ..Default::default()
            })
            .insert(Health(100.0))
            .insert(WallMarker)
            .insert(GameRelated);

    });

}
