#![feature(stmt_expr_attributes)]
#![feature(adt_const_params)]

// A variety of (mostly math) functions that don't really fit anywhere, but are pretty useful
#![deny(clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(incomplete_features)]

use bevy::math::{Vec2, const_vec2};

use std::f32::consts::PI;
use std::convert::TryInto;

#[cfg(not(target_arch = "wasm32"))]
use std::net::UdpSocket;

#[cfg(feature = "graphics")]
pub mod graphics;

#[cfg(feature = "graphics")]
pub use graphics::*;

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

// Store an f32 as a u128, completely in safe Rust
// It just stores the 4 f32 bytes as the last 4 bytes of a u128
#[inline]
pub fn f32_u8_to_u128(data_f32: f32, data_u8: (u8, u8)) -> u128 {
    let mut slice = [0; 16];
    let (left, right) = slice.split_at_mut(12);

    right.copy_from_slice(&data_f32.to_be_bytes());
    left[10] = data_u8.0;
    left[11] = data_u8.1;

    u128::from_be_bytes(slice)
}

#[inline(always)]
// Just the reverse of f32_to_u128
pub fn u128_to_f32_u8(data: u128) -> (f32, (u8, u8)) {
    let bytes = data.to_be_bytes();

    (f32::from_be_bytes(bytes[12..16].try_into().unwrap()), (bytes[10], bytes[11]))
}

pub fn get_angle(cx: f32, cy: f32, ex: f32, ey: f32) -> f32 {
    //let dy = unsafe { fsub_fast(ey, cy) };
    //let dx = unsafe { fsub_fast(ex, cx) };
    let dy = ey - cy;
    let dx = ex - cx;

    let mut angle = match dx != 0.0 {
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
    };

    angle = match cx < ex {
        true => angle - PI,
        false => angle,

    };

    angle
    
}

const TWO: Vec2 = const_vec2!([2.0; 2]);

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

pub fn aabb_check(rect1_coords: Vec2, rect1_size: Vec2, rect2_coords: Vec2, rect2_size: Vec2) -> bool {
    let half_rect1_size = rect1_size / TWO;
    let half_rect2_size = rect2_size / TWO;

    let a_min = rect1_coords - half_rect1_size;
    let a_max = rect1_coords + half_rect1_size;

    let b_min = rect2_coords - half_rect2_size;
    let b_max = rect2_coords + half_rect2_size;

    !(
        a_max.cmple(a_min).any() ||
        b_max.cmple(b_min).any()
    )

}

pub fn mean_angle(angles: &[f32]) -> f32 {
    let length: f32 = angles.len() as f32;
    let cos_mean: f32 = angles.iter().fold(0.0, |sum, i| sum + i.cos()) / length;
    let sin_mean: f32 = angles.iter().fold(0.0, |sum, i| sum + i.sin()) / length;
    (sin_mean).atan2(cos_mean)
}

#[cfg(not(target_arch = "wasm32"))]
#[inline]
pub fn get_available_port(ip: &str) -> Option<u16> {
    (8000..9000).into_iter().find(|port| port_is_available(ip, *port))
}

#[cfg(not(target_arch = "wasm32"))]
#[inline(always)]
fn port_is_available(ip: &str, port: u16) -> bool {
    UdpSocket::bind((ip, port)).is_ok()
}