use ggez::graphics::{Color, Rect};
//TODO: Probably should turn Map and MapObjects into traits, but since the game's geometry is so simple at the moment it really doesn't matter.

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
    
    pub fn collision(&self, other_object: &Rect) -> bool {
        let mut collided = false;
        
        // Basically, every object checks if it's collided with the object in question
        // It's done this way to allow for customization of how collisions work
        for object in &self.objects {
            if object.collision(other_object) {
                collided = true;
                break;
                
            }
            
        }
        
        collided
    }
}

pub struct MapObject {
    // It's x, y, width, and height
    pub data: Rect,
    pub color: Color,
    
}

impl MapObject {
    pub fn new(data: Rect, color: Color) -> MapObject {
        MapObject {
            data,
            color,
        }
        
    }
    
    // For now, all MapObjects will simply run the rectangle collision code from main.rs
    fn collision(&self, other_object: &Rect) -> bool{
        crate::game_logic::collision(&self.data, other_object)
        
    }
}
