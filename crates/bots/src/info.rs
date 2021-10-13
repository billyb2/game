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

use std::iter::repeat_with;

/// The returned value is the requested movement angle of the player in radians
#[derive(Copy, Clone)]
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
    fn new(map: &Map, my_player_id: PlayerID) -> (Self, Ability, Model) where Self: Sized;
    /// For updating the internals of the bot depending on changes in the map
    fn update_map_info(&mut self, map: &Map);
    /// For updating the internals of the bot depending on changes in the player's positions, abilities, weapons, etc.
    fn update_player_info(&mut self, players: &Vec<TruncatedPlayer>);
    /// Updates based on changes to the health of the player
    fn update_health(&mut self, new_health: Health);
    /// Any final updates to itself that the bot wants to do
    fn misc_update(&mut self);
    /// Returns the angle at which the player will move
    /// While the player can say whether or not they're dashing, it isn't guaranteed that the server will let them (if, say, their cooldown isn't finished)
    fn movement(&self) -> Option<(Angle, Dashing)>;
    /// The direction which the player is facing in
    fn update_direction(&self) -> Option<Angle>;
    /// For some abilities, they need extra information besides whether or not they'll be moving or not
    fn use_ability(&self) -> bool;
    fn should_shoot(&self) -> bool;

}

pub fn handle_bots(mut bots: Query<(&mut Transform, &PlayerID, Option<&mut BotWrapper>, &RigidBodyHandle, &mut Health, &Model, &Ability, &MaxDistance, &Speed, &TimeSinceLastShot, &AmmoInMag, &RecoilRange, &TimeSinceStartReload, &UsingAbility)>, mut rigid_body_set: ResMut<RigidBodySet>, map_crc32: Res<MapCRC32>, maps: Res<Maps>, mut shoot_event: EventWriter<ShootEvent>, mut ev_reload: EventWriter<ReloadEvent>, mut ev_ability: EventWriter<AbilityEvent>) {
    // Generate the list of TruncatedPlayer by looping over the bots list initially
    let players: Vec<(TruncatedPlayer, bool)> = bots.iter_mut().map(|(transform, id, _bw, _rgb, _h, _model, ability, _md, _pjs, _ttls, _aig, _rr, _rt, using_ability)| {
        // Cloaking players aren't shown to bots
        (TruncatedPlayer {
            pos: transform.translation.truncate(),
            id: *id,
        }, *ability == Ability::Cloak && using_ability.0)

    }).collect();

    bots.for_each_mut(|(mut transform, player_id, mut bot, rigid_body_handle, mut health, model, ability, max_distance, proj_speed, time_since_last_shot, ammo_in_mag, recoil_range, reload_timer, _using_ability)| {
        if let Some(bot) = bot.as_mut() {
            let players: Vec<TruncatedPlayer> = players.iter().filter_map(|(t_player, inv)| {
                match *inv && t_player.id.0 != player_id.0 {
                    true => None,
                    false => Some(t_player)

                }
            }).cloned().collect();

            let rigid_body = rigid_body_set.get_mut(*rigid_body_handle).unwrap();
            let map = maps.0.get(&map_crc32.0).unwrap();

            bot.0.update_map_info(map);
            bot.0.update_player_info(&players);
            bot.0.update_health(*health);
            bot.0.misc_update();

            let gun = Gun::new(*model, *ability, Perk::ExtendedMag);

            if health.0 > 0.0 {
                if let Some((angle, dashing)) = bot.0.movement() {
                    rigid_body.set_linvel(Vector2::new(DEFAULT_PLAYER_SPEED, DEFAULT_PLAYER_SPEED).component_mul(&Vector2::new(angle.0.cos(), angle.0.sin())), true);

                }

                if let Some(angle) = bot.0.update_direction() { 
                    transform.rotation = Quat::from_rotation_z(angle.0); 

                    if bot.0.should_shoot() && time_since_last_shot.0.finished() {
                        if ammo_in_mag.0 > 0 {
                            let pos_direction = (Vec2::new(angle.0.cos(), angle.0.sin()) * 200.0) + transform.translation.truncate();

                            // TODO: Make number of bullets into a part of the gun
                            let num_of_recoil = match *model {
                                Model::Shotgun => 12,
                                Model::ClusterShotgun => 6,
                                Model::Flamethrower => 5,
                                _ => 1,

                            };

                            if !(health.0 - gun.damage.0 <= 0.0) && *model == Model::Widowmaker {
                                health.0 -= gun.damage.0;

                            }

                            let rng = fastrand::Rng::new();

                            let recoil_vec: Vec<f32> = repeat_with(|| {
                                let sign = rng.i8(..).signum() as f32;
                                rng.f32() * recoil_range.0 * 2.0 * sign
                            }).take(num_of_recoil).collect();

                            shoot_event.send(
                                ShootEvent {
                                    start_pos: transform.translation,
                                    player_id: player_id.0,
                                    pos_direction,
                                    health: health.0,
                                    model: *model,
                                    max_distance: max_distance.0,
                                    recoil_vec,
                                    // Bullets need to travel "backwards" when moving to the left
                                    speed: proj_speed.0.copysign(pos_direction.x - transform.translation.x),
                                    projectile_type: gun.projectile_type,
                                    damage: gun.damage,
                                    player_ability: *ability,
                                    size: Vec2::new(5.0, 5.0),
                                    reloading: reload_timer.reloading,
                                }

                            );

                        } else {
                            ev_reload.send(ReloadEvent(player_id.0));

                        }
                    }

                }

                if bot.0.use_ability() {
                    ev_ability.send(AbilityEvent(player_id.0));

                }
            }
        }
    });
}
