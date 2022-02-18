pub const SIZE_OF_31_PLAYERS: u32 = 8 * 31;

#[no_mangle]
#[used]
pub static mut PLAYER_MEM_BUFFER: [u8; 9] = [0; 9];

#[no_mangle]
#[used]
pub static mut CURRENT_PLAYER_HEALTH: f32 = 0.0;

#[no_mangle]
#[used]
pub static mut ENEMY_PLAYER_MEM_BUFFER: [u8; SIZE_OF_31_PLAYERS as usize] = [0; SIZE_OF_31_PLAYERS as usize];

// Unsafe extern functions
#[no_mangle]
pub extern fn enemy_player_mem_buffer_size() -> u32 {
    unsafe { ENEMY_PLAYER_MEM_BUFFER.len().try_into().unwrap() }

}

#[no_mangle]
pub extern fn enemy_player_mem_buffer_ptr() -> u32 {
    unsafe { ENEMY_PLAYER_MEM_BUFFER.as_ptr() as u32 }

}

#[no_mangle]
pub extern fn player_mem_buffer_ptr() -> u32 {
    unsafe { PLAYER_MEM_BUFFER.as_ptr() as u32 }

}

//TODO: Change this to a set_health fn
#[no_mangle]
pub extern fn player_health_buffer_ptr() -> u32 {
    unsafe { &mut CURRENT_PLAYER_HEALTH as *mut f32 as u32 }

}
