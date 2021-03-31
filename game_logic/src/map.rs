use crate::collision;
use crate::objects::{Rect, Color};
use serde::{Deserialize, Serialize};

//TODO: Probably should turn Map and MapObjects into traits, but since the game's geometry is so simple at the moment it really doesn't matter.

#[derive(Deserialize, Serialize)]
pub struct Map {
    pub objects: Vec<MapObject>,
    
    pub width: f32,
    pub height: f32,
}

impl Map {
    pub fn new(objects: Vec<MapObject>, dimensions: Option<[f32; 2]>) -> Map {
    
        let mut map_width: f32 = 0.0;
        let mut map_height: f32 = 0.0;
    
        if dimensions == None {
            for object in objects.iter() {
                let object = object.data;
            
                if dimensions == None && object.x + object.w > map_width{
                    map_width = object.x + object.w ;
                        
                }
                
                if dimensions == None && object.y + object.h > map_height {
                    map_height = object.y + object.h;
                        
                }
            }
            
        } else {
            map_width = dimensions.unwrap()[0];
            map_height = dimensions.unwrap()[1];
            
        }
    
        Map {
            objects,
            width: map_width,
            height: map_height,
            
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

    pub fn from_json_str(string: String) -> Map {
        let map: Map = serde_json::from_str(&string).unwrap();

        map

    }
}

#[derive(Deserialize, Serialize)]
pub struct MapObject {
    // It's x, y, width, and height
    pub data: Rect,
    pub color: Color,
    pub health: Option<u16>,
    
}

impl MapObject {
    pub fn new(data: Rect, color: Color, health: Option<u16>) -> MapObject {
        MapObject {
            data,
            color,
            // If the Option is None, then the wall cannot be destroyed
            health,
        }
        
    }
    
    // For now, all MapObjects will simply run the rectangle collision code from main.rs
    fn collision(&mut self, other_object: &Rect, damage: u16) -> bool{
        if collision(&self.data, other_object) {
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
