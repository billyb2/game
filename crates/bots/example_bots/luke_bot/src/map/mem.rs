// Up to 10kb for map stuff, which would be a very complex map
// For reference, map2 takes up around 3kb when converted to a binary
pub(crate) static mut MAP_MEM_BUFFER: [u8; 10_000] = [0; 10_000];

// Unsafe extern functions
#[no_mangle]
pub extern fn map_mem_buffer_size() -> u32 {
    unsafe { MAP_MEM_BUFFER.len().try_into().unwrap() }

}

#[no_mangle]
pub extern fn map_mem_buffer_ptr() -> u32 {
    unsafe { MAP_MEM_BUFFER.as_ptr() as u32 }

}
