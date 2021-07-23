#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

use std::cell::RefCell;
use std::io::{Read, Write};
use std::rc::Rc;

use bevy::asset::prelude::*;
use bevy::ecs::prelude::*;
use bevy::math::prelude::*;
use bevy::math::Vec4Swizzles;
use bevy::render::prelude::*;
use bevy::sprite::prelude::*;
use bevy::transform::prelude::*;

use crate::components::WallMarker;
use crate::{GameRelated, Health, MapCRC32};

use crate::helper_functions::*;

use crc32fast::Hasher;
use lz4_flex::frame::{FrameEncoder, FrameDecoder};

use hashbrown::HashMap as HashBrownMap;
use single_byte_hashmap::*;

#[cfg(feature = "parallel")]
use rayon::join;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[derive(Bundle)]
pub struct MapObject {
    pub coords: Vec4,
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
    pub crc32: u32,
    pub spawn_points: Vec<Vec2>,
}

pub struct MapAssets(pub HashMap<u8, Handle<ColorMaterial>>);

pub struct Maps(pub HashBrownMap<u32, Map>);

impl MapObject {
    fn collision(
        &self,
        other_object_coords: Vec2,
        other_object_size: Vec2,
        distance: f32,
        angle: f32,
    ) -> bool {
        //Just runs a simple rectangle - rectangle collision function, if the given map object can be collided with
        self.collidable && collide(other_object_coords, other_object_size, self.coords.xy(), self.size, distance, angle)
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
        let map_width = slice_to_u32(&bytes[0..=3]) as f32;
        let map_height = slice_to_u32(&bytes[4..=7]) as f32;
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

        let map_crc32: String = map_char_vec.into_iter().collect();

        let mut objects: Vec<MapObject> = Vec::with_capacity(0);

        // The map metadata length is 11 bytes
        // Splits the map into chunks each the size of a single map object
        #[cfg(feature = "parallel")]
        let chunks = (&bytes[start_of_map..]).par_chunks_exact(32);

        #[cfg(not(feature = "parallel"))]
        let chunks = (&bytes[start_of_map..]).chunks_exact(32);

        // Since the CRC32 is 4 bytes, it will be the final remainder of the map
        let crc32: u32 = slice_to_u32(chunks.remainder());
        let add_map_objects = || {
            // Iterates through the entire map, adding a map object for each chunk
            objects = chunks
                .map(|chunk| {
                    let x = (slice_to_u32(&chunk[0..=(3)])) as f32;
                    let y = (slice_to_u32(&chunk[(4)..=(7)])) as f32;
                    let z = (slice_to_u32(&chunk[(8)..=(11)])) as f32;

                    let rotation = (slice_to_u32(&chunk[(12)..=(15)])) as f32;

                    let width = (slice_to_u32(&chunk[(16)..=(19)])) as f32;
                    let height = (slice_to_u32(&chunk[(20)..=(23)])) as f32;

                    MapObject {
                        // Gotta adjust for Bevy's coordinate system center being at (0, 0)
                        coords: Vec4::new(x + width / 2.0, -y -height / 2.0, z, rotation),
                        size: Vec2::new(width, height),
                        player_spawn: matches!(&chunk[(24)], 255),
                        collidable: matches!(&chunk[(25)], 255),

                        using_image: matches!(&chunk[26], 255),

                        sprite: UVec4::new(
                            chunk[27].into(),
                            chunk[28].into(),
                            chunk[29].into(),
                            chunk[30].into(),
                        ),

                        health: match chunk[31] {
                            0 => None,
                            health => Some(health as f32),
                        },
                    }
                })
                .collect();
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

        let find_spawn_points = |map_object: &MapObject| {
            if map_object.player_spawn {
                Some(map_object.coords.truncate().truncate())

            } else {
                None

            }
        };

        #[cfg(feature = "parallel")]
        let spawn_points = objects.par_iter().filter_map(find_spawn_points).collect();

        #[cfg(not(feature = "parallel"))]
        let spawn_points = objects.iter().filter_map(find_spawn_points).collect();

        let map = Map::new(map_crc32, objects, [map_width, map_height], background_color, crc32, spawn_points);

        // Quick check to make sure the to_bin function is working
        debug_assert!(bytes[..] == map_to_bin(&map, false));

        map
    }

    // Returns whether a collision took place, and the health of the wall (if it has health)
    pub fn collision(&mut self, other_object_coords: Vec2, other_object_size: Vec2, damage: f32, distance: f32, angle: f32) -> (bool, Option<(f32, Vec2)>) {
        let map_collision = |index: &usize| {
            self.objects[*index].collision(other_object_coords, other_object_size, distance, angle)
        };

        // The collision function just iterates throuhg each map object within the map, and runs the collide function within
        #[cfg(feature = "parallel")]
        let index = (0..self.objects.len())
            .into_par_iter()
            .find_any(map_collision);

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

                    Some((health, self.objects[index].coords.xy()))
                } else {
                    None
                }
            }
            None => None,
        };

        (index.is_some(), health_and_coords)
    }

    // Identical to collision, except it's a non-mutable reference so it's safe to use in a parallel iterator
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
pub fn draw_map(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, maps: Res<Maps>, map_crc32: Res<MapCRC32>, mut map_assets: ResMut<MapAssets>, asset_server: Res<AssetServer>) {
    let map = maps.0.get(&map_crc32.0).unwrap();

    // Set the background color to the map's specified color
    commands.insert_resource(ClearColor(map.background_color));

    map.objects.iter().for_each(|object| {
        let map_coords = object.coords;
        let map_object_size = object.size;

        let map_asset_int = slice_to_u32(&[
            object.sprite.x as u8,
            object.sprite.y as u8,
            object.sprite.z as u8,
            object.sprite.w as u8,
        ]) as u8;

        let color_handle = match object.using_image {
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
                    object.sprite.x as u8,
                    object.sprite.y as u8,
                    object.sprite.z as u8,
                    object.sprite.w as u8,
                )
                .into(),
            ),
        };

        // Spawn a new map sprite
        commands
            .spawn_bundle(SpriteBundle {
                material: color_handle,
                sprite: Sprite::new(map_object_size),
                transform: Transform {
                    translation: map_coords.truncate(),
                    rotation: Quat::from_rotation_z(map_coords.w),

                    ..Default::default()

                },
                ..Default::default()
            })
            .insert(Health(100.0))
            .insert(WallMarker)
            .insert(GameRelated);
    });
}

fn map_to_bin(map: &Map, should_compress: bool) -> Vec<u8> {
    let map_bytes: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::with_capacity(700)));

    // Need to use a reference counter or the compiler complains about moving map_bytes
    let push_to_map = |b: &u8| map_bytes.borrow_mut().push(*b);

    (map.size.x as u32)
        .to_be_bytes()
        .iter()
        .for_each(push_to_map);
    (map.size.y as u32)
        .to_be_bytes()
        .iter()
        .for_each(push_to_map);

    push_to_map(&((map.background_color.r() * u8::MAX as f32).round() as u8));
    push_to_map(&((map.background_color.g() * u8::MAX as f32).round() as u8));
    push_to_map(&((map.background_color.b() * u8::MAX as f32).round() as u8));

    map.name.as_bytes().iter().for_each(push_to_map);

    //TODO: par_map/par_iter
    map.objects.iter().for_each(|object| {
        ((object.coords.x - (object.size.x / 2.0)) as u32)
            .to_be_bytes()
            .iter()
            .for_each(push_to_map);
        ((-object.coords.y - (object.size.y / 2.0)) as u32)
            .to_be_bytes()
            .iter()
            .for_each(push_to_map);

        (object.coords.z as u32)
            .to_be_bytes()
            .iter()
            .for_each(push_to_map);

        (object.coords.w as u32)
            .to_be_bytes()
            .iter()
            .for_each(push_to_map);

        (object.size.x as u32)
            .to_be_bytes()
            .iter()
            .for_each(push_to_map);
        (object.size.y as u32)
            .to_be_bytes()
            .iter()
            .for_each(push_to_map);

        push_to_map(&match object.player_spawn {
            true => 255,
            false => 0,
        });

        push_to_map(&match object.collidable {
            true => 255,
            false => 0,
        });

        push_to_map(&match object.using_image {
            true => 255,
            false => 0,
        });

        push_to_map(&(object.sprite.x as u8));
        push_to_map(&(object.sprite.y as u8));
        push_to_map(&(object.sprite.z as u8));
        push_to_map(&(object.sprite.w as u8));

        push_to_map(&match object.health {
            Some(health) => health as u8,
            None => 0,
        });
    });

    let mut hasher = Hasher::new();
    hasher.update(&map_bytes.borrow());
    let crc32 = hasher.finalize();

    crc32.to_be_bytes().iter().for_each(push_to_map);

    if should_compress {
        let mut compressed_bytes: Vec<u8> = Vec::with_capacity(500);

        let mut compressor = FrameEncoder::new(&mut compressed_bytes);

        compressor.write_all(&map_bytes.borrow()).unwrap();
        (*compressor.finish().unwrap()).clone()

    } else {
        Rc::try_unwrap(map_bytes).unwrap().into_inner()

    }
}
