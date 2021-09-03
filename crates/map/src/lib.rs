#![feature(stmt_expr_attributes)]
#![feature(slice_as_chunks)]
#![feature(option_result_unwrap_unchecked)]

#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

use std::cell::RefCell;
use std::io::{Read, Write};
use std::rc::Rc;

use bevy::math::Vec4Swizzles;
use bevy::prelude::*;

use game_types::{GameRelated, Health};

use helper_functions::*;

use crc32fast::Hasher;
use lz4_flex::frame::{FrameEncoder, FrameDecoder};

use rustc_hash::FxHashMap;
use single_byte_hashmap::*;

#[cfg(feature = "parallel")]
use rayon::join;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

// The identifier for the map
pub struct MapCRC32(pub u32);

pub struct WallMarker;

#[derive(Bundle, Clone, PartialEq)]
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

pub struct Maps(pub FxHashMap<u32, Map>);

impl MapObject {
    fn collision(
        &self,
        other_object_coords: Vec2,
        other_object_size: Vec2,
        distance: f32,
        angle: Vec2,
    ) -> (bool, bool) {
        //Just runs a simple rectangle - rectangle collision function, if the given map object can be collided with
        match self.collidable{
            true => collide(other_object_coords, other_object_size, self.coords.xy(), self.size, distance, angle),
            false => (false, false)
        }
    }

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

        let mut bytes: [u8; 32] = [0; 32];
        let byte_chunks = unsafe { bytes.as_chunks_unchecked_mut::<4>() };

        byte_chunks[0] = ((self.coords.x - (self.size.x / 2.0))).to_le_bytes();
        byte_chunks[1] = ((-self.coords.y - (self.size.y / 2.0))).to_le_bytes();
        byte_chunks[2] = (self.coords.z).to_le_bytes();
        byte_chunks[3] = (self.coords.w).to_le_bytes();

        byte_chunks[4] = (self.size.x).to_le_bytes();
        byte_chunks[5] = (self.size.y).to_le_bytes();

        // Arrays neeed to be the exact same size in order to be concactenated for some reason
        byte_chunks[6] = [
            bool_to_byte(self.player_spawn),
            bool_to_byte(self.collidable),
            bool_to_byte(self.using_image),
            self.sprite.x as u8
        ];

        byte_chunks[7] = [
            self.sprite.y as u8,
            self.sprite.z as u8,
            self.sprite.w as u8,

            match self.health {
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
    }

    pub const fn default() -> MapObject {
        MapObject {
            coords: Vec4::ZERO,
            size: Vec2::ZERO,
            sprite: UVec4::ZERO,
            collidable: false,
            using_image: false,
            player_spawn: false,
            health: None
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

        let mut start_of_map = 11;

        let mut map_name_char_vec = Vec::with_capacity(10);

        for byte in &bytes[11..] {
            if *byte == 0 {
                break;
            }

            map_name_char_vec.push(*byte as char);

            start_of_map += 1;
        }

        let map_name: String = map_name_char_vec.into_iter().collect();

        let mut objects: Vec<MapObject> = Vec::with_capacity(0);

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
            objects = chunks.map(|chunk| MapObject::from_bin(chunk)).collect();
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
            match map_object.player_spawn {
                true => Some(map_object.coords.truncate().truncate()),
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

    // Returns whether a collision took place, and the health of the wall (if it has health)
    pub fn collision(&mut self, other_object_coords: Vec2, other_object_size: Vec2, damage: f32, distance: f32, angle: Vec2) -> (bool, Option<(f32, Vec2)>) {
        // The collision function just iterates through each map object within the map, and runs the collide function within
        #[cfg(feature = "parallel")]
        let object = self.objects.par_iter_mut().enumerate()
            .find_any(|(_i, object)| {
                let c = object.collision(other_object_coords, other_object_size, distance, angle);
                c.0 || c.1

            });

        #[cfg(not(feature = "parallel"))]
        let object = self.objects.iter_mut().enumerate()
        .find(|(_i, object)| {
            let c = object.collision(other_object_coords, other_object_size, distance, angle);
            c.0 || c.1

        });

        let found_object = object.is_some();
        // The map object to remove if a player dies
        let mut object_to_remove = None;

        let health_and_coords = match object {
            Some((index, object)) => {
                if let Some(mut health) = &mut object.health {
                    // Damagable objects take damage
                    health = match health as i16 - damage as i16 <= 0 {
                        true => 0.0,
                        false => health - damage,
                    };
 
                    if health == 0.0 {
                        object_to_remove = Some(index);

                    }

                    Some((health, object.coords.xy()))
                } else {
                    None
                }
            }
            None => None,
        };

        if let Some(i) = object_to_remove {
            self.objects.remove(i);

        }

        (found_object, health_and_coords)
    }

    // Identical to collision, except it's a non-mutable reference so it's safe to use in a parallel iterator
    pub fn collision_no_damage(&self, other_object_coords: Vec2, other_object_size: Vec2, distance: f32, angle: Vec2) -> (bool, bool) {
        let map_collision = |object: &MapObject| object.collision(other_object_coords, other_object_size, distance, angle);
        
        // The collision function just iterates through each map object within the map, and runs the collide function within
        // Since this function is only used in par_for_each loops, we don't need extra parallelism
        #[cfg(not(feature = "parallel"))]
        // The collision only returns None if the Iterator is emtpy, which it never will be
        let collision = unsafe { self.objects.iter().map(map_collision).reduce(|old_coll, new_coll| (old_coll.0 || new_coll.0, old_coll.1 || new_coll.1)).unwrap_unchecked() };

        #[cfg(feature = "parallel")]
        let collision = self.objects.par_iter().map(map_collision).reduce(|| (false, false), |old_coll, new_coll| (old_coll.0 || new_coll.0, old_coll.1 || new_coll.1));

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