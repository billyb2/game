use bevy::prelude::*;
use crate::{Coords, Size};

use crc32fast::Hasher;

use lz_fear::framed::decompress_frame;

use std::convert::TryInto;

#[derive(Bundle, Copy, Clone)]
pub struct MapObject {
    pub coords: Coords,
    pub size: Size,
    pub color: Color,
    pub player_collidable: bool,
    pub player_spawn: bool,
    pub health: Option<u8>,

}

#[derive(Clone)]
pub  struct Map {
    pub objects: Vec<MapObject>,
    pub background_color: Color,
    pub size: Size,

}

impl Map {
    pub fn new(objects: Vec<MapObject>, size: [f32; 2], background_color: Color) -> Map {

        Map {
            objects,
            size: Size::new(size[0],  size[1]),
            background_color,

        }
    }

    pub fn from_bin(compressed_bytes: &[u8]) -> Map {
        //Decompress the map
        let mut bytes: Vec<u8> = decompress_frame(compressed_bytes).unwrap();

       //Unallocates all the extra memory
       bytes.shrink_to_fit();

        let map_width = slice_to_u32(&bytes[0..=3]);
        let map_height = slice_to_u32(&bytes[4..=7]);
        let background_color = Color::rgb_u8(bytes[8], bytes[9], bytes[10]);

        let mut objects: Vec<MapObject> = Vec::with_capacity(bytes.len() - 11);

        let mut i = 11;
        let mut crc32: u32 = 0;
        let mut data_end_index = 0;

        while i < bytes.len() - 22 {
            let x = (slice_to_u32(&bytes[i..=(i + 3)])) as f32;
            let y = (slice_to_u32(&bytes[(i + 4)..=(i + 7)])) as f32;
            let w = (slice_to_u32(&bytes[(i + 8)..=(i + 11)])) as f32;
            let h = (slice_to_u32(&bytes[(i + 12)..=(i + 15)])) as f32;

            objects.push(
                MapObject {
                    coords: Coords::new(x, y),
                    size: Size::new(w, h),
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

        Map::new(objects, [map_width as f32, map_height as f32], background_color)

    }

}

fn slice_to_u32(data: &[u8]) -> u32 {
    debug_assert!(data.len() == 4);

    let data_array: [u8; 4] = data.try_into().unwrap();

    u32::from_be_bytes(data_array)
}
