#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

use std::io::Read;

use bevy::prelude::*;

use crate::helper_functions::collide;
use crate::helper_functions::slice_to_u32;

use crc32fast::Hasher;
use lz4_flex::frame::FrameDecoder;

#[derive(Bundle, Copy, Clone)]
pub struct MapObject {
    pub coords: Vec3,
    pub size: Vec2,
    pub color: Color,
    pub collidable: bool,
    pub player_spawn: bool,
    pub health: Option<u8>,

}

#[derive(Clone)]
pub struct Map {
    pub objects: Vec<MapObject>,
    pub background_color: Color,
    pub size: Vec2,

}

impl MapObject {
    fn collision(&mut self, other_object_coords: Vec3, other_object_size: Vec2, damage: u8) -> bool {
        // Just runs a simple rectangle - rectangle collision function, if the given map object can be collided with
        if self.collidable && collide(self.coords, self.size, other_object_coords, other_object_size) {
            // Damagable objects take damage
            if self.health.is_some() {
                if self.health.unwrap() as i16 - damage as i16 <= 0 {
                    self.health = Some(0);

                } else {
                    self.health = Some(self.health.unwrap() - damage);

                }

            }

            true

        } else {
            false

        }

    }


}

impl Map {
    pub fn new(objects: Vec<MapObject>, size: [f32; 2], background_color: Color) -> Map {
        Map {
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

        // Just dropping the FrameDecoder to save a little bit of memroy
        std::mem::drop(decoder);


       //Unallocates all the extra capacity
       bytes.shrink_to_fit();

        // The first few bytes of the map are metadata, like the dimensions of the map, its background color, etc.
        let map_width = slice_to_u32(&bytes[0..=3]) * 5;
        let map_height = slice_to_u32(&bytes[4..=7]) * 5;
        let background_color = Color::rgb_u8(bytes[8], bytes[9], bytes[10]);

        let mut objects: Vec<MapObject> = Vec::with_capacity(bytes.len() - 11);

        let mut i = 11;
        let mut crc32: u32 = 0;
        let mut data_end_index = 0;

        // Iterates through each 22 byte binary map object
        while i < bytes.len() - 22 {
            let x = (slice_to_u32(&bytes[i..=(i + 3)])) as f32;
            let y = (slice_to_u32(&bytes[(i + 4)..=(i + 7)])) as f32;
            let width = (slice_to_u32(&bytes[(i + 8)..=(i + 11)])) as f32;
            let height = (slice_to_u32(&bytes[(i + 12)..=(i + 15)])) as f32;

            objects.push(
                MapObject {
                    // Gotta adjust for Bevy's coordinate system center being at (0, 0)
                    coords: Vec3::new(x, -y, 0.0) + Vec3::new(width / 2.0, -height / 2.0, 0.0),
                    size: Vec2::new(width, height),
                    player_spawn: matches!(bytes[(i + 16)], 255),
                    collidable: matches!(bytes[(i + 17)], 255),
                    // This will eventually become a real image, eventually :(
                    color: Color::rgba_u8(bytes[i + 18], bytes[i + 19], bytes[i + 20], bytes[i + 21]),
                    health: match bytes[i + 22] {
                        0 => None,
                        _ => Some(bytes[i + 22]),
                    },
                }
            );

            // Look for an entirely null map object, indicating the end of the map object data and the beginning of the CRC32
            if bytes[(i + 22)..=(i + 43)] == [0; 22] {
                crc32 = slice_to_u32(&bytes[(i + 44)..=(i + 47)]);
                data_end_index = i + 43;
                break;

            }

            i += 23;
        }

        //Deallocate any extra memory in objects
        objects.shrink_to_fit();

        if data_end_index == 0 {
            panic!("No CRC32 found, please check your map file");

        }

        // Performs a CRC32 hash of the file, and compares it to the CRC32 given
        let mut hasher = Hasher::new();
        hasher.update(&bytes[0..=data_end_index]);

        let checksum: u32 = hasher.finalize();

        if checksum == crc32 {
            println!("Verified map checksum!");

        } else {
            panic!("The map file is corrupted! (Checksums don't match)");

        }

        Map::new(objects, [map_width as f32, map_height as f32], background_color)

    }

     pub fn collision(&mut self, other_object_coords: Vec3, other_object_size: Vec2, damage: u8) -> bool {
        let mut i = 0;
        let mut collided = false;

        // The collision function just iterates throuhg each map object within the map, and runs the collide function within
        while i != self.objects.len() {
            if self.objects[i].collision(other_object_coords, other_object_size, damage) {
                if self.objects[i].health == Some(0) {
                    self.objects.remove(i);

                }

                collided = true;
                break;

            } else {
                i += 1;

            }
        }

        collided

     }

}

// This system just iterates through the map and draws each MapObject
pub fn draw_map(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, map: Res<Map>) {

    // Set the background color to the map's specified color
    commands.insert_resource(ClearColor((*map).background_color));

    let mut i = 0;

    while i != (*map).objects.len() {
        let map_coords = (*map).objects[i].coords;
        let map_object_size =  (*map).objects[i].size;
        let color = (*map).objects[i].color;

        //Either create a new material, or grab a currently existing one
        let color: Handle<ColorMaterial> = {
            let mut color_to_return = None;

            for (id, material_to_return) in materials.iter() {
                if color == material_to_return.color {
                    color_to_return = Some(materials.get_handle(id));

                }

            }


            if let Some(color) = color_to_return {
                color

            } else {
                materials.add(color.into())

            }
        };

        // Spawn a new map sprite
        commands
            .spawn_bundle(SpriteBundle {
                material: color.clone(),
                sprite: Sprite::new(map_object_size),
                transform: Transform {
                    translation: map_coords,
                    ..Default::default()
                },
                ..Default::default()
            });

        i += 1;
    }
}
