#![deny(clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::utils::Duration;
use rand::Rng;
use rand::seq::SliceRandom;


use crate::helper_functions::get_angle;

use crate::*;
use crate::components::*;
use crate::player_attr::*;

#[cfg(feature = "web")]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// This just keeps the camera in sync with the player
//TODO: Make MapSize its own resource
pub fn move_camera(mut camera: Query<&mut Transform, With<GameCamera>>, players: Query<(&Transform, &Sprite), Without<GameCamera>>, my_player_id: Res<MyPlayerID>, window: Res<WindowDescriptor>, map: Res<Map>, player_entity: Res<HashMap<u8, Entity>>) {
    if let Some(my_player_id) = &my_player_id.0 {
        let (player, sprite) = players.get(*player_entity.get(&my_player_id.0).unwrap()).unwrap();

        let mut x = player.translation.x - sprite.size.x / 2.0;
        let mut y = player.translation.y + sprite.size.y / 2.0;

        if x - window.width / 2.0 < 0.0 {
            x = window.width / 2.0;

        } else if x + window.width / 2.0 > map.size.x {
            x = map.size.x - window.width / 2.0;

        }

        if -y - window.height / 2.0 < 0.0 {
            y = -window.height / 2.0;

        } else if -y + window.height / 2.0 > map.size.y {
            y = -map.size.y + window.height / 2.0;

        }

        camera.single_mut().unwrap().translation.x = x;
        camera.single_mut().unwrap().translation.y = y;

    }
}


//TODO: Use EventReader<KeyboardInput> for more efficient input checking (https://bevy-cheatbook.github.io/features/input-handling.html)
pub fn my_keyboard_input(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut RequestedMovement, &mut PlayerSpeed, &mut DashingInfo)>, mut ev_reload: EventWriter<ReloadEvent>, mut ev_use_ability: EventWriter<AbilityEvent>, keybindings: Res<KeyBindings>, my_player_id: Res<MyPlayerID>, asset_server: Res<AssetServer>, mut score_ui: Query<(&mut Text, &mut Visible), With<ScoreUI>>, score: Res<DeathmatchScore>, player_entity: Res<HashMap<u8, Entity>>) {
    let mut angle = None;

    if keyboard_input.pressed(keybindings.left) && angle.is_none() {
        match keyboard_input.pressed(keybindings.up) {
            true => { angle = Some(PI  * 0.75); }
            false => {
                match keyboard_input.pressed(keybindings.down) {
                    true => { angle = Some(PI * 1.25); }
                    false => { angle = Some(PI); }

                }

            }

        }

    }

    if keyboard_input.pressed(keybindings.right) && angle.is_none() {
        match keyboard_input.pressed(keybindings.up) {
            true => { angle = Some(PI  * 0.25); }
            false => {
                match keyboard_input.pressed(keybindings.down) {
                    true => { angle = Some(PI * 1.75); }
                    false => { angle = Some(0.0); }

                }

            }

        }

    }

    if keyboard_input.pressed(keybindings.down) && angle.is_none() {
        angle = Some(-PI / 2.0);

    }

    if keyboard_input.pressed(keybindings.up) && angle.is_none() {
       angle = Some(PI / 2.0);

    }

    if keyboard_input.pressed(keybindings.reload) {
        ev_reload.send(ReloadEvent);

    }

    if keyboard_input.just_pressed(keybindings.show_score) {
        let (mut text, mut visible) = score_ui.single_mut().unwrap();

        visible.is_visible = true;


        for (player_id, kills) in score.0.iter() {
            let singular_or_plural_kills = 
                if *kills == 1 {
                    "kill"

                } else {
                    "kills"

                };

            text.sections.push(
                TextSection {
                    value: format!("Player {}: {} {}\n", player_id + 1, kills, singular_or_plural_kills).to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 45.0,
                        color: Color::WHITE,
                    },
                }
            );

        }



    } else if keyboard_input.just_released(keybindings.show_score) {
        let (mut text, mut visible) = score_ui.single_mut().unwrap();

        visible.is_visible = false;

        while text.sections.len() != 1 {
            text.sections.pop();

        }


    }

    if let Some(my_player_id) = &my_player_id.0 {
        let (mut requested_movement, mut speed, mut dashing_info) = query.get_mut(*player_entity.get(&my_player_id.0).unwrap()).unwrap();

        if keyboard_input.just_pressed(keybindings.dash) && !dashing_info.dashing && dashing_info.time_till_can_dash.finished() {
            speed.0 *= 3.1;

            dashing_info.dashing = true;
            dashing_info.time_till_stop_dash.reset();

        }

        // Only do a change event if a key has been pressed
        if keyboard_input.pressed(keybindings.use_ability) {
            ev_use_ability.send(AbilityEvent(my_player_id.0));

        }

        if let Some(angle) = angle {
            // If the player is dashing then they can't change the angle that they move in
            if !dashing_info.dashing {
                requested_movement.angle = angle;

            }
            requested_movement.speed = speed.0;

        }

    }
}

pub fn shooting_player_input(btn: Res<Input<MouseButton>>, mouse_pos: Res<MousePosition>,  mut shoot_event: EventWriter<ShootEvent>, query: Query<(&Bursting, &Transform, &PlayerID, &Health, &Model, &MaxDistance, &RecoilRange, &Speed, &ProjectileType, &Damage, &Ability, &Size, &TimeSinceStartReload)>, my_player_id: Res<MyPlayerID>) {
    for (bursting, transform, id, health, model, max_distance, recoil_range, speed, projectile_type, damage, player_ability, size, reload_timer) in query.iter() {
        if let Some(my_player_id)= &my_player_id.0 {
            if id.0 == my_player_id.0 {
                if btn.pressed(MouseButton::Left) || btn.just_pressed(MouseButton::Left) || bursting.0 {
                    let mut rng = rand::thread_rng();

                    // To allow for deterministic shooting, the recoil of every bullet is pre-generated and then sent over the network
                    // It needs to be a vector since shotguns, for example, send multiple bulelts at a time, each with a different amount of recoil

                    // TODO: Make number of bullets into a part of the gun
                    let mut recoil_vec: Vec<f32> = match *model {
                        Model::Shotgun => Vec::with_capacity(12),
                        Model::ClusterShotgun => Vec::with_capacity(6),
                        Model::Flamethrower => Vec::with_capacity(5),
                        _ => Vec::with_capacity(1),

                    };

                    // Fill the recoil_vec to capacity
                    while recoil_vec.len() < recoil_vec.capacity() {
                        let recoil =
                            if recoil_range.0 == 0.0 {
                                0.0

                            } else {
                                rng.gen_range(-recoil_range.0..recoil_range.0)

                        };

                        recoil_vec.push(recoil);

                    }

                    let event = ShootEvent {
                        start_pos: transform.translation,
                        player_id: id.0,
                        pos_direction: mouse_pos.0,
                        health: health.0,
                        model: *model,
                        max_distance: max_distance.0,
                        recoil_vec,
                        // Bullets need to travel "backwards" when moving to the left
                        speed: match mouse_pos.0.x <= transform.translation.x {
                            true => -speed.0,
                            false => speed.0,
                        },
                        projectile_type: *projectile_type,
                        damage: *damage,
                        player_ability: *player_ability,
                        size: Vec2::new(size.width, size.height),
                        reloading: reload_timer.reloading,


                    };

                    shoot_event.send(event);

                }

                break;

            }
        }

    }

}

pub fn spawn_projectile(mut shoot_event: EventReader<ShootEvent>, mut commands: Commands, materials: Res<ProjectileMaterials>,  mut query: Query<(&mut Bursting, &mut TimeSinceLastShot, &mut AmmoInMag)>, mut ev_reload: EventWriter<ReloadEvent>,  mut net: ResMut<NetworkResource>, my_player_id: Res<MyPlayerID>, player_entity: Res<HashMap<u8, Entity>>) {
    if let Some(my_player_id)= &my_player_id.0 {
        for ev in shoot_event.iter() {
            if ev.health != 0.0 {
                let angle = get_angle(ev.pos_direction.x, ev.pos_direction.y, ev.start_pos.x, ev.start_pos.y);

                let mut shooting = false;

                let mut rng = rand::thread_rng();

                let speed = ev.speed;

                let player_id = ev.player_id;

                if ev.projectile_type != ProjectileType::Molotov {
                    let (mut bursting, mut time_since_last_shot, mut ammo_in_mag) = query.get_mut(*player_entity.get(&player_id).unwrap()).unwrap();

                    // Checks that said player can shoot, and isnt reloading
                    if time_since_last_shot.0.finished() && ammo_in_mag.0 > 0 && !ev.reloading {
                        shooting = true;

                        if ev.model == Model::BurstRifle {
                            if !bursting.0 {
                                bursting.0 = true;
                                time_since_last_shot.0.set_duration(Duration::from_millis(45));

                            } else if ammo_in_mag.0 % 3 == 0 {
                                bursting.0 = false;
                                shooting = false;

                                time_since_last_shot.0.set_duration(Duration::from_millis(500));

                            }

                        }


                        if shooting {
                            ammo_in_mag.0 -= 1;

                        }

                        time_since_last_shot.0.reset();

                    } else if ammo_in_mag.0 == 0 && player_id == my_player_id.0 {
                        // Reload automatically if the player tries to shoot with no ammo
                        ev_reload.send(ReloadEvent);

                    }

                } else if ev.projectile_type == ProjectileType::Molotov {
                    shooting = true;

                }

                if shooting || player_id != my_player_id.0 {
                    net.broadcast_message((*ev).clone());

                    for recoil in ev.recoil_vec.iter() {
                        let movement = RequestedMovement::new(angle + recoil, speed);

                        let material =
                            if ev.player_ability == Ability::Engineer && ev.model != Model::Flamethrower {
                                materials.engineer.clone()

                            } else {
                                let num = rng.gen_range(0..=2);

                                let flame_material = match num {
                                    0 => materials.flamethrower1.clone(),
                                    1 => materials.flamethrower2.clone(),
                                    2 => materials.flamethrower3.clone(),
                                    _ => materials.flamethrower1.clone(),
                                };

                                match ev.projectile_type {
                                    ProjectileType::Regular => materials.regular.clone(),
                                    ProjectileType::Speedball => materials.speedball.clone(),
                                    ProjectileType::Molotov => materials.molotov.clone(),
                                    ProjectileType::MolotovFire => materials.molotov_fire.clone(),
                                    ProjectileType::MolotovLiquid => materials.molotov_liquid.clone(),
                                    ProjectileType::Flame => flame_material,

                                }

                            };

                        commands
                            .spawn_bundle(Projectile::new(movement, ev.projectile_type, ev.max_distance, Size::new(ev.size.x, ev.size.y), player_id, ev.damage))
                            .insert_bundle(SpriteBundle {
                                material,
                                sprite: Sprite::new(Vec2::new(5.0 * (4.0/3.0), 5.0 * (4.0/3.0))),
                                transform: Transform::from_xyz(ev.start_pos.x + 5.0, ev.start_pos.y + 5.0, 0.0),
                                ..Default::default()
                            });
                    }
                }

            }
        }

    }
}

pub fn start_reload(mut query: Query<(&AmmoInMag, &MaxAmmo, &mut TimeSinceStartReload)>, mut ev_reload: EventReader<ReloadEvent>, my_player_id: Res<MyPlayerID>, player_entity: Res<HashMap<u8, Entity>>) {
    // Only start a reload if the reload event is read
    if let Some(my_player_id)= &my_player_id.0 {
        for _ in ev_reload.iter() {
            let (ammo_in_mag, max_ammo, mut reload_timer) = query.get_mut(*player_entity.get(&my_player_id.0).unwrap()).unwrap();

            if ammo_in_mag.0 < max_ammo.0 && !reload_timer.reloading {
                reload_timer.reloading = true;
                reload_timer.timer.reset();

            }
        }
    }
}

pub fn use_ability(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, mut
query: Query<(&Transform, &mut RequestedMovement, &Ability, &mut AbilityCharge, &mut
AbilityCompleted, &mut PlayerSpeed, &Health, &mut UsingAbility, &Model, &TimeSinceStartReload, &mut Visible)>, mut ev_use_ability: EventReader<AbilityEvent>, mut map:
ResMut<Map>, mut net: ResMut<NetworkResource>, my_player_id: Res<MyPlayerID>, online_player_ids: Res<OnlinePlayerIDs>, mouse_pos: Res<MousePosition>, mut shoot_event: EventWriter<ShootEvent>, player_entity: Res<HashMap<u8, Entity>>) {
    if let Some(my_player_id)= &my_player_id.0 {
        for ev_id in ev_use_ability.iter() {
                let (transform, mut requested_movement, ability, mut ability_charge, mut
            ability_completed, mut speed, health, mut using_ability, model, reload_timer, mut
            visible) = query.get_mut(*player_entity.get(&ev_id.0).unwrap()).unwrap();

            // Events that come from other players dont need to wait for ability charge to finish
            if ability_charge.0.finished() || ev_id.0 != my_player_id.0 {
                match ability {
                    Ability::Wall => {
                        if requested_movement.speed != 0.0 || ev_id.0 != my_player_id.0 {
                            let message_array: [f32; 3] = [transform.translation.x, transform.translation.y, requested_movement.angle];

                            let message: ([u8; 2], [f32; 3]) = ([my_player_id.0, Ability::Wall.into()], message_array);

                            if ev_id.0 == my_player_id.0 {
                                net.broadcast_message(message);

                            }

                            let color = Color::rgb_u8(255, 255, 0);

                            let color_handle: Handle<ColorMaterial> = {
                                let mut color_to_return = None;

                                for (id, material_to_return) in materials.iter() {
                                    if color == material_to_return.color {
                                        color_to_return = Some(materials.get_handle(id));
                                        break;

                                    }

                                }

                                match color_to_return {
                                    Some(color) => color,
                                    None => materials.add(color.into())

                                }

                            };

                            let coords = transform.translation + Vec3::new(100.0 * requested_movement.angle.cos(), 100.0 * requested_movement.angle.sin(), 0.0);

                            let size =
                                if requested_movement.angle.abs() == PI / 2.0 {
                                    Vec2::new(100.0, 25.0)

                                } else {
                                    Vec2::new(25.0, 100.0)

                                };

                            let health_of_wall: f32 = 300.0;

                            commands
                                .spawn_bundle(SpriteBundle {
                                    material: color_handle.clone(),
                                    sprite: Sprite::new(size),
                                    transform: Transform {
                                        translation: coords,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(Health(health_of_wall))
                                .insert(WallMarker);

                            map.objects.push(
                                MapObject {
                                    coords,
                                    size,
                                    color,
                                    collidable: true,
                                    player_spawn: false,
                                    health: Some(health_of_wall),

                                }
                            );

                            ability_charge.0.reset();

                        }
                    },
                    Ability::Warp => {
                        requested_movement.speed += 500.0;
                        #[cfg(feature = "web")]
                        console_log!("Warp speed: {}", requested_movement.speed);

                        ability_charge.0.reset();

                    },
                    Ability::Stim => {
                        if !using_ability.0 && ability_charge.0.finished() {
                            speed.0 *= 2.0;
                            ability_completed.0.reset();
                            using_ability.0 = true;

                        }
                    },
                    Ability::Hacker => {
                        let mut potential_players_to_be_hacked: Vec<u8> = Vec::with_capacity(255);

                        for id in online_player_ids.0.iter() {
                            if *id != my_player_id.0 {
                                potential_players_to_be_hacked.push(*id);

                            }
                        }

                        let mut rng = rand::thread_rng();

                        if !potential_players_to_be_hacked.is_empty() {
                            // Get a random player that isn't the current player
                            let player_to_be_hacked: u8 = *potential_players_to_be_hacked.choose(&mut rng).unwrap();

                            let message: ([u8; 2], [f32; 3]) = ([player_to_be_hacked, Ability::Hacker.into()], [transform.translation.x, transform.translation.y, 0.0]);
                            
                            net.broadcast_message(message);


                            ability_charge.0.reset();

                        }

                    },
                    // The engineer ability is passive, so when the use ability button is pressed nothing happens
                    Ability::Engineer => {},
                    // Inferno throws a molotov that lights an area on fire for a few seconds
                    Ability::Inferno => {
                        let projectile_speed: f32 = 6.0;

                        let event = ShootEvent {
                            start_pos: transform.translation,
                            player_id: ev_id.0,
                            pos_direction: mouse_pos.0,
                            health: health.0,
                            model: *model,
                            // The distance that the bullet will travel is just the distance between the mouse and the player
                            max_distance: mouse_pos.0.distance(transform.translation.truncate()),
                            recoil_vec: vec![0.0],
                            // Bullets need to travel "backwards" when moving to the left
                            speed: match mouse_pos.0.x <= transform.translation.x {
                                true => -projectile_speed,
                                false => projectile_speed,
                            },
                            projectile_type: ProjectileType::Molotov,
                            damage: Damage(5.0),
                            player_ability: Ability::Inferno,
                            size: Vec2::new(3.0, 3.0),
                            reloading: reload_timer.reloading,

                        };

                        shoot_event.send(event);

                        ability_charge.0.reset();

                    },
                    Ability::Cloak => {
                        if !using_ability.0 && ability_charge.0.finished() {
                            let message_array: [f32; 3] = [transform.translation.x, transform.translation.y, requested_movement.angle];

                            let message: ([u8; 2], [f32; 3]) = ([my_player_id.0, Ability::Cloak.into
                            ()], message_array);

                            if ev_id.0 == my_player_id.0 {
                                net.broadcast_message(message);

                            }

                            visible.is_visible = false;
                            ability_completed.0.reset();
                            using_ability.0 = true;
                        }

                    },
                };


            }
        }
    }
}

pub fn reset_player_resources(mut query: Query<(&mut AmmoInMag, &MaxAmmo, &mut
TimeSinceStartReload, &mut Bursting, &AbilityCompleted, &Ability, &mut UsingAbility, &mut
AbilityCharge, &mut PlayerSpeed, & mut Visible, &mut DashingInfo)>) {
    query.for_each_mut(|(mut ammo_in_mag, max_ammo, mut reload_timer, mut bursting, ability_completed, ability,
        mut using_ability, mut ability_charge, mut speed, mut visible, mut dashing_info)| {
        if reload_timer.reloading && reload_timer.timer.finished() {
            ammo_in_mag.0 = max_ammo.0;
            reload_timer.reloading = false;
            bursting.0 = false;


        }

        if using_ability.0 && ability_completed.0.finished() {
            if *ability == Ability::Stim {
                speed.0 /= 2.0;

            } else if *ability == Ability::Cloak {
                visible.is_visible = true;

            }

            using_ability.0 = false;
            ability_charge.0.reset();

        }

        if dashing_info.dashing && dashing_info.time_till_stop_dash.finished() {
            speed.0 /= 3.1;
            dashing_info.dashing = false;
            dashing_info.time_till_can_dash.reset();

        }

    });
}


pub fn set_mouse_coords(wnds: Res<Windows>, camera: Query<&Transform, With<GameCamera>>, mut mouse_pos: ResMut<MousePosition>, mut shader_mouse_pos: Query<&mut ShaderMousePosition> ) {
    // assuming there is exactly one main camera entity, so this is OK
    let camera_transform = camera.single().unwrap();

    // get the size of the window that the event is for
    let wnd = wnds.get_primary().unwrap();
    let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

    // the default orthographic projection is in pixels from the center;
    // just undo the translation
    let cursor_pos = match wnd.cursor_position() {
        Some(pos) => pos,
        None => Vec2::ZERO,

    };

    let p = cursor_pos - size / 2.0;


    shader_mouse_pos.for_each_mut(|mut shader_mouse_pos| {
        shader_mouse_pos.value = cursor_pos / size;

    });

    // apply the camera transform
    let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

    mouse_pos.0 = pos_wld.into();

}

pub fn set_player_sprite_direction(my_player_id: Res<MyPlayerID>, mouse_pos: Res<MousePosition>, mut player_query: Query<(&mut Sprite, &Transform)>, player_entity: Res<HashMap<u8, Entity>>) {
    if let Some(my_player_id) = &my_player_id.0 {
        let (mut sprite, transform) = player_query.get_mut(*player_entity.get(&my_player_id.0).unwrap()).unwrap();
        sprite.flip_x = mouse_pos.0.x >= transform.translation.x;

    }

}
