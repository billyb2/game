#![deny(clippy::all)]
#![allow(unused_assignments)]
#![allow(clippy::type_complexity)]

// Some definitions for aspects of the game
// Don't worry about this file unless you're dealing with how bots are defined
use bevy::prelude::*;

use map::*;
use helper_functions::*;
use game_types::*;
use game_types::player_attr::*;

use rapier2d::prelude::*;
use rapier2d::na::Vector2;

/// The returned value is the requested movement angle of the player in radians
pub struct Angle(pub f32);
/// Whether or not a player is dashing
pub struct Dashing(pub bool);

pub struct BotWrapper(pub Box<dyn Bot + Send + Sync>);

#[derive(Clone)]
/// Like the player struct with certain attributes not being shown so that bots have to play with the same information that normal players do
pub struct TruncatedPlayer {
    pub pos: Vec2,
    pub id: PlayerID,
}

impl TruncatedPlayer {
    pub(crate) fn empty(id: PlayerID) -> Self {
        TruncatedPlayer {
            pos: Vec2::ZERO,
            id,
        }
    }
}

pub trait Bot {
    fn new(map: &Map, players: &Vec<TruncatedPlayer>, my_player_id: PlayerID) -> Self where Self: Sized;
    /// For updating the internals of the bot depending on changes in the map
    fn update_map_info(&mut self, map: &Map);
    /// For updating the internals of the bot depending on changes in the player's positions, abilities, weapons, etc.
    fn update_player_info(&mut self, players: &Vec<TruncatedPlayer>);
    /// Any final updates to itself that the bot wants to do
    fn misc_update(&mut self);
    /// Returns the angle at which the player will move
    /// While the player can say whether or not they're dashing, it isn't guaranteed that the server will let them (if, say, their cooldown isn't finished)
    fn movement(&self) -> Option<(Angle, Dashing)>;
    /// The direction which the player is facing in
    fn update_direction(&self) -> Option<Angle>;
    /// For some abilities, they need extra information besides whether or not they'll be moving or not
    fn use_ability(&self) -> Option<Angle>;
    fn should_shoot(&self) -> bool;

}

pub fn handle_bots(mut bots: Query<(&mut Transform, &PlayerID, Option<&mut BotWrapper>, &RigidBodyHandle, &Health, &Model, &TimeSinceStartReload, &Damage, &MaxDistance, &Speed, &TimeSinceStartReload, &TimeSinceLastShot)>, mut rigid_body_set: ResMut<RigidBodySet>, map_crc32: Res<MapCRC32>, maps: Res<Maps>, mut shoot_event: EventWriter<ShootEvent>) {
    // Generate the list of TruncatedPlayer by looping over the bots list initially
    let players: Vec<TruncatedPlayer> = bots.iter_mut().map(|(transform, id, _bw, _rgb, _h, _model, _r_timer, _d, _md, _pjs, _ttsr, _ttls)| {
        TruncatedPlayer {
            pos: transform.translation.truncate(),
            id: *id,

        }
    }).collect();

    bots.for_each_mut(|(mut transform, player_id, mut bot, rigid_body_handle, health, model, reload_timer, damage, max_distance, proj_speed, time_since_start_reload, time_since_last_shot)| {
        if let Some(bot) = bot.as_mut() {
            let rigid_body = rigid_body_set.get_mut(*rigid_body_handle).unwrap();
            let map = maps.0.get(&map_crc32.0).unwrap();

            bot.0.update_map_info(map);
            bot.0.update_player_info(&players);
            bot.0.misc_update();

            if health.0 > 0.0 {
                if let Some((angle, dashing)) = bot.0.movement() {
                    rigid_body.set_linvel(Vector2::new(DEFAULT_PLAYER_SPEED, DEFAULT_PLAYER_SPEED).component_mul(&Vector2::new(angle.0.cos(), angle.0.sin())), true);

                }

                if let Some(angle) = bot.0.update_direction() { transform.rotation = Quat::from_rotation_z(angle.0); }
            }
        }
    });
}
