#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

use bevy::math::{Vec2, Vec3};

use std::f32::consts::PI;
use std::convert::TryInto;

#[cfg(feature = "native")]
use std::net::UdpSocket;

pub fn slice_to_u32(data: &[u8]) -> u32 {
    debug_assert!(data.len() == 4);

    let data_array: [u8; 4] = data.try_into().unwrap();

    u32::from_be_bytes(data_array)

}


pub fn get_angle(cx: f32, cy: f32, ex: f32, ey: f32) -> f32 {
    let dy = ey - cy;
    let dx = ex - cx;

    if dx != 0.0 {
        let d = dy / dx;

        // Returns the angle in radians
        d.atan()

    } else if dy > 0.0 {
            PI / 2.0

    }  else {
            PI

    }
}

#[cfg(feature = "native")]
pub fn get_available_port(ip: &str) -> Option<u16> {
    (8000..9000).find(|port| port_is_available(ip, *port))
}

#[cfg(feature = "native")]
fn port_is_available(ip: &str, port: u16) -> bool {
    match UdpSocket::bind((ip, port)) {
        Ok(_) => {
            true
        }
        Err(_) => false,
    }
}

// Part of the collision code is taken from Bevy

/*
MIT License

Copyright (c) 2020 Carter Anderson

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

pub fn collide(rect1_coords: Vec3, rect1_size: Vec2, rect2_coords: Vec3, rect2_size: Vec2) -> bool {
    // A bounding box collision test between two rectangles
    // This code is partially stolen from https://github.com/bevyengine/bevy/blob/cf221f9659127427c99d621b76c8085c4860e2ef/crates/bevy_sprite/src/collide_aabb.rs
    // It basically just adjusts the rectangles before doing a rectangle-rectangle collision test

    let a_min = rect1_coords.truncate() - rect1_size / 2.0;
    let a_max = rect1_coords.truncate() + rect1_size / 2.0;

    let b_min = rect2_coords.truncate() - rect2_size / 2.0;
    let b_max = rect2_coords.truncate() + rect2_size / 2.0;

    {
        a_min.x < b_max.x
        && a_max.x > b_min.x
        && a_min.y < b_max.y
        && a_max.y > b_min.y

    }

}

pub fn out_of_bounds(rect_coords: Vec3, rect_size: Vec2, map_size: Vec2) -> bool {
    let a_min = rect_coords.truncate() - rect_size / 2.0;
    let a_max = rect_coords.truncate() + rect_size / 2.0;

    {
        a_min.x <= 0.0
        // Gotta make the y coordinates negative due to the funky way Bevy uses coordinates
        || -a_min.y <= 0.0
        || a_max.x >= map_size.x
        || -a_max.y >= map_size.y

    }


}
