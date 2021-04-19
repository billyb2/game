use std::f32::consts::PI;

use std::convert::TryInto;
use std::io::Read;
use lz4_flex::frame::FrameDecoder;

pub fn slice_to_u32(data: &[u8]) -> u32 {
    debug_assert!(data.len() == 4);

    let data_array: [u8; 4] = data.try_into().unwrap();

    u32::from_be_bytes(data_array)

}

pub fn decompress_lz4_frame(input: &[u8]) -> Result<Vec<u8>, lz4_flex::frame::Error> {
    let mut de = FrameDecoder::new(input);
    let mut out = Vec::with_capacity(1_000);

    de.read_to_end(&mut out)?;

    Ok(out)

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
