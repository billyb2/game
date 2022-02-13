#![deny(clippy::all)]
#![allow(unused_assignments)]
#![allow(clippy::type_complexity)]

#![feature(trivial_bounds)]
#![feature(control_flow_enum)]

use std::sync::Arc;
use std::ops::ControlFlow;
use std::f32::consts::PI;
use std::marker::{Send, Sync};
use std::iter::repeat_with;

use bevy::prelude::*;
use rapier2d::prelude::*;
use rapier2d::na::Vector2;

use wasmer::{imports, Instance, Module, NativeFunc, Store};
#[cfg(target_arch = "x86_64")]
use wasmer_compiler_singlepass::Singlepass;
#[cfg(target_arch = "x86_64")]
use wasmer_engine_universal::Universal;

use map::*;
use helper_functions::*;
use game_types::*;

#[derive(Component)]
pub struct Bot {
    name: String,
    instance: InstanceWrapper,
    id: u8,

}

pub struct InstanceWrapper(pub Instance);


unsafe impl Send for InstanceWrapper {}
unsafe impl Sync for InstanceWrapper {}

pub struct BotActions {
    // Some for an angle to move in, None if shouldn't move
    pub movement_angle: Option<f32>,
    pub angle: f32,
    pub dashing: bool,
    pub using_ability: bool,
    pub shooting: bool,
    pub ability: Ability,
    pub perk: Perk,
    pub gun: Model,
}

impl Bot {
    pub fn new(wasm_bytes: &[u8], map_bytes: &[u8], id: &PlayerID) -> Self {
        #[cfg(target_arch = "x86_64")]
        let store = Store::new(&Universal::new(Singlepass::default()).engine());

        #[cfg(target_arch = "wasm32")]
        let store = Store::new();

        println!("Compiling wasm module...");
        // Let's compile the Wasm module.
        let module = Module::new(&store, wasm_bytes).unwrap();

        // Create an empty import object.
        let import_object = imports! {};

        let instance = Instance::new(&module, &import_object).unwrap();

        let mem_buffer = {
            let memory = instance.exports.get_memory("memory").unwrap();
            unsafe { memory.data_unchecked_mut() }

        };

        let name = {
            let bot_name_ptr: NativeFunc<(), u32> = instance.exports.get_native_function("bot_name").unwrap();
            let bot_name_ptr: usize = bot_name_ptr.call().unwrap().try_into().unwrap();

            mem_buffer[bot_name_ptr..].as_ref().iter().try_fold(String::with_capacity(10), |mut o_map_name, &byte_char| {
                match byte_char != 0 {
                    true => {
                        o_map_name.push(byte_char as char);
                        ControlFlow::Continue(o_map_name)
                    },
                    false => ControlFlow::Break(o_map_name),

                }

            }).break_value().unwrap().to_uppercase()


        };

        {
            let map_ptr: NativeFunc<(), u32> = instance.exports.get_native_function("map_mem_buffer_ptr").unwrap();
            let map_buffer_size: NativeFunc<(), u32> = instance.exports.get_native_function("map_mem_buffer_size").unwrap();

            let map_ptr: usize = map_ptr.call().unwrap().try_into().unwrap();
            let map_buffer_size: usize = map_buffer_size.call().unwrap().try_into().unwrap();

            // Set the initial map
            assert!(map_bytes.len() <= map_buffer_size);
            mem_buffer[map_ptr..map_ptr + map_bytes.len()].copy_from_slice(map_bytes);

        };

        {
            let player_health_ptr: NativeFunc<(), u32> = instance.exports.get_native_function("player_health_buffer_ptr").unwrap();
            let player_health_ptr: usize = player_health_ptr.call().unwrap().try_into().unwrap();
            // Set the player's health to 100
            mem_buffer[player_health_ptr..player_health_ptr + 4].copy_from_slice(&100.0_f32.to_be_bytes());

        };

        Bot {
            name,
            instance: InstanceWrapper(instance),
            id: id.0,
        }

    }

    pub fn update_info(&self, map_bytes: &[u8], enemy_bytes: &[u8], player_bytes: &[u8], health: f32) -> BotActions {
        let memory = self.instance.0.exports.get_memory("memory").unwrap();
        let mem_buffer = unsafe { memory.data_unchecked_mut() };

        {
            let map_ptr: NativeFunc<(), u32> = self.instance.0.exports.get_native_function("map_mem_buffer_ptr").unwrap();
            let map_buffer_size: NativeFunc<(), u32> = self.instance.0.exports.get_native_function("map_mem_buffer_size").unwrap();

            let map_ptr: usize = map_ptr.call().unwrap().try_into().unwrap();
            let map_buffer_size: usize = map_buffer_size.call().unwrap().try_into().unwrap();

            // Update map
            assert!(map_bytes.len() <= map_buffer_size);
            mem_buffer[map_ptr..map_ptr + map_bytes.len()].copy_from_slice(map_bytes)

        };

        {
            let player_health_ptr: NativeFunc<(), u32> = self.instance.0.exports.get_native_function("player_health_buffer_ptr").unwrap();
            let player_health_ptr: usize = player_health_ptr.call().unwrap().try_into().unwrap();
            // Update the player's health
            mem_buffer[player_health_ptr..player_health_ptr + 4].copy_from_slice(&health.to_be_bytes())

        };

        {
            let player_buffer_ptr: NativeFunc<(), u32> = self.instance.0.exports.get_native_function("player_mem_buffer_ptr").unwrap();
            let player_buffer_ptr: usize = player_buffer_ptr.call().unwrap().try_into().unwrap();

            // Update the player's position
            mem_buffer[player_buffer_ptr..player_buffer_ptr + 8].copy_from_slice(player_bytes)
        };

        {
            let enemy_buffer_ptr: NativeFunc<(), u32> = self.instance.0.exports.get_native_function("enemy_player_mem_buffer_ptr").unwrap();
            let enemy_buffer_size: NativeFunc<(), u32> = self.instance.0.exports.get_native_function("enemy_player_mem_buffer_size").unwrap();

            let enemy_buffer_ptr: usize = enemy_buffer_ptr.call().unwrap().try_into().unwrap();
            let enemy_buffer_size: usize = enemy_buffer_size.call().unwrap().try_into().unwrap();

            // Update enemy positions
            assert!(enemy_bytes.len() <= enemy_buffer_size);
            mem_buffer[enemy_buffer_ptr..enemy_buffer_ptr + enemy_bytes.len()].copy_from_slice(enemy_bytes);

        };

        {
            let player_health_ptr: NativeFunc<(), u32> = self.instance.0.exports.get_native_function("player_health_buffer_ptr").unwrap();
            let player_health_ptr: usize = player_health_ptr.call().unwrap().try_into().unwrap();

            // Update the player's health
            mem_buffer[player_health_ptr..player_health_ptr + 4].copy_from_slice(&health.to_be_bytes());

        };

        let (ability, perk, gun) = {
            let bot_ability_info: NativeFunc<(), u32> = self.instance.0.exports.get_native_function("new").unwrap();
            let info = bot_ability_info.call().unwrap().to_be_bytes();

            (info[0].into(), info[1].into(), info[2].into())

        };

        let get_bot_actions = || {
            let action_info: NativeFunc<(), u64> = self.instance.0.exports.get_native_function("action_info").unwrap();
            let angle_info: NativeFunc<(), f32> = self.instance.0.exports.get_native_function("direction_info").unwrap();
            let info_bytes = action_info.call().unwrap().to_be_bytes();

            let int_to_bool = |int: u8| -> bool {
                match int {
                    0 => false,
                    _ => true,
                }

            };

            let angle = angle_info.call().unwrap();
            let movement_angle = f32::from_be_bytes(info_bytes[0..4].try_into().unwrap());
            let should_move = int_to_bool(info_bytes[5]);
            let should_dash = int_to_bool(info_bytes[6]);
            let should_use_ability = int_to_bool(info_bytes[6]);
            let should_shoot = int_to_bool(info_bytes[7]);

            BotActions {
                ability,
                gun,
                perk,
                angle,
                movement_angle: match should_move {
                    true => Some(movement_angle),
                    false => None,
                },
                dashing: should_dash,
                using_ability: should_use_ability,
                shooting: should_shoot,
            }

        };

        get_bot_actions()

    }

    pub fn name(&self) -> &str {
        &self.name
        
    }

}

pub fn handle_bots(mut bots: Query<(&mut Transform, &PlayerID, Option<&mut Bot>, &RigidBodyHandleWrapper, &mut Health, &Model, &AbilityInfo, &MaxDistance, &Speed, &TimeSinceLastShot, &AmmoInMag, &RecoilRange, &TimeSinceStartReload)>, mut rigid_body_set: ResMut<RigidBodySet>, map_crc32: Res<MapCRC32>, maps: Res<Maps>, mut shoot_event: EventWriter<ShootEvent>, mut ev_reload: EventWriter<ReloadEvent>, mut ev_ability: EventWriter<AbilityEvent>) {
    // Generate the list of TruncatedPlayer by looping over the bots list initially
    let mut players: [u8; 9 * 32] = [0; 9 * 32];

    let mut bot_player_bytes: [u8; 8] = [0; 8];
    let mut enemy_player_bytes: [u8; 8 * 31] = [0; 8 * 31];

    let map_bin = {
        let map = maps.0.get(&map_crc32.0).unwrap();
        map.to_min_bin()

    };

    bots.iter().zip(players.chunks_mut(9)).for_each(|((transform, id, _bw, _rgb, health, _model, ability_info, _md, _pjs, _ttls, _aig, _rr, _rt), player_bytes)| {
        // Cloaking players and dead players aren't shown
        if !(ability_info.ability == Ability::Cloak && ability_info.using_ability) && health.0 > 0.0 {
            player_bytes[0..4].copy_from_slice(&transform.translation.x.to_be_bytes());
            player_bytes[4..8].copy_from_slice(&transform.translation.y.to_be_bytes());
            player_bytes[8] = id.0;

        }

    });

    bots.for_each_mut(|(mut transform, player_id, mut bot, rigid_body_handle, mut health, model, ability_info, max_distance, proj_speed, time_since_last_shot, ammo_in_mag, recoil_range, reload_timer)| {
        let rigid_body_handle = &rigid_body_handle.0;

        let mut enemy_player_index = 0;
        let mut added_player = false;

        if let Some(bot) = bot.as_mut() {
            players.chunks(9).for_each(|player_bytes| {
                if *player_bytes != [0; 9] {
                    if player_bytes[8] == player_id.0 {
                        bot_player_bytes.copy_from_slice(&player_bytes[0..8]);
                        added_player = true;

                    } else {
                        enemy_player_bytes[enemy_player_index..enemy_player_index + 8].copy_from_slice(&player_bytes[0..8]);
                        enemy_player_index += 8;

                    }

                }
            });

            let rigid_body = rigid_body_set.get_mut(*rigid_body_handle).unwrap();

            let actions = bot.update_info(&map_bin, &enemy_player_bytes, &bot_player_bytes, health.0);

            let gun = Gun::new(*model, ability_info.ability, Perk::ExtendedMag);

            if health.0 > 0.0 {
                if let Some(angle) = actions.movement_angle {
                    rigid_body.set_linvel(Vector2::new(DEFAULT_PLAYER_SPEED, DEFAULT_PLAYER_SPEED).component_mul(&Vector2::new(angle.cos(), angle.sin())), true);

                }

                transform.rotation = Quat::from_rotation_z(actions.angle); 

                if actions.shooting && time_since_last_shot.0.finished() {
                    if ammo_in_mag.0 > 0 {
                        let pos_direction = (Vec2::new(actions.angle.cos(), actions.angle.sin()) * 200.0) + transform.translation.truncate();

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
                                speed: proj_speed.0,
                                projectile_type: gun.projectile_type,
                                damage: gun.damage,
                                player_ability: ability_info.ability,
                                size: Vec2::new(5.0, 5.0),
                                reloading: reload_timer.reloading,
                            }

                        );

                    } else {
                        ev_reload.send(ReloadEvent(player_id.0));

                    }
                }

                if actions.using_ability && ability_info.ability_charge.finished() {
                    ev_ability.send(AbilityEvent(player_id.0));

                }
            }
        }
    });
}