#![allow(unused_assignments)]
use std::f32::consts::PI;
use bevy::prelude::*;

use crate::map::Map;

// The returned value is the requested movement angle of the player

pub fn bounce(player_coords: Vec3, player_size: Vec2, current_direction: f32, map: &mut Map) -> f32 {
    let movement_radius = Vec3::new(7.0, 0.0, 0.0);

    if map.collision(player_coords, player_size * 2.0, 0) {
        if map.collision(player_coords + movement_radius, player_size, 0) {
            // Move west
            PI

        } else if map.collision(player_coords - movement_radius, player_size, 0) {
            // Move east
            0.0

        } else {
            current_direction

        }

    } else {
        current_direction

    }
    
}
