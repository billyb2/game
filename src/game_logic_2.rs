use crate::{Coords, Size};

pub fn collision (rect1_coords: &Coords, rect1_size: &Size, rect2_coords: &Coords, rect2_size: &Size) -> bool {
    // A bounding box collision test between two rectangles
    {
        rect1_coords.x < rect2_coords.x + rect2_size.w &&
        rect1_coords.x + rect1_size.w > rect2_coords.x &&
        rect1_coords.y < rect2_coords.y + rect2_size.h &&
        rect1_coords.y + rect1_size.h > rect2_coords.y
    }
}
