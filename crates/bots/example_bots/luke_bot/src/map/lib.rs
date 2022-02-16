use crate::map::mem::*;
use glam::Vec2;

// A sort of copy of MapObject from the map crate, minimized
pub(crate) struct MinMapObject {
    pub(crate) coords: Vec2,
    pub(crate) size: Vec2,
    pub(crate) health: Option<f32>,
}

pub(crate) fn view_map() -> Vec<MinMapObject> {
    let mut map_objects = Vec::with_capacity(unsafe { MAP_MEM_BUFFER.len() % 20 });

    for map_object_bytes in unsafe { MAP_MEM_BUFFER.chunks(20) } {
        if *map_object_bytes == [0; 20] {
            break;

        }

        map_objects.push(MinMapObject {
            coords: Vec2::new(
                f32::from_be_bytes(map_object_bytes[0..4].try_into().unwrap()), f32::from_be_bytes(map_object_bytes[4..8].try_into().unwrap())
            ),
            size: Vec2::new(
                f32::from_be_bytes(map_object_bytes[8..12].try_into().unwrap()), f32::from_be_bytes(map_object_bytes[12..16].try_into().unwrap())
            ),
            health: match map_object_bytes[16..20] == [0; 4] {
                true => None,
                false => Some(f32::from_be_bytes(map_object_bytes[16..20].try_into().unwrap())),
            }
        });

    }

    map_objects

}
