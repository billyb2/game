use crate::collision;

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,

}

impl Rect{
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect{
            x,
            y,
            w,
            h,

        }

    }


}

pub struct Point2 {
    pub x: f32,
    pub y: f32,

}


#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
    /// Red component
    pub r: f32,
    /// Green component
    pub g: f32,
    /// Blue component
    pub b: f32,
    /// Alpha component
    pub a: f32,
}

impl Color {
    /// Create a new `Color` from four `f32`'s in the range `[0.0-1.0]`
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }

    /// Create a new `Color` from four `u8`'s in the range `[0-255]`
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color::from((r, g, b, a))
    }

    /// Create a new `Color` from three u8's in the range `[0-255]`,
    /// with the alpha component fixed to 255 (opaque)
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color::from((r, g, b))
    }

    /// Return a tuple of four `u8`'s in the range `[0-255]` with the `Color`'s
    /// components.
    pub fn to_rgba(self) -> (u8, u8, u8, u8) {
        self.into()
    }

    /// Return a tuple of three `u8`'s in the range `[0-255]` with the `Color`'s
    /// components.
    pub fn to_rgb(self) -> (u8, u8, u8) {
        self.into()
    }

    /// Convert a packed `u32` containing `0xRRGGBBAA` into a `Color`
    pub fn from_rgba_u32(c: u32) -> Color {
        let c = c.to_be_bytes();

        Color::from((c[0], c[1], c[2], c[3]))
    }

    /// Convert a packed `u32` containing `0x00RRGGBB` into a `Color`.
    /// This lets you do things like `Color::from_rgb_u32(0xCD09AA)` easily if you want.
    pub fn from_rgb_u32(c: u32) -> Color {
        let c = c.to_be_bytes();

        Color::from((c[1], c[2], c[3]))
    }

    /// Convert a `Color` into a packed `u32`, containing `0xRRGGBBAA` as bytes.
    pub fn to_rgba_u32(self) -> u32 {
        let (r, g, b, a): (u8, u8, u8, u8) = self.into();

        u32::from_be_bytes([r, g, b, a])
    }

    /// Convert a `Color` into a packed `u32`, containing `0x00RRGGBB` as bytes.
    pub fn to_rgb_u32(self) -> u32 {
        let (r, g, b, _a): (u8, u8, u8, u8) = self.into();

        u32::from_be_bytes([0, r, g, b])
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    /// Convert a `(R, G, B, A)` tuple of `u8`'s in the range `[0-255]` into a `Color`
    fn from(val: (u8, u8, u8, u8)) -> Self {
        let (r, g, b, a) = val;
        let rf = (f32::from(r)) / 255.0;
        let gf = (f32::from(g)) / 255.0;
        let bf = (f32::from(b)) / 255.0;
        let af = (f32::from(a)) / 255.0;
        Color::new(rf, gf, bf, af)
    }
}

impl From<(u8, u8, u8)> for Color {
    /// Convert a `(R, G, B)` tuple of `u8`'s in the range `[0-255]` into a `Color`,
    /// with a value of 255 for the alpha element (i.e., no transparency.)
    fn from(val: (u8, u8, u8)) -> Self {
        let (r, g, b) = val;
        Color::from((r, g, b, 255))
    }
}

impl From<[f32; 4]> for Color {
    /// Turns an `[R, G, B, A] array of `f32`'s into a `Color` with no format changes.
    /// All inputs should be in the range `[0.0-1.0]`.
    fn from(val: [f32; 4]) -> Self {
        Color::new(val[0], val[1], val[2], val[3])
    }
}

impl From<(f32, f32, f32)> for Color {
    /// Convert a `(R, G, B)` tuple of `f32`'s in the range `[0.0-1.0]` into a `Color`,
    /// with a value of 1.0 to for the alpha element (ie, no transparency.)
    fn from(val: (f32, f32, f32)) -> Self {
        let (r, g, b) = val;
        Color::new(r, g, b, 1.0)
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    /// Convert a `(R, G, B, A)` tuple of `f32`'s in the range `[0.0-1.0]` into a `Color`
    fn from(val: (f32, f32, f32, f32)) -> Self {
        let (r, g, b, a) = val;
        Color::new(r, g, b, a)
    }
}

impl From<Color> for (u8, u8, u8, u8) {
    /// Convert a `Color` into a `(R, G, B, A)` tuple of `u8`'s in the range of `[0-255]`.
    fn from(color: Color) -> Self {
        let r = (color.r * 255.0) as u8;
        let g = (color.g * 255.0) as u8;
        let b = (color.b * 255.0) as u8;
        let a = (color.a * 255.0) as u8;
        (r, g, b, a)
    }
}

impl From<Color> for (u8, u8, u8) {
    /// Convert a `Color` into a `(R, G, B)` tuple of `u8`'s in the range of `[0-255]`,
    /// ignoring the alpha term.
    fn from(color: Color) -> Self {
        let (r, g, b, _) = color.into();
        (r, g, b)
    }
}


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
}

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
