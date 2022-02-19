use bot_lib::*;
use glam::Vec2;

pub(crate) struct PlayerInfo {
    pub(crate) pos: Vec2,
    pub(crate) ability_charged: bool,
    pub(crate) using_ability: bool,

}

// Safe functions for use in bot
pub(crate) fn view_current_health() -> f32 {
    unsafe { CURRENT_PLAYER_HEALTH }

}

pub(crate) fn view_player_info() -> PlayerInfo {
    unsafe {
        PlayerInfo {
            pos: Vec2::new(
                f32::from_be_bytes(PLAYER_MEM_BUFFER[0..4].try_into().unwrap()),
                f32::from_be_bytes(PLAYER_MEM_BUFFER[4..8].try_into().unwrap())
            ),
            ability_charged: match PLAYER_MEM_BUFFER[8] {
                0 => false,
                _ => true,
            },
            using_ability: match PLAYER_MEM_BUFFER[9] {
                0 => false,
                _ => true,
            }

        }
    }
}


pub(crate) fn view_enemy_info() -> Vec<PlayerInfo> {
    // Maximum of 31 players
    let mut vector = Vec::with_capacity(12);

    for player_bytes in unsafe { ENEMY_PLAYER_MEM_BUFFER.chunks(8) } {
        // Skip this iteration if it's a null player
        if *player_bytes == [0; 8] {
            continue;

        }


        vector.push(PlayerInfo {
            pos: Vec2::new(
                f32::from_be_bytes(player_bytes[0..4].try_into().unwrap()),
                f32::from_be_bytes(player_bytes[4..8].try_into().unwrap())
            ),
            // When viewing enemy info, assume their ability is never charged to not give advantages
            ability_charged: false,
            using_ability: false,
        });

    }

    vector


}

