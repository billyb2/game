mod player;
mod map;

use glam::Vec2;
use std::sync::Mutex;

use player::{
    PlayerInfo,
    view_player_info,
    view_enemy_info,
    view_current_health,
};

use map::{
    MinMapObject,
    view_map,
};

// DO NOT ACESS ANY MEMORY BUFFERS DIRECTLY!!!!! PLEASE USE THE view_*_info FUNCTIONS FOR SAFE INTERFACES!!!!!!!!
// DO NOT USE ANY OF THE BELOW FUNCTIONS!!!!!
pub use player::mem::{
    enemy_player_mem_buffer_ptr,
    enemy_player_mem_buffer_size,
    player_health_buffer_ptr,
    player_mem_buffer_ptr,
};
pub use map::mem::{
    map_mem_buffer_ptr,
    map_mem_buffer_size,
};

static mut LAST_HEALTH: f32 = 100.0;
static mut INTERNAL_ANGLE: f32 = 0.0;

#[no_mangle]
pub extern fn new() -> u32 {
    // The first byte is the ability the player wants to use
    // The second byte is the gun the player wants to use
    // The third byte is the player's perk
    let mut request_bytes: [u8; 4] = [0; 4];

    // The cloak ability
    request_bytes[0] = 5;
    // The shotgun
    request_bytes[1] = 1;
    // The heavy armor perk
    request_bytes[2] = 1;

    u32::from_be_bytes(request_bytes)
}

#[no_mangle]
pub unsafe extern fn action_info() -> u64 {
    // The first 4 bytes are the angle (f32)
    // The 5th byte is whether or not the player is dashing (0 for false, anything else for true)
    // The 6th byte is whether or not the player should move at all (0 for false, anything else for true)
    // The 7th byte is whether or not the player should use their ability (0 for false, anything else for true)
    // The 8th byte is whether or not the player should shoot their gun (0 for false, anything else for true)
    let mut movement_info_bytes: [u8; 8] = [0; 8];

    let bot = view_player_info();
    let enemy_players = view_enemy_info();
    let enemy_players_iter = enemy_players.iter();
    let mut closest_distance = f32::MAX;
    let mut closest_angle = 0.0;


    if enemy_players_iter.len() > 0 {
        for player in enemy_players_iter {
            if bot.pos.distance(player.pos) < closest_distance {
                closest_distance = bot.pos.distance(player.pos);
                closest_angle = calc_angle(bot.pos, player.pos);
            }
        }

        INTERNAL_ANGLE = closest_angle;

        set_angle(closest_angle, &mut movement_info_bytes);

        let should_move_bool = closest_distance <= 400.0 && closest_distance >= 300.0;

        set_should_move(should_move_bool, &mut movement_info_bytes);

        u64::from_be_bytes(movement_info_bytes)

    } else {
        u64::from_be_bytes(movement_info_bytes)
        // end check now since there are no players
    }
}

#[no_mangle]
pub extern fn direction_info() -> f32 {
    unsafe { INTERNAL_ANGLE }
}

fn set_angle(angle: f32, bytes: &mut [u8]) {
    bytes[0..4].copy_from_slice(&angle.to_be_bytes());
}

fn set_should_dash(should_dash: bool, bytes: &mut [u8]) {
    bytes[4] = match should_dash {
        false => 0,
        true => 1,
    }
}

fn set_should_move(should_move: bool, bytes: &mut [u8]) {
    bytes[5] = match should_move {
        false => 0,
        true => 1,
    }
}

fn set_should_use_ability(should_use_ability: bool, bytes: &mut [u8]) {
    bytes[6] = match should_use_ability {
        false => 0,
        true => 1,
    }
}

fn set_should_shoot(should_shoot: bool, bytes: &mut [u8]) {
    bytes[7] = match should_shoot {
        false => 0,
        true => 1,
    }
}



// The player's name as a null terminated string
pub static NAME: &'static str = "Amogus Bot\0";

#[no_mangle]
pub extern fn bot_name() -> u32 {
    NAME.as_ptr() as u32
}

pub fn calc_angle(first_object: Vec2, second_object: Vec2) -> f32 {
    let d = second_object - first_object;
    f32::atan2(d.y, d.x)

}
