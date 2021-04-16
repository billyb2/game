use std::convert::TryInto;
use std::io::Read;
use lz4_flex::frame::decompress::FrameDecoder;

pub fn slice_to_u32(data: &[u8]) -> u32 {
    debug_assert!(data.len() == 4);

    let data_array: [u8; 4] = data.try_into().unwrap();

    u32::from_be_bytes(data_array)
}

pub fn decompress_frame(input: &[u8]) -> Result<Vec<u8>, lz4_flex::frame::Error> {
    let mut de = FrameDecoder::new(input);
    let mut out = Vec::new();

    de.read_to_end(&mut out)?;

    Ok(out)
}
