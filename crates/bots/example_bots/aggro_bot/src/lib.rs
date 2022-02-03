mod player;
mod map;

use glam::Vec2;

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
pub extern fn action_info() -> u64 {
    // The first 4 bytes are the angle (f32)
    // The 5th byte is whether or not the player is dashing (0 for false, anything else for true)
    // The 6th byte is whether or not the player should move at all (0 for false, anything else for true)
    // The 7th byte is whether or not the player should use their ability (0 for false, anything else for true)
    // The 8th byte is whether or not the player should shoot their gun (0 for false, anything else for true)
    let mut movement_info_bytes: [u8; 8] = [0; 8];

    let bot = view_player_info();
    let enemy_players = view_enemy_info();

    if enemy_players.len() > 0 {
        let enemy_player = enemy_players.iter().fold(Vec2::ZERO, |target, potential_target| {
            let potential_target_distance = potential_target.pos.distance(bot.pos);
            let target_distance = target.distance(bot.pos);

            match potential_target_distance < target_distance {
                true => potential_target.pos,
                false => target,
            }

        });

        let distance = bot.pos.distance(enemy_player);
        set_should_dash(false, &mut movement_info_bytes);

        unsafe { INTERNAL_ANGLE = calc_angle(bot.pos, enemy_player) };

        // Only run towards players if they're relatively close
        match distance >= 350.0 && distance <= 450.0 {
            true => {
                set_angle(unsafe { INTERNAL_ANGLE }, &mut movement_info_bytes);
                set_should_move(true, &mut movement_info_bytes);
            },
            false => set_should_move(false, &mut movement_info_bytes),

        }

    // If there are no living players, spin in a circle to flex
    } else {
        unsafe { INTERNAL_ANGLE += 0.2 };

        set_should_move(false, &mut movement_info_bytes);
        set_angle(unsafe{INTERNAL_ANGLE}, &mut movement_info_bytes);

    }

    set_should_shoot(true, &mut movement_info_bytes);
    u64::from_be_bytes(movement_info_bytes)

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
pub static NAME: &'static str = "Aggro Bot\0";

#[no_mangle]
pub extern fn bot_name() -> u32 {
    NAME.as_ptr() as u32
}

pub fn calc_angle(first_object: Vec2, second_object: Vec2) -> f32 {
    let d = second_object - first_object;
    f32::atan2(d.y, d.x)

}
