use bevy::prelude::*;
use crate::{Coords, Size};

use crc32fast::Hasher;

use lz_fear::framed::decompress_frame;

use std::convert::TryInto;

#[derive(Bundle, Copy, Clone)]
pub struct MapObject {
    pub coords: Coords,
    pub color: Color,
    pub player_collidable: bool,
    pub player_spawn: bool,
    pub health: Option<u8>,

}

impl MapObject {
    pub fn new(x: f32, y: f32, color: Color, player_collidable: bool, player_spawn: bool, health: Option<u8>) -> MapObject {
        MapObject {
            coords: Coords::new(x, y),
            color,
            player_collidable,
            player_spawn,
            health,

        }

    }
}

#[derive(Clone)]
pub  struct Map {
    pub objects: Vec<MapObject>,
    pub size: Size,

}

impl Map {
    pub fn new(objects: Vec<MapObject>, size: Option<[f32; 2]>) -> Map {

        Map {
            objects,
            size: Size {
                w: size.unwrap()[0],
                h: size.unwrap()[1],

            }

        }
    }

    pub fn from_bin(compressed_bytes: &[u8]) -> Map {
        //Decompress the map
        let mut bytes: Vec<u8> = decompress_frame(compressed_bytes).unwrap();

       //Unallocates all the extra memory
       bytes.shrink_to_fit();

        let width = slice_to_u32(&bytes[0..=3]);
        let height = slice_to_u32(&bytes[4..=7]);

        let mut objects: Vec<MapObject> = Vec::with_capacity(bytes.len() - 8);

        let mut i = 8;
        let mut crc32: u32 = 0;
        let mut data_end_index = 0;

        while i < bytes.len() - 22 {
            let x = (slice_to_u32(&bytes[i..=(i + 3)])) as f32;
            let y = (slice_to_u32(&bytes[(i + 4)..=(i + 7)])) as f32;

            objects.push(
                MapObject {
                    coords: Coords::new(x, y),
                    player_spawn: !matches!(bytes[(i + 16)], 0),
                    player_collidable: !matches!(bytes[(i + 17)], 0),
                    color: Color::rgba_u8(bytes[i + 18], bytes[i + 19], bytes[i + 20], bytes[i + 21]),
                    health: match bytes[i + 22] {
                        0 => None,
                        _ => Some(bytes[i + 22]),
                    },
                }
            );

            // Look for an entirely null map object, indicating the end of the data and the beginning of the CRC32
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

        let mut hasher = Hasher::new();
        hasher.update(&bytes[0..=data_end_index]);

        let checksum: u32 = hasher.finalize();

        if checksum == crc32 {
            println!("Verified map checksum!");

        } else {
            panic!("The map file is corrupted! (Checksums don't match)");

        }

        Map::new(objects, Some([width as f32, height as f32]))

    }

    //TODO: Use the include_bytes! macro

}

fn slice_to_u32(data: &[u8]) -> u32 {
    debug_assert!(data.len() == 4);

    let data_array: [u8; 4] = data.try_into().unwrap();

    u32::from_be_bytes(data_array)
}
