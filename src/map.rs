use crate::collision;
use crate::objects::{Rect, Color};

use crc32fast::Hasher;

use lz_fear::framed::decompress_frame;

use std::convert::TryInto;

//TODO: Probably should turn Map and MapObjects into traits, but since the game's geometry is so simple at the moment it really doesn't matter.

#[derive(Debug)]
pub struct Map {
    pub objects: Vec<MapObject>,
    
    pub width: f32,
    pub height: f32,
}

impl Map {
    pub fn new(objects: Vec<MapObject>, dimensions: Option<[f32; 2]>) -> Map {

        Map {
            objects,
            width: dimensions.unwrap()[0],
            height: dimensions.unwrap()[1],
            
        }
    }
    
    pub fn collision(&mut self, other_object: &Rect, damage: u16) -> bool {
        let mut collided = false;
        
        // Basically, every object checks if it's collided with the object in question
        // It's done this way to allow for customization of how collisions work
        let mut i = 0;

        while i != self.objects.len() {
            if self.objects[i].collision(other_object, damage) {
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
            objects.push(
                MapObject {
                    data: Rect {
                        x: (slice_to_u32(&bytes[i..=(i + 3)])) as f32,
                        y: (slice_to_u32(&bytes[(i + 4)..=(i + 7)])) as f32,
                        w: (slice_to_u32(&bytes[(i + 8)..=(i + 11)])) as f32,
                        h: (slice_to_u32(&bytes[(i + 12)..=(i + 15)])) as f32,
                    },
                    player_spawn: !matches!(bytes[(i + 16)], 0),
                    player_collidable: !matches!(bytes[(i + 17)], 0),
                    color: Color::from_rgba(bytes[i + 18], bytes[i + 19], bytes[i + 20], bytes[i + 21]),
                    health: match bytes[i + 22] {
                        0 => None,
                        _ => Some(bytes[i + 22] as u16),
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
}

#[derive(Debug)]
pub struct MapObject {
    // It's x, y, width, and height
    pub data: Rect,
    pub color: Color,
    pub player_collidable: bool,
    pub player_spawn: bool,
    pub health: Option<u16>,
    
}

impl MapObject {
    pub fn new(data: Rect, color: Color, health: Option<u16>, player_spawn: bool, player_collidable: bool) -> MapObject {
        MapObject {
            data,
            color,
            player_spawn,
            player_collidable,
            // If the Option is None, then the wall cannot be destroyed
            health,
        }
        
    }
    
    // For now, all MapObjects will simply run the rectangle collision code from main.rs
    fn collision(&mut self, other_object: &Rect, damage: u16) -> bool{
        if collision(&self.data, other_object) && self.player_collidable {
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

fn slice_to_u32(data: &[u8]) -> u32 {
    debug_assert!(data.len() == 4);

    let data_array: [u8; 4] = data.try_into().unwrap();

    u32::from_be_bytes(data_array)
}
