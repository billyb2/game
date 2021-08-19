// A variety of (mostly math) functions that don't really fit anywhere, but are pretty useful
#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

use bevy::math::Vec2;

use std::f32::consts::PI;
use std::convert::TryInto;
use std::intrinsics::*;

#[cfg(feature = "native")]
use std::net::UdpSocket;

use core_simd::*;

#[inline]
pub fn slice_to_u32(data: &[u8]) -> u32 {
    debug_assert!(data.len() == 4);

    let data_array: [u8; 4] = data.try_into().unwrap();

    u32::from_be_bytes(data_array)

}

pub fn get_angle(cx: f32, cy: f32, ex: f32, ey: f32) -> f32 {
    let dy = unsafe { fsub_fast(ey, cy) };
    let dx = unsafe { fsub_fast(ex, cx) };
    if dx != 0.0 {
        // Returns the angle in radians
        unsafe { fdiv_fast(dy, dx) }.atan()

    } else if dy > 0.0 {
            unsafe { fdiv_fast(PI, 2.0) }

    }  else {
            PI

    }
}

#[cfg(feature = "native")]
pub fn get_available_port(ip: &str) -> Option<u16> {
    #[cfg(feature = "parallel")]
    return (8000..9000).into_iter().find(|port| port_is_available(ip, *port));

    #[cfg(not(feature = "parallel"))]
    return (8000..9000).into_par_iter().find(|port| port_is_available(ip, *port));
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

const TWO: f32x2 = f32x2::splat(2.0);

pub fn collide(rect1_coords: f32x2, rect1_size: Vec2, rect2_coords: Vec2, rect2_size: Vec2, distance: f32, angle: f32x2) -> (bool, bool) {
    // A bounding box collision test between two rectangles
    // This code is partially stolen from https://github.com/bevyengine/bevy/blob/cf221f9659127427c99d621b76c8085c4860e2ef/crates/bevy_sprite/src/collide_aabb.rs
    // It basically just adjusts the rectangles before doing a rectangle-rectangle collision test
    
    // So what this code essentially does is it tries to move object 1 a few increments for a certain distance at a certain angle, until it reaches its destination
    let rect2_coords = f32x2::from_array(rect2_coords.to_array());

    let rect1_size = f32x2::from_array(rect1_size.to_array());

    let half_rect1_size = {
        rect1_size / TWO

    };

    let half_rect2_size = {
        let rect2_size = f32x2::from_array(rect2_size.to_array());
        rect2_size / TWO

    };

    let rect2_min = rect2_coords - half_rect2_size;
    let rect2_max = rect2_coords + half_rect2_size;


    if distance != 0.0 {
        let a_size_f32 = rect1_size.horizontal_product().sqrt();
        let interval_size = distance / a_size_f32;
        let num_of_iters = (distance / interval_size).ceil() as u32;

        let rect1_coords = f32x2::from_array(rect1_coords.to_array());

        let collision = |i: u32| -> (bool, bool) {
            let distance = f32x2::splat(interval_size * i as f32);
            
            let rect1_coords = distance.mul_add(angle, rect1_coords);

            let rect1_min = rect1_coords - half_rect1_size;
            let rect1_max = rect1_coords + half_rect1_size;

            // Check for collision
            (
                unlikely(rect1_min.lanes_le(rect2_max) == mask32x2::splat(true)),
                unlikely(rect2_min.lanes_le(rect1_max) == mask32x2::splat(true))
            )

        };

        // Tries to find whether the x coordinate collides or if the y coordinate collides
        (1..num_of_iters).into_iter().map(collision).fold((false, false), |old_coll, new_coll| (old_coll.0 || new_coll.0, old_coll.1 || new_coll.1))


    } else {
        let distance = f32x2::splat(distance);

        let mut rect1_coords = f32x2::from_array(rect1_coords.to_array());
        rect1_coords = distance.mul_add(angle, rect1_coords);

        let rect1_min = rect1_coords - half_rect1_size;
        let rect1_max = rect1_coords + half_rect1_size;

        // Check for collision
        (
            rect1_min.lanes_le(rect2_max) == mask32x2::splat(true),
            rect2_min.lanes_le(rect1_max) == mask32x2::splat(true)
        )

    }

}

pub fn collide_rect_circle(rect_coords: Vec2, rect_size: Vec2, circle_coords: f32x2, radius: f32) -> bool {
    let rect_coords = f32x2::from_array(rect_coords.to_array());

    let half_rect_size = f32x2::from_array(rect_size.to_array()) / TWO;

    let delta = circle_coords - (rect_coords - half_rect_size).max(circle_coords.min(rect_coords + half_rect_size)); 

    (delta * delta).horizontal_sum() <= (radius / 2.0).powi(2)


}

pub fn out_of_bounds(rect_coords: f32x2, rect_size: Vec2, map_size: Vec2) -> bool {
    const ZERO: f32x2 = f32x2::splat(0.0);

    let half_rect_size = f32x2::from_array(rect_size.to_array()) / TWO;

    let min = rect_coords - half_rect_size;
    let max = rect_coords + half_rect_size;

    let map_size = f32x2::from_array(map_size.to_array());

    unlikely(
        min.lanes_le(ZERO) == mask32x2::splat(true) &&
        max.lanes_ge(map_size) == mask32x2::splat(true)
    )



}

#[inline]
pub fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into().unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}