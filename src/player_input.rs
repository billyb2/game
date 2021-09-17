#![deny(clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use std::f32::consts::PI;
use std::iter::repeat_with;

use bevy::math::{Vec3A, const_vec3};
use bevy::prelude::*;
use bevy::utils::Duration;

//use bevy_kira_audio::Audio;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::*;
use map::MapCRC32;
use game_types::*;
use game_types::player_attr::*;
use map::WallMarker;

use helper_functions::get_angle;

// This just keeps the camera in sync with the player
//TODO: Make MapSize its own resource
pub fn move_camera(mut camera: Query<&mut Transform, With<GameCamera>>, players: Query<(&Transform, &Sprite, &Perk), Without<GameCamera>>, my_player_id: Res<MyPlayerID>, window: Res<WindowDescriptor>, maps: Res<Maps>, map_crc32: Res<MapCRC32>, player_entity: Res<HashMap<u8, Entity>>) {
    if let Some(my_player_id) = &my_player_id.0 {
        let (player, sprite, &perk) = players.get(*player_entity.get(&my_player_id.0).unwrap()).unwrap();

        let map = maps.0.get(&map_crc32.0).unwrap();

        let mut x = player.translation.x - sprite.size.x / 2.0;
        let mut y = player.translation.y + sprite.size.y / 2.0;

        let camera = &mut camera.single_mut();

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

        camera.translation.x = x;
        camera.translation.y = y;


        camera.scale = match perk {
            Perk::ExtendedVision => const_vec3!([0.7; 3]),
            _ => const_vec3!([1.0; 3]),
        };

    }
}


//TODO: Use EventReader<KeyboardInput> for more efficient input checking (https://bevy-cheatbook.github.io/features/input-handling.html)
pub fn my_keyboard_input(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut RequestedMovement, &mut PlayerSpeed, &mut DashingInfo)>, mut ev_reload: EventWriter<ReloadEvent>, mut ev_use_ability: EventWriter<AbilityEvent>, keybindings: Res<KeyBindings>, my_player_id: Res<MyPlayerID>, asset_server: Res<AssetServer>, mut score_ui: Query<(&mut Text, &mut Visible), With<ScoreUI>>, score: Res<DeathmatchScore>, player_entity: Res<HashMap<u8, Entity>>, button_materials: Res<ButtonMaterials>, mut materials: ResMut<Assets<ColorMaterial>>, in_game_settings: Query<(Entity, &InGameSettings)>) {
    if in_game_settings.is_empty() {
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

        if keyboard_input.just_pressed(KeyCode::Escape) {
            commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::ColumnReverse,
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
                    size: Size {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                    },

                    ..Default::default()
                },
                material: materials.add(Color::rgba_u8(255, 255, 255, 10).into()),
                visible: Visible {
                    is_visible: true,
                    ..Default::default()
                },
                ..Default::default()

            })
            .with_children(|node_parent| {
                node_parent.spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "Settings".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 80.0,
                                    color: Color::GOLD,
                                },
                            },
                        ],
                        ..Default::default()
                    },
                    ..Default::default()

                });

                node_parent.spawn_bundle(ButtonBundle {
                style: Style {
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Px(225.0), Val::Px(85.0)),

                    ..Default::default()
                },
                material: button_materials.normal.clone(),
                ..Default::default()
                })
                .with_children(|button_parent| {
                    button_parent
                        .spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection {
                                        value: String::from("Customize"),
                                        style: TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 55.0,
                                            color: Color::WHITE,
                                        },
                                    },
                                ],
                                ..Default::default()
                            },
                            ..Default::default()

                    });
                });

            })
            .insert(InGameSettings::Settings);
        }

        if keyboard_input.just_pressed(keybindings.show_score) {
            let (mut text, mut visible) = score_ui.single_mut();

            visible.is_visible = true;

            // Sorts the HashMap by number of kills first, before displaying
            let mut v: Vec<(&u8, &u8)> = (&score.0).into_iter().collect();
            
            let compare = |x: &(&u8, &u8), y: &(&u8, &u8)| {
                y.1.cmp(x.1)

            };

            #[cfg(feature = "parallel")]
            v.par_sort_unstable_by(compare);

            #[cfg(not(feature = "parallel"))]
            v.sort_unstable_by(compare);

            for (player_id, kills) in v.iter() {
                let singular_or_plural_kills =
                    match **kills {
                        1 => "kill",
                        _ => "kills"
                    };

                text.sections.push(
                    TextSection {
                        value: format!("Player {}: {} {}\n", *player_id, kills, singular_or_plural_kills),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 45.0,
                            color: Color::WHITE,
                        },
                    }
                );

            }



        } else if keyboard_input.just_released(keybindings.show_score) {
            let (mut text, mut visible) = score_ui.single_mut();

            visible.is_visible = false;

            text.sections.truncate(1);


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

    } else if keyboard_input.just_pressed(KeyCode::Escape) {
        let entity = in_game_settings.single().0;
        commands.entity(entity).despawn_recursive();
    }
}

pub fn shooting_player_input(btn: Res<Input<MouseButton>>, mouse_pos: Res<MousePosition>,  mut shoot_event: EventWriter<ShootEvent>, query: Query<(&Bursting, &Transform, &Health, &Model, &MaxDistance, &RecoilRange, &Speed, &ProjectileType, &Damage, &Ability, &Size, &TimeSinceStartReload, &Phasing)>, my_player_id: Res<MyPlayerID>, player_entity: Res<HashMap<u8, Entity>>, in_game_settings: Query<&InGameSettings>) {
    if in_game_settings.is_empty() {
        if let Some(my_player_id)= &my_player_id.0 {
            let (bursting, transform, health, model, max_distance, recoil_range, speed, projectile_type, damage, player_ability, size, reload_timer, phasing) = query.get(*player_entity.get(&my_player_id.0).unwrap()).unwrap();

            if !phasing.0 && (btn.pressed(MouseButton::Left) || btn.just_pressed(MouseButton::Left) || bursting.0) {
                // To allow for deterministic shooting, the recoil of every bullet is pre-generated and then sent over the network
                // It needs to be a vector since shotguns, for example, send multiple bulelts at a time, each with a different amount of recoil

                // TODO: Make number of bullets into a part of the gun
                let num_of_recoil = match *model {
                    Model::Shotgun => 12,
                    Model::ClusterShotgun => 6,
                    Model::Flamethrower => 5,
                    _ => 1,

                };

                let rng = fastrand::Rng::new();

                let recoil_vec: Vec<f32> = repeat_with(|| {
                    let sign = rng.i8(..).signum() as f32;
                    rng.f32() * recoil_range.0 * sign
                }).take(num_of_recoil).collect();

                let event = ShootEvent {
                    start_pos: transform.translation + Vec3::new(size.width, size.height, 0.0) / 2.0,
                    player_id: my_player_id.0,
                    pos_direction: mouse_pos.0,
                    health: health.0,
                    model: *model,
                    max_distance: max_distance.0,
                    recoil_vec,
                    // Bullets need to travel "backwards" when moving to the left
                    speed: speed.0.copysign(mouse_pos.0.x - transform.translation.x),
                    projectile_type: *projectile_type,
                    damage: *damage,
                    player_ability: *player_ability,
                    size: Vec2::new(size.width, size.height),
                    reloading: reload_timer.reloading,

                };

                shoot_event.send(event);

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

                let speed = ev.speed;

                let player_id = ev.player_id;

                if ev.projectile_type != ProjectileType::Molotov && ev.projectile_type != ProjectileType::PulseWave {
                    let (mut bursting, mut time_since_last_shot, mut ammo_in_mag) = query.get_mut(*player_entity.get(&player_id).unwrap()).unwrap();

                    // Checks that said player can shoot, and isnt reloading
                    if (time_since_last_shot.0.finished() && ammo_in_mag.0 > 0 && !ev.reloading) || ev.projectile_type == ProjectileType::TractorBeam {
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

                        if ev.projectile_type != ProjectileType::TractorBeam {
                            if shooting && ev.projectile_type != ProjectileType::Melee {
                                ammo_in_mag.0 -= 1;

                            }

                            time_since_last_shot.0.reset();
                        }

                    } else if ammo_in_mag.0 == 0 && player_id == my_player_id.0 {
                        // Reload automatically if the player tries to shoot with no ammo
                        ev_reload.send(ReloadEvent);

                    }

                } else {
                    shooting = true;

                }

                if shooting || player_id != my_player_id.0 {
                    // Only broadcast shots that the player shoots
                    if player_id == my_player_id.0 {
                        net.broadcast_message((*ev).clone());
                    }

                    for recoil in ev.recoil_vec.iter() {
                        let movement = RequestedMovement::new(angle + recoil, speed);

                        let material =
                            if ev.player_ability == Ability::Engineer && ev.model != Model::Flamethrower {
                                materials.engineer.clone()

                            } else {
                                let num = fastrand::u8(0..=2);

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
                                    ProjectileType::PulseWave => materials.pulsewave.clone(),
                                    ProjectileType::TractorBeam => materials.beam.clone(),
                                    ProjectileType::Melee => materials.regular.clone(),

                                }

                            };

                            /*let sound = match ev.model { 
                                Model::Speedball => asset_server.load("audio/laser.flac"),
                                _ => asset_server.load("audio/pew.flac"),

                            };
                            audio.play(sound);*/

                        let angle = match speed.is_sign_negative() {
                            true => angle - PI,
                            false => angle,

                        };

                        // Move the projectile in front of the player according to the projectile's size
                        let size_vec3a = Vec3A::from((ev.size, 1.0));
                        
                        let angle_trig = Vec3A::new(angle.cos(), angle.sin(), 0.0);
                        let mut translation: Vec3A = ev.start_pos.into();
                        
                        translation += size_vec3a * angle_trig;
                        
                        commands
                            .spawn_bundle(Projectile::new(movement, ev.projectile_type, ev.max_distance, Size::new(ev.size.x, ev.size.y), player_id, ev.damage))
                            .insert_bundle(SpriteBundle {
                                material,
                                sprite: Sprite::new(ev.size),
                                transform: Transform {
                                    translation: translation.into(),
                                    rotation: Quat::from_rotation_z(angle),
                                    ..Default::default()

                                },
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
AbilityCompleted, &mut PlayerSpeed, &Health, &mut UsingAbility, &Model, &TimeSinceStartReload, &mut Phasing, &mut Alpha)>, mut ev_use_ability: EventReader<AbilityEvent>, mut maps:
ResMut<Maps>, map_crc32: Res<MapCRC32>, mut net: ResMut<NetworkResource>, my_player_id: Res<MyPlayerID>, online_player_ids: Res<OnlinePlayerIDs>, mouse_pos: Res<MousePosition>, mut shoot_event: EventWriter<ShootEvent>, player_entity: Res<HashMap<u8, Entity>>) {
    if let Some(my_player_id)= &my_player_id.0 {
        for ev_id in ev_use_ability.iter() {
                let (transform, mut requested_movement, ability, mut ability_charge, mut
            ability_completed, mut speed, health, mut using_ability, model, reload_timer, mut phasing, mut shader_phasing) = query.get_mut(*player_entity.get(&ev_id.0).unwrap()).unwrap();

            // Events that come from other players dont need to wait for ability charge to finish
            if (*ability != Ability::Brute && ability_charge.0.finished()) || ev_id.0 != my_player_id.0 || (*ability == Ability::Brute && ability_charge.0.elapsed_secs() >= 0.5) {
                match ability {
                    Ability::Wall => {
                        if requested_movement.speed != 0.0 || ev_id.0 != my_player_id.0 {
                            let message_array: [f32; 3] = [transform.translation.x, transform.translation.y, requested_movement.angle];

                            let message: ([u8; 2], [f32; 3]) = ([my_player_id.0, Ability::Wall.into()], message_array);

                            if ev_id.0 == my_player_id.0 {
                                net.broadcast_message(message);

                            }

                            let color_vec = UVec4::new(255, 255, 0, 255);
                            let color = Color::rgb_u8(color_vec.x as u8, color_vec.y as u8, color_vec.z as u8);

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

                            let coords = transform.translation + Vec3::new(100.0 * requested_movement.angle.cos(), 100.0 * requested_movement.angle.sin(), 5.0);

                            let rotation = 0.0;


                            let size = match requested_movement.angle.abs() == PI / 2.0 {
                                true => Vec2::new(100.0, 25.0),
                                false => Vec2::new(25.0, 100.0)

                            };

                            let health_of_wall: f32 = 75.0;

                            commands
                                .spawn_bundle(SpriteBundle {
                                    material: color_handle.clone(),
                                    sprite: Sprite::new(size),
                                    transform: Transform {
                                        translation: coords,
                                        rotation: Quat::from_rotation_z(rotation),

                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(Health(health_of_wall))
                                .insert(WallMarker);

                            maps.0.get_mut(&map_crc32.0).unwrap().objects.push(
                                MapObject {
                                    coords: coords.extend(rotation),
                                    size,
                                    sprite: color_vec,
                                    collidable: true,
                                    player_spawn: false,
                                    health: Some(health_of_wall),
                                    using_image: false,

                                }
                            );

                            ability_charge.0.reset();

                        }
                    },
                    Ability::Warp => {
                        requested_movement.speed = 550.0;
                        ability_charge.0.reset();

                    },
                    Ability::Stim => {
                        if !using_ability.0 && ability_charge.0.finished() {
                            speed.0 *= 1.5;
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

                        if !potential_players_to_be_hacked.is_empty() {
                            // Get a random player that isn't the current player
                            let rand_index = fastrand::usize(..potential_players_to_be_hacked.len());

                            let player_to_be_hacked: u8 = unsafe { *potential_players_to_be_hacked.get_unchecked(rand_index) };

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
                            player_ability: *ability,
                            size: Vec2::new(3.0, 3.0),
                            reloading: reload_timer.reloading,

                        };

                        shoot_event.send(event);

                        ability_charge.0.reset();

                    },
                    Ability::Cloak => {
                        let my_player_used_ability = ev_id.0 == my_player_id.0;

                        if !my_player_used_ability || (!using_ability.0 && ability_charge.0.finished()) {
                            if my_player_used_ability {
                                let message_array: [f32; 3] = [transform.translation.x, transform.translation.y, requested_movement.angle];
                                let message: ([u8; 2], [f32; 3]) = ([my_player_id.0, Ability::Cloak.into()], message_array);

                                net.broadcast_message(message);
                                shader_phasing.value = 0.25;

                            } else {
                                shader_phasing.value = 0.0;
                            }

                            ability_completed.0.reset();
                            using_ability.0 = true;
                        }

                    },
                    Ability::PulseWave => {
                        let projectile_speed: f32 = 20.5;

                        let event = ShootEvent {
                            start_pos: transform.translation,
                            player_id: ev_id.0,
                            pos_direction: mouse_pos.0,
                            health: health.0,
                            model: *model,
                            max_distance: 2000.0,
                            recoil_vec: vec![0.0],
                            // Bullets need to travel "backwards" when moving to the left
                            speed: match mouse_pos.0.x <= transform.translation.x {
                                true => -projectile_speed,
                                false => projectile_speed,
                            },
                            projectile_type: ProjectileType::PulseWave,
                            damage: Damage(15.0),
                            player_ability: Ability::PulseWave,
                            size: Vec2::new(100.0, 100.0),
                            reloading: reload_timer.reloading,

                        };

                        shoot_event.send(event);

                        ability_charge.0.reset();             

                    },
                    Ability::Ghost =>  {
                        let my_player_used_ability = ev_id.0 == my_player_id.0;

                        if !my_player_used_ability || (!using_ability.0 && ability_charge.0.finished()) {
                            phasing.0 = true;

                            shader_phasing.value = 0.5;

                            if my_player_used_ability {
                                let message_array: [f32; 3] = [transform.translation.x, transform.translation.y, requested_movement.angle];
                                let message: ([u8; 2], [f32; 3]) = ([my_player_id.0, Ability::Ghost.into()], message_array);

                                net.broadcast_message(message);

                            }

                            using_ability.0 = true;
                            ability_completed.0.reset();
                        }
                    },
                    Ability::Brute => {
                        let projectile_speed = 1.0;

                        let event = ShootEvent {
                            start_pos: transform.translation,
                            player_id: ev_id.0,
                            pos_direction: mouse_pos.0,
                            health: health.0,
                            model: *model,
                            max_distance: 1.0,
                            recoil_vec: vec![0.0],
                            // Bullets need to travel "backwards" when moving to the left
                            speed: match mouse_pos.0.x <= transform.translation.x {
                                true => -projectile_speed,
                                false => projectile_speed,
                            },
                            projectile_type: ProjectileType::TractorBeam,
                            damage: Damage(0.0),
                            player_ability: Ability::Brute,
                            size: Vec2::new(400.0, 45.0),
                            reloading: reload_timer.reloading,

                        };

                        shoot_event.send(event);

                        if ability_charge.0.elapsed_secs() - 0.8 >= 0.0 {
                            let new_charge_f32 = ability_charge.0.elapsed() - Duration::from_secs_f32(0.06);
                            ability_charge.0.set_elapsed(new_charge_f32);

                        } else {
                            ability_charge.0.set_elapsed(Duration::from_secs_f32(0.0));

                        }

                    }
                };


            }
        }
    }
}

pub fn reset_player_resources(mut query: Query<(&mut AmmoInMag, &MaxAmmo, &mut
TimeSinceStartReload, &mut Bursting, &AbilityCompleted, &Ability, &mut UsingAbility, &mut
AbilityCharge, &mut PlayerSpeed, &mut DashingInfo, &mut Phasing, &Transform, &Sprite, &mut Health)>, maps: Res<Maps>, map_crc32: Res<MapCRC32>, mut death_event: EventWriter<DeathEvent>, my_player_id: Res<MyPlayerID>) {
    query.for_each_mut(|(mut ammo_in_mag, max_ammo, mut reload_timer, mut bursting, ability_completed, ability,
        mut using_ability, mut ability_charge, mut speed, mut dashing_info, mut phasing, transform, sprite, mut health)| {
        if reload_timer.reloading && reload_timer.timer.finished() {
            ammo_in_mag.0 = max_ammo.0;
            reload_timer.reloading = false;
            bursting.0 = false;


        }

        if using_ability.0 && ability_completed.0.finished() {
            if *ability == Ability::Stim {
                speed.0 = DEFAULT_PLAYER_SPEED + 1.0;

            } else if *ability == Ability::Ghost {
                let map = maps.0.get(&map_crc32.0).unwrap();
                let collision = map.collision_no_damage(transform.translation.truncate(), sprite.size, 0.0, Vec2::splat(0.0));

                if collision.0 || collision.1 {
                    health.0 = 0.0;
                    death_event.send(DeathEvent(my_player_id.0.as_ref().unwrap().0));
                }

                phasing.0 = false;
            }

            using_ability.0 = false;
            ability_charge.0.reset();

        }

        if dashing_info.dashing && dashing_info.time_till_stop_dash.finished() {
            speed.0 = match *ability {
                Ability::Stim => DEFAULT_PLAYER_SPEED + 1.0,
                Ability::Brute => DEFAULT_PLAYER_SPEED * 1.4,
                _ => DEFAULT_PLAYER_SPEED,

            };

            dashing_info.dashing = false;
            dashing_info.time_till_can_dash.reset();

        }

    });
}

pub fn reset_player_phasing(mut query: Query<(&UsingAbility, &Ability, &mut Alpha)>) {
    query.for_each_mut(|(using_ability, ability, mut shader_phasing)| {
        if !using_ability.0 && *ability != Ability::Stim {
            shader_phasing.value = 1.0;

        }
    });
}

pub fn set_mouse_coords(wnds: Res<Windows>, camera: Query<&Transform, With<GameCamera>>, mut mouse_pos: ResMut<MousePosition>, mut shader_mouse_pos: Query<&mut ShaderMousePosition> ) {
    // assuming there is exactly one main camera entity, so this is OK
    let camera_transform = camera.single();

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

pub fn set_player_sprite_direction(my_player_id: Res<MyPlayerID>, mouse_pos: Res<MousePosition>, mut player_query: Query<&mut Transform>, player_entity: Res<HashMap<u8, Entity>>, in_game_settings: Query<(Entity, &InGameSettings)>) {
    if let Some(my_player_id) = &my_player_id.0 {
        if in_game_settings.is_empty() {
            let mut transform = player_query.get_mut(*player_entity.get(&my_player_id.0).unwrap()).unwrap();

            let angle = get_angle(mouse_pos.0.x, mouse_pos.0.y, transform.translation.x, transform.translation.y);

            transform.rotation = match mouse_pos.0.x <= transform.translation.x {
                true => Quat::from_rotation_z(angle - PI),
                false => Quat::from_rotation_z(angle),

            };

        }
    }

}
