#![feature(core_intrinsics)]
#![feature(option_result_unwrap_unchecked)]
#![feature(stmt_expr_attributes)]

// A variety of (mostly math) functions that don't really fit anywhere, but are pretty useful
#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

use bevy::math::{Vec2, const_vec2};

use std::f32::consts::PI;
use std::convert::TryInto;
use std::intrinsics::*;

#[cfg(not(target_arch = "wasm32"))]
use std::net::UdpSocket;

//TODO: Maybe genericize the slice_to function?
#[inline]
pub fn slice_to_i32(data: &[u8]) -> i32 {
    let data_array: [u8; 4] = data.try_into().unwrap();
    i32::from_be_bytes(data_array)

}

#[inline]
pub fn slice_to_u32(data: &[u8]) -> u32 {
    let data_array: [u8; 4] = data.try_into().unwrap();
    u32::from_be_bytes(data_array)

}

#[inline]
pub fn slice_to_f32(data: &[u8]) -> f32 {
    let data_array: [u8; 4] = data.try_into().unwrap();
    f32::from_le_bytes(data_array)

}

pub fn get_angle(cx: f32, cy: f32, ex: f32, ey: f32) -> f32 {
    //let dy = unsafe { fsub_fast(ey, cy) };
    //let dx = unsafe { fsub_fast(ex, cx) };
    let dy = ey - cy;
    let dx = ex - cx;

    match dx != 0.0 {
        // Returns the angle in radians
        //true => unsafe { fdiv_fast(dy, dx) }.atan(),
        true => (dy / dx).atan(),
        false => match dy > 0.0 {
            true => {
                const HALF_PI: f32 = PI / 2.0;
                HALF_PI

            },
            false => PI,
        },
    }
    
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_available_port(ip: &str) -> Option<u16> {
    #[cfg(feature = "parallel")]
    return (8000..9000).into_iter().find(|port| port_is_available(ip, *port));

    #[cfg(not(feature = "parallel"))]
    return (8000..9000).into_par_iter().find(|port| port_is_available(ip, *port));
}

#[cfg(not(target_arch = "wasm32"))]
fn port_is_available(ip: &str, port: u16) -> bool {
    match UdpSocket::bind((ip, port)) {
        Ok(_) => {
            true
        }
        Err(_) => false,
    }
}

const TWO: Vec2 = const_vec2!([2.0; 2]);

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

pub fn collide(rect1_coords: Vec2, rect1_size: Vec2, rect2_coords: Vec2, rect2_size: Vec2, distance: f32, angle: Vec2) -> (bool, bool) {
    // A bounding box collision test between two rectangles
    // This code is partially stolen from https://github.com/bevyengine/bevy/blob/cf221f9659127427c99d621b76c8085c4860e2ef/crates/bevy_sprite/src/collide_aabb.rs
    // It basically just adjusts the rectangles before doing a rectangle-rectangle collision test
    
    // So what this code essentially does is it tries to move object 1 a few increments for a certain distance at a certain angle, until it reaches its destination

    let half_rect1_size = rect1_size / TWO; 
    let half_rect2_size = rect2_size / TWO;

    let rect2_min = rect2_coords - half_rect2_size;
    let rect2_max = rect2_coords + half_rect2_size;

    let distance = distance * angle;

    // If the distance is less than the size of the object, there's no point in doing so many computations when I could just calculate the object's final position collision
    if distance.cmple(rect1_size).all() {
        let new_rect1_coords = distance + rect1_coords;

        // The coords when moving only in the x direction, and only in the y direction
        let coords_cos_sin = (Vec2::new(new_rect1_coords[0], rect1_coords[1]), Vec2::new(rect1_coords[0], new_rect1_coords[1])) ;

        ({
            let rect1_min = coords_cos_sin.0 - half_rect1_size;
            let rect1_max = coords_cos_sin.0 + half_rect1_size;

            unlikely(rect1_min.cmple(rect2_max).all() && rect2_min.cmple(rect1_max).all())
            
        }, {
            let rect1_min = coords_cos_sin.1 - half_rect1_size;
            let rect1_max = coords_cos_sin.1 + half_rect1_size;

            unlikely(rect1_min.cmple(rect2_max).all() && rect2_min.cmple(rect1_max).all())

        })

    } else {
        let velocity = distance * angle;

        let (inv_entry, inv_exit) = if velocity.cmpgt(Vec2::ZERO).all() {
            let inv_entry = rect2_coords - (rect1_coords + rect1_size);
            let inv_exit = (rect2_coords + rect2_size) - rect1_coords;

            (inv_entry, inv_exit)

        } else {
            let (x_inv_entry, x_inv_exit) = match velocity[0] > 0.0 {
                true => ((rect2_coords[0] - (rect1_coords[0] + rect1_size[0])), (rect2_coords[0] + rect2_size[0]) - rect1_coords[0]),
                false => ((rect2_coords[0] + rect2_size[0]) - rect1_coords[0], (rect2_coords[0] - (rect1_coords[0] + rect1_size[0]))),
            };

            let (y_inv_entry, y_inv_exit) = match velocity[1] > 0.0 {
                true => ((rect2_coords[1] - (rect1_coords[1] + rect1_size[1])), (rect2_coords[1] + rect2_size[1]) - rect1_coords[1]),
                false => ((rect2_coords[1] + rect2_size[1]) - rect1_coords[1], (rect2_coords[1] - (rect1_coords[1] + rect1_size[1]))),
            };

            (Vec2::new(x_inv_entry, y_inv_entry), Vec2::new(x_inv_exit, y_inv_exit))

        };

        let (entry, exit) = match likely(velocity.cmpne(Vec2::ZERO).all()) {
            true => (inv_entry / velocity, inv_exit / velocity),
            false => {
                let (x_entry, x_exit) = match velocity[0] == 0.0 {
                    true => (f32::NEG_INFINITY, f32::INFINITY),
                    false => (inv_entry[0] / velocity[0], inv_exit[0] / velocity[0]),
                };

                let (y_entry, y_exit) = match velocity[1] == 0.0 {
                    true => (f32::NEG_INFINITY, f32::INFINITY),
                    false => (inv_entry[1] / velocity[1], inv_exit[1] / velocity[1]),
                };

                (Vec2::new(x_entry, y_entry), Vec2::new(x_exit, y_exit))
            }
        };

        let entry_time = f32::max(entry[0], entry[1]);
        let exit_time = f32::min(exit[0], exit[1]);

        // No collision
        if entry_time > exit_time || entry.cmplt(Vec2::ZERO).all() || entry.cmpgt(Vec2::ONE).any() { 
            /*let normalx = 0.0f; 
            let normaly = 0.0f; 
            return 1.0f; */

            (false, false)

        } else {
            // Calculate normal of collided surface
            let (normal_x, normal_y) = match entry[0] > entry[1] {
                true => match inv_entry[0] < 0.0 {
                    true => (1.0, 0.0),
                    false => (-1.0, 0.0),
                },
                false => match inv_entry[1] < 0.0 {
                    true => (0.0, 1.0),
                    false => (0.0, -1.0),
                }
            };

            (true, true)

        }

    }

}

pub fn collide_rect_circle(rect_coords: Vec2, rect_size: Vec2, circle_coords: Vec2, radius: f32) -> bool {
    let half_rect_size = rect_size / TWO;

    let delta = circle_coords - (rect_coords - half_rect_size).max(circle_coords.min(rect_coords + half_rect_size)); 

    // Sums delta squared
    (unsafe { let delta_squared = (delta * delta).to_array(); delta_squared.get_unchecked(0) + delta_squared.get_unchecked(1) })
    <= (radius / 2.0).powi(2)


}

pub fn out_of_bounds(rect_coords: Vec2, rect_size: Vec2, map_size: Vec2) -> bool {
    let half_rect_size = rect_size / TWO;

    let min = rect_coords - half_rect_size;
    let max = rect_coords + half_rect_size;

    unlikely(
        min.cmple(Vec2::ZERO).any() &&
        max.cmpge(map_size).any()
    )

}

/*#[inline]
pub fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into().unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}*/
