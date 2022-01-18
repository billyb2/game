#![deny(clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use std::f32::consts::PI;
use std::iter::repeat_with;

use bevy::prelude::*;
use bevy::math::{Size as UI_Size, const_vec3, const_vec2};
use bevy::utils::Duration;
use bevy::input::ElementState;
use bevy::input::keyboard::KeyboardInput;

//use bevy_kira_audio::Audio;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use rapier2d::prelude::*;
use rapier2d::na::Vector2;

use crate::*;
use map::{MapCRC32, MapHealth};
use game_types::*;
use game_types::Size;
use net::*;
use map::WallMarker;

use helper_functions::{get_angle, f32_u8_to_u128};

// This just keeps the camera in sync with the player
//TODO: Make MapSize its own resource
pub fn move_camera(mut camera: Query<&mut Transform, With<GameCamera>>, players: Query<(&Transform, &Sprite, &Perk), Without<GameCamera>>, my_player_id: Res<MyPlayerID>, window: Res<WindowDescriptor>, maps: Res<Maps>, map_crc32: Res<MapCRC32>, player_entity: Res<HashMap<u8, Entity>>, wnds: Res<Windows>) {
    if let Some(my_player_id) = &my_player_id.0 {
        let (player, sprite, &perk) = players.get(*player_entity.get(&my_player_id.0).unwrap()).unwrap();

        let map = maps.0.get(&map_crc32.0).unwrap();

        let mut x = sprite.custom_size.unwrap().x.mul_add(-0.5, player.translation.x);
        let mut y = sprite.custom_size.unwrap().y.mul_add(0.5, player.translation.y);

        let half_window_width = window.width / 2.0;
        let half_window_height = window.height / 2.0;

        let camera = &mut camera.single_mut();

        if x - half_window_width < 0.0 {
            x = half_window_width;

        } else if x + half_window_width > map.size.x {
            x = map.size.x - half_window_width;

        }

        if -y - half_window_height < 0.0 {
            y = -half_window_height;

        } else if -y + half_window_height > map.size.y {
            y = -map.size.y + half_window_height;

        }

        camera.translation.x = x;
        camera.translation.y = y;

        let wnd = wnds.get_primary().unwrap();
        let res_scale = ((wnd.width() as f32 / 1366.0).min(wnd.height() as f32 / 768.0) * 0.95).recip();

        camera.scale = match perk {
            Perk::ExtendedVision => const_vec3!([1.5; 3]) * Vec3::new(res_scale, res_scale, 1.0),
            _ => const_vec3!([1.25; 3]) * Vec3::new(res_scale, res_scale, 1.0),
        };

    }
}


pub fn my_keyboard_input(mut commands: Commands, mut query: Query<(&mut PlayerSpeedInfo, &RigidBodyHandleWrapper, &Health)>, mut ev_reload: EventWriter<ReloadEvent>, mut ev_use_ability: EventWriter<AbilityEvent>, keybindings: Res<KeyBindings>, my_player_id: Res<MyPlayerID>, asset_server: Res<AssetServer>, player_entity: Res<HashMap<u8, Entity>>, button_materials: Res<ButtonMaterials>, mut materials: ResMut<Assets<ColorMaterial>>, in_game_settings: Query<(Entity, &InGameSettings)>, mut rigid_body_set: ResMut<RigidBodySet>, mut typing: ResMut<Typing>, keyboard_input: Res<Input<KeyCode>>, (gamepads, axes, button_inputs): (Res<Gamepads>, Res<Axis<GamepadAxis>>, Res<Input<GamepadButton>>)) {
    if !typing.0 {
        if in_game_settings.is_empty() {
            let mut angle = None;

            if keyboard_input.just_released(keybindings.talk) {
                typing.0 = true;

            }

            // All movement code
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
                ev_reload.send(ReloadEvent(my_player_id.0.unwrap().0));

            }

            for gamepad in gamepads.iter().cloned() {
                let left_stick_x = axes.get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX)).unwrap();
                let left_stick_y = axes.get(GamepadAxis(gamepad, GamepadAxisType::LeftStickY)).unwrap();
                
                if left_stick_x.abs() > 0.1 || left_stick_y.abs() > 0.1 {
                    angle =  Some(get_angle(left_stick_x, left_stick_y, 0.0, 0.0));

                }

                if button_inputs.pressed(GamepadButton(gamepad, GamepadButtonType::West)) {
                    ev_reload.send(ReloadEvent(my_player_id.0.unwrap().0));

                }

                if button_inputs.pressed(GamepadButton(gamepad, GamepadButtonType::RightTrigger)) {
                    ev_use_ability.send(AbilityEvent(my_player_id.0.unwrap().0));

                }

                let (mut player_speed_info, _rigid_body_handle, _health) = query.get_mut(*player_entity.get(&my_player_id.0.unwrap().0).unwrap()).unwrap();

                if button_inputs.pressed(GamepadButton(gamepad, GamepadButtonType::South)) && !player_speed_info.dash_info.dashing && player_speed_info.dash_info.time_till_can_dash.finished() {
                    player_speed_info.speed *= 3.1;

                    player_speed_info.dash_info.dashing = true;
                    player_speed_info.dash_info.time_till_stop_dash.reset();

                }

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
                        size: UI_Size {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                        },

                        ..Default::default()
                    },
                    color: UiColor(Color::rgba_u8(255, 255, 255, 10)),
                    visibility: Visibility {
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
                        size: UI_Size::new(Val::Px(225.0), Val::Px(85.0)),

                        ..Default::default()
                    },
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
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

            if let Some(my_player_id) = &my_player_id.0 {
                let (mut player_speed_info, rigid_body_handle, health) = query.get_mut(*player_entity.get(&my_player_id.0).unwrap()).unwrap();

                if health.0 > 0.0 {
                    if keyboard_input.just_pressed(keybindings.dash) && !player_speed_info.dash_info.dashing && player_speed_info.dash_info.time_till_can_dash.finished() {
                        player_speed_info.speed *= 3.1;

                        player_speed_info.dash_info.dashing = true;
                        player_speed_info.dash_info.time_till_stop_dash.reset();

                    }

                    if keyboard_input.pressed(keybindings.use_ability) {
                        ev_use_ability.send(AbilityEvent(my_player_id.0));

                    }

                    if let Some(angle) = angle {
                        let speed = &mut player_speed_info.speed;

                        let rigid_body = rigid_body_set.get_mut(rigid_body_handle.0).unwrap();
                        // If the player is dashing then they can't change the angle that they move in
                        let new_linvel = Vector2::new(angle.cos(), angle.sin()).component_mul(&Vector2::new(*speed, *speed));

                        rigid_body.set_linvel(new_linvel, true);

                    }
                }

            }

        } else if keyboard_input.just_pressed(KeyCode::Escape) {
            let entity = in_game_settings.single().0;
            commands.entity(entity).despawn_recursive();
        }

    }
}

pub fn score_input(mut score_ui: Query<(&mut Text, &mut Visibility), With<ScoreUI>>, score: Res<DeathmatchScore>, keyboard_input: Res<Input<KeyCode>>, asset_server: Res<AssetServer>, keybindings: Res<KeyBindings>, names: Query<&PlayerName>, player_entity: Res<HashMap<u8, Entity>>) {
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
            let player_name = names.get(*player_entity.get(player_id).unwrap()).unwrap();

            let singular_or_plural_kills =
                match **kills {
                    1 => "kill",
                    _ => "kills"
                };

            text.sections.push(
                TextSection {
                    value: format!("{}: {} {}\n", player_name, kills, singular_or_plural_kills),
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
}

pub fn chat_input(mut chat_text: Query<&mut Text, With<ChatText>>, mut typing: ResMut<Typing>, mut keyboard_input_events: EventReader<KeyboardInput>, mut net: ResMut<NetworkResource>, my_player_id: Res<MyPlayerID>, mut log_event: EventWriter<ChatEvent>, my_name: Res<PlayerName>, mut app_state: ResMut<State<AppState>>) {
    if typing.0 {
        let text = &mut chat_text.single_mut().sections[0].value;

        keyboard_input_events.iter().for_each(|ev| {
            if let Some(key_code) = ev.key_code {
                if ev.state == ElementState::Pressed {
                    // Whether or not the player has typed anything
                    let text_in_chat = text.len() > 6;

                    match key_code {
                        KeyCode::Return => {
                            if text_in_chat {
                                let my_player_id = my_player_id.0.as_ref().unwrap().0;
                                let message: TextMessage = (my_player_id, text[6..].to_owned(), 0);
                                net.broadcast_message(&message, &TEXT_MESSAGE_CHANNEL).err_on_server_disconnect(&mut net, &mut app_state);
                                log_event.send(ChatEvent(format!("{}: {}", my_name.as_ref(), text[6..].to_owned())));

                                text.truncate(6);
                                typing.0 = false;

                            }

                        },
                        KeyCode::Space => {
                            text.push(' ');

                        },
                        KeyCode::Back => {
                            if text_in_chat {
                                text.pop();

                            }

                        },
                        KeyCode::Escape => {
                            text.truncate(6);
                            typing.0 = false;

                        },
                        _ => {
                            let key_text = format!("{:?}", ev.key_code.unwrap());

                            if key_text.len() == 1 {
                                text.push_str(&key_text);

                            } else if key_text.starts_with("Key") {
                                text.push(key_text.chars().nth(3).unwrap());

                            }

                        },
                    };
                }

            }

        });

    }

}

pub fn shooting_player_input(btn: Res<Input<MouseButton>>, keyboard_input: Res<Input<KeyCode>>, mouse_pos: Res<MousePosition>,  mut shoot_event: EventWriter<ShootEvent>, mut query: Query<(&Bursting, &Transform, &mut Health, &Model, &MaxDistance, &RecoilRange, &Speed, &ProjectileType, &Damage, &AbilityInfo, &Size, &TimeSinceStartReload, &TimeSinceLastShot, &Perk)>, my_player_id: Res<MyPlayerID>, player_entity: Res<HashMap<u8, Entity>>, in_game_settings: Query<&InGameSettings>, keybindings: Res<KeyBindings>, gamepads: Res<Gamepads>, button_inputs: Res<Input<GamepadButton>>, axes: Res<Axis<GamepadAxis>>, wnds: Res<Windows>) {
    if in_game_settings.is_empty() {
        if let Some(my_player_id)= &my_player_id.0 {
            let (bursting, transform, mut health, model, max_distance, recoil_range, speed, projectile_type, damage, ability_info, size, reload_timer, time_since_last_shot, perk) = query.get_mut(*player_entity.get(&my_player_id.0).unwrap()).unwrap();

            let player_ability = &ability_info.ability;

            let mut shooting_controller = false;

            for gamepad in gamepads.iter().cloned() {
                if button_inputs.pressed(GamepadButton(gamepad, GamepadButtonType::RightTrigger2)) {
                    shooting_controller = true;
                    break;

                }

            }

            if btn.pressed(MouseButton::Left) || btn.just_pressed(MouseButton::Left) || bursting.0 || shooting_controller {
                // To allow for deterministic shooting, the recoil of every bullet is pre-generated and then sent over the network
                // It needs to be a vector since shotguns, for example, send multiple bulelts at a time, each with a different amount of recoil

                // TODO: Make number of bullets into a part of the gun
                let num_of_recoil = match *model {
                    Model::Shotgun => 12,
                    Model::ClusterShotgun => 6,
                    Model::Flamethrower => 5,
                    _ => 1,

                };

                if *model == Model::Widowmaker && time_since_last_shot.0.finished() {
                    if health.0 - damage.0 > 0.0 {
                        health.0 -= damage.0;

                    } else if health.0 * 0.5 > 1.0 {
                        health.0 *= 0.5;

                    } else {
                        health.0 = 1.0;

                    }
                }

                let rng = fastrand::Rng::new();

                let recoil_vec: Vec<f32> = repeat_with(|| {
                    let sign = rng.i8(..).signum() as f32;

                    (rng.f32() * recoil_range.0 * 2.0).copysign(sign)
                }).take(num_of_recoil).collect();

                let start_pos = transform.translation;

                let pos_direction = mouse_pos.0;

                let event = ShootEvent {
                    start_pos,
                    player_id: my_player_id.0,
                    pos_direction,
                    health: health.0,
                    model: *model,
                    max_distance: match *model == Model::StickyGrenade {
                        false => max_distance.0,
                        true => mouse_pos.0.distance(start_pos.truncate()),

                    },
                    recoil_vec,
                    speed: speed.0,
                    projectile_type: *projectile_type,
                    damage: *damage,
                    player_ability: *player_ability,
                    size: size.0,
                    reloading: reload_timer.reloading,

                };

                shoot_event.send(event);

            // Melee is the F key
            } else if keyboard_input.pressed(keybindings.melee) {
                // TODO: Const or smth?
                let melee = Gun::new(Model::Melee, *player_ability, *perk);

                let event = ShootEvent {
                    start_pos: transform.translation + melee.projectile_size.0.extend(0.0) / 2.0,
                    player_id: my_player_id.0,
                    pos_direction: mouse_pos.0,
                    health: health.0,
                    model: Model::Melee,
                    max_distance: melee.max_distance.0,
                    recoil_vec: vec![0.0],
                    speed: speed.0,
                    projectile_type: ProjectileType::Melee,
                    damage: melee.damage,
                    player_ability: *player_ability,
                    size: melee.projectile_size.0,
                    reloading: reload_timer.reloading,

                };

                shoot_event.send(event);
            }                
        }
    }

}

pub fn spawn_projectile(mut shoot_event: EventReader<ShootEvent>, mut commands: Commands, materials: Res<ProjectileMaterials>,  mut query: Query<(&mut Bursting, &mut TimeSinceLastShot, &mut AmmoInMag, &mut CanMelee)>, mut ev_reload: EventWriter<ReloadEvent>,  mut net: ResMut<NetworkResource>, my_player_id: Res<MyPlayerID>, player_entity: Res<HashMap<u8, Entity>>, mut rigid_body_set: ResMut<RigidBodySet>, mut collider_set: ResMut<ColliderSet>, local_players: Res<LocalPlayers>, camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>, mut app_state: ResMut<State<AppState>>, asset_server: Res<AssetServer>) {
    if my_player_id.0.is_some() {
        shoot_event.iter().for_each(|ev| {
            let player_is_local = local_players.0.contains(&ev.player_id);

            if ev.health != 0.0 {
                let angle = get_angle(ev.pos_direction.x, ev.pos_direction.y, ev.start_pos.x, ev.start_pos.y);

                let mut shooting = false;

                let speed = ev.speed;

                let player_id = ev.player_id;

                if ev.projectile_type != ProjectileType::Molotov && ev.projectile_type != ProjectileType::PulseWave {
                    let (mut bursting, mut time_since_last_shot, mut ammo_in_mag, mut can_melee) = query.get_mut(*player_entity.get(&player_id).unwrap()).unwrap();

                    // Checks that said player can shoot, and isn't reloading
                    if (time_since_last_shot.0.finished() && ammo_in_mag.0 > 0 && !ev.reloading && ev.projectile_type != ProjectileType::Melee) || ev.projectile_type == ProjectileType::TractorBeam  || (ev.projectile_type == ProjectileType::Melee && can_melee.0.finished()) {
                        shooting = true;

                        if ev.model == Model::Melee {
                            can_melee.0.reset();

                        } else if ev.model == Model::BurstRifle {
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

                    } else if ammo_in_mag.0 == 0 {
                        // Reload automatically if the player tries to shoot with no ammo
                        ev_reload.send(ReloadEvent(ev.player_id));

                    }

                } else {
                    shooting = true;

                }

                if shooting || !player_is_local {
                    // Only broadcast shots that the player shoots
                    if player_is_local {
                        net.broadcast_message(ev, &PROJECTILE_MESSAGE_CHANNEL).err_on_server_disconnect(&mut net, &mut app_state);

                    }

                    for recoil in ev.recoil_vec.iter() {
                        let movement = Vector2::new(recoil + angle.cos(), recoil + angle.sin()).component_mul(&Vector2::new(speed, speed));

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
                                    // TODO: Add a projectile material for WidowMaker
                                    ProjectileType::Regular | ProjectileType::WidowMaker => materials.regular.clone(),
                                    ProjectileType::Speedball => materials.speedball.clone(),
                                    ProjectileType::Molotov => materials.molotov.clone(),
                                    ProjectileType::MolotovFire => materials.molotov_fire.clone(),
                                    ProjectileType::MolotovLiquid => materials.molotov_liquid.clone(),
                                    ProjectileType::Flame => flame_material,
                                    ProjectileType::PulseWave => materials.pulsewave.clone(),
                                    ProjectileType::TractorBeam => materials.beam.clone(),
                                    ProjectileType::Melee => materials.regular.clone(),
                                    ProjectileType::Arrow => materials.arrow.clone(),
                                    ProjectileType::StickyGrenade => materials.arrow.clone(),
                                    // This branch should almost never happen
                                    ProjectileType::UsedBullet => materials.used_bullet.clone(),

                                }

                            };

                            /*let sound = match ev.model { 
                                Model::Speedball => asset_server.load("audio/laser.flac"),
                                _ => asset_server.load("audio/pew.flac"),

                            };
                            audio.play(sound);*/


                        // Move the projectile in front of the player according to the projectile's size                        
                        let angle_trig = Vec2::new(angle.cos(), angle.sin());

                        let translation = ev.start_pos.truncate() + (angle_trig * const_vec2!([75.0; 2])) + (ev.size * angle_trig);

                        let rigid_body = RigidBodyBuilder::new(RigidBodyType::Dynamic)
                            // The user_data is the damage, (shot_from, projectile_type) (f32, (u8, u8)) of the bullet
                            .user_data(f32_u8_to_u128(ev.damage.0, (player_id, ev.projectile_type.into())))
                            .translation((Vector2::new(translation.x, translation.y)).component_div(&Vector2::new(250.0, 250.0)))
                            .linvel(movement.component_div(&Vector2::new(5.0, 5.0)))
                            // The Speedball's projectiles move faster over time, thus, a negative linear dampening
                            .linear_damping(match ev.projectile_type {
                                ProjectileType::Speedball => -5.0,
                                ProjectileType::StickyGrenade => 3.0,
                                _ => 0.0,
                            })
                            .gravity_scale(0.0)
                            .ccd_enabled(true)
                            .build();

                        let collider_size = Vec2::new(ev.size.x, ev.size.y) / Vec2::new(500.0, 500.0);

                        let collider = ColliderBuilder::cuboid(collider_size.x, collider_size.x)
                            .user_data(f32_u8_to_u128(ev.damage.0, (player_id, ev.projectile_type.into())))
                            .restitution(0.0)
                            .friction(0.0)
                            .collision_groups(match ev.projectile_type {
                                // Pulsewaves move through walls, and therefore have different interaction groups
                                ProjectileType::PulseWave => InteractionGroups::new(0b0001, 0b1000),
                                ProjectileType::Molotov => InteractionGroups::new(0b0010, 0b0100),
                                _ => InteractionGroups::new(0b0010, 0b1100),

                            })
                            .build();

                    let rigid_body_handle = rigid_body_set.insert(rigid_body);
                    let collider_handle = collider_set.insert_with_parent(collider, rigid_body_handle, &mut rigid_body_set);

                    /*let light_destruction_timer = LightDestructionTimer(Timer::from_seconds(0.05, false));

                    let (camera, camera_transform) = camera.single();

                    let wnd = windows.get_primary().unwrap();
                    let wnd_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

                    let light_handle = lights_res.add_light(Vec2::ZERO);
                    
                    // Adjust the position of the shader's light
                    calc_shader_light_pos(translation.into(), &mut lights_res, camera, camera_transform, &windows, wnd_size, &light_handle);*/

                        commands
                            .spawn_bundle(Projectile::new(ev.projectile_type, Size::new(ev.size.x, ev.size.y), player_id, ev.damage))
                            .insert_bundle(SpriteBundle {
                                texture: material.as_image(&asset_server),
                                sprite: Sprite {
                                    color: material.as_color().unwrap_or(Default::default()),
                                    custom_size: Some(ev.size),
                                    ..Default::default()
                                },
                                transform: Transform {
                                    translation: translation.extend(ev.start_pos.z),
                                    rotation: Quat::from_rotation_z(angle),
                                    ..Default::default()

                                },
                                ..Default::default()
                            })
                            .insert(RigidBodyHandleWrapper(rigid_body_handle))
                            .insert(ColliderHandleWrapper(collider_handle))
                            .insert(MaxDistance(ev.max_distance))
                            .insert(DistanceTraveled(0.0))
                            .insert(Speed(ev.speed));
                            //.insert(light_handle)
                            //.insert(light_destruction_timer);

                        if ev.projectile_type == ProjectileType::Regular || ev.projectile_type == ProjectileType::WidowMaker {
                            let rng = fastrand::Rng::new();

                            let angle = rng.f32() * PI;
                            let rotation = rng.f32() * PI;

                            let rigid_body = RigidBodyBuilder::new(RigidBodyType::Dynamic)
                                .user_data(f32_u8_to_u128(0.0, (player_id, ProjectileType::UsedBullet.into())))
                                .translation((Vector2::new(ev.start_pos.x, ev.start_pos.y)).component_div(&Vector2::new(250.0, 250.0)))
                                .linvel(Vector2::new(rng.f32() * 2.25 * rng.i8(..).signum() as f32, rng.f32() * 2.25 * rng.i8(..).signum() as f32))
                                .linear_damping(5.0)
                                .gravity_scale(0.0)
                                .rotation(rotation)
                                .build();

                            let collider = ColliderBuilder::cuboid(collider_size.x, collider_size.x)
                                .restitution(1.0)
                                .friction(0.0)
                                .collision_groups(InteractionGroups::new(0b0010, 0b0100),)
                                .build();

                            let rigid_body_handle = rigid_body_set.insert(rigid_body);
                            let collider_handle = collider_set.insert_with_parent(collider, rigid_body_handle, &mut rigid_body_set);

                            commands
                                .spawn_bundle(Projectile::new(ProjectileType::UsedBullet, Size::new(ev.size.x, ev.size.y), player_id, Damage(0.0)))
                                .insert_bundle(SpriteBundle {
                                    texture: materials.used_bullet.as_image(&asset_server),
                                    sprite: Sprite {
                                        color: materials.used_bullet.as_color().unwrap_or(Default::default()),
                                        custom_size: Some(ev.size),
                                        ..Default::default()
                                    },
                                    transform: Transform {
                                        translation: ev.start_pos,
                                        rotation: Quat::from_rotation_z(angle),
                                        ..Default::default()

                                    },
                                    ..Default::default()
                                })
                                .insert(RigidBodyHandleWrapper(rigid_body_handle))
                                .insert(ColliderHandleWrapper(collider_handle))
                                .insert(MaxDistance(50.0))
                                .insert(DistanceTraveled(0.0))
                                .insert(Speed(0.01));

                        }
                    }
                }

            }
        });

    }
}

pub fn start_reload(mut query: Query<(&AmmoInMag, &MaxAmmo, &mut TimeSinceStartReload)>, mut ev_reload: EventReader<ReloadEvent>, player_entity: Res<HashMap<u8, Entity>>) {
    // Only start a reload if the reload event is read
    ev_reload.iter().for_each(|ev| {
        let (ammo_in_mag, max_ammo, mut reload_timer) = query.get_mut(*player_entity.get(&ev.0).unwrap()).unwrap();

        if ammo_in_mag.0 < max_ammo.0 && !reload_timer.reloading {
            reload_timer.reloading = true;
            // If the player reloads with one round or more in their mag, they reload faster
            reload_timer.fast_reload = ammo_in_mag.0 > 0;
            reload_timer.timer.reset();

        }
    });
}

pub fn use_ability(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, mut query: Query<(&Transform, &mut AbilityInfo, &mut PlayerSpeedInfo, &Health, &Model, &TimeSinceStartReload, &ColliderHandleWrapper, &RigidBodyHandleWrapper)>, mut ev_use_ability: EventReader<AbilityEvent>, mut net: ResMut<NetworkResource>, my_player_id: Res<MyPlayerID>, mouse_pos: Res<MousePosition>, mut shoot_event: EventWriter<ShootEvent>, player_entity: Res<HashMap<u8, Entity>>, mut collider_set: ResMut<ColliderSet>, mut rigid_body_set: ResMut<RigidBodySet>, local_players: Res<LocalPlayers>, gamepads: Res<Gamepads>, axes: Res<Axis<GamepadAxis>>) {
    if let Some(my_player_id)= &my_player_id.0 {
        for ev_id in ev_use_ability.iter() {
            let (transform, mut ability_info, mut speed_info, health, model, reload_timer, collider_handle, rigid_body_handle) = query.get_mut(*player_entity.get(&ev_id.0).unwrap()).unwrap();

            let speed = &mut speed_info.speed;

            let player_is_local = local_players.0.contains(&ev_id.0);

            // Events that come from other players dont need to wait for ability charge to finish
            if (ability_info.ability != Ability::Brute && ability_info.ability_charge.finished()) || !player_is_local || (ability_info.ability == Ability::Brute && ability_info.ability_charge.elapsed_secs() >= 0.5) {

                match ability_info.ability {
                    Ability::Wall => {
                        let rotation = get_angle(mouse_pos.0.x, mouse_pos.0.y, transform.translation.x, transform.translation.y);
                        let coords = transform.translation + const_vec3!([100.0, 100.0, 0.0]) * Vec3::new(rotation.cos(), rotation.sin(), 0.0);

                        let rotation = rotation + PI / 2.0;


                        let message_array: [f32; 3] = [coords.x, coords.y, rotation];
                        let message: AbilityMessage = ([ev_id.0, Ability::Wall.into()], message_array);

                        if player_is_local {
                            net.broadcast_message(&message, &ABILITY_MESSAGE_CHANNEL);

                        }

                        const COLOR: Color = Color::rgb(1.0, 1.0, 0.0);

                        const SIZE: Vec2 = const_vec2!([175.0, 50.0]);

                        const HEALTH_OF_WALL: f32 = 75.0;

                        // Storing the half extents as a tuple since I don't need to do any fancy SIMD stuff
                        // Equal to SIZE / [500.0; 2]
                        const HALF_EXTENTS: (f32, f32) = (0.35, 0.1);

                        let rigid_body = RigidBodyBuilder::new(RigidBodyType::Static)
                            .translation(Vector2::new(coords.x, coords.y).component_div(&Vector2::new(250.0, 250.0))) 
                            .rotation(rotation)
                            .user_data(0)
                            .gravity_scale(0.0)
                            .build();


                        let collider = ColliderBuilder::cuboid(HALF_EXTENTS.0, HALF_EXTENTS.1)
                            .collision_groups(InteractionGroups::new(0b0100, 0b1010))
                            .user_data(0)
                            .friction(0.1)
                            .build();

                        let rigid_body_handle = rigid_body_set.insert(rigid_body);
                        let collider_handle = collider_set.insert_with_parent(collider, rigid_body_handle, &mut rigid_body_set);

                        commands
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    color: COLOR,
                                    custom_size: Some(SIZE),
                                    ..Default::default()
                                },
                                transform: Transform {
                                    translation: coords,
                                    rotation: Quat::from_rotation_z(rotation),

                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(MapHealth(Some(HEALTH_OF_WALL)))
                            .insert(WallMarker)
                            .insert(GameRelated)
                            .insert(RigidBodyHandleWrapper(rigid_body_handle))
                            .insert(ColliderHandleWrapper(collider_handle));

                        ability_info.ability_charge.reset();

                    },
                    Ability::Warp => {
                        //TODO: Get some out of bounds checks for warping

                        let rigid_body = rigid_body_set.get_mut(rigid_body_handle.0).unwrap();

                        const MAX_WARP_DISTANCE: f32 = 3.3;

                        let mut warp_distance = rigid_body.linvel().component_div(&Vector2::new(4.0, 4.0));

                        warp_distance.x = match warp_distance.x.abs() > MAX_WARP_DISTANCE {
                            true => MAX_WARP_DISTANCE.copysign(warp_distance.x),
                            false => warp_distance.x,

                        };

                        warp_distance.y = match warp_distance.y.abs() > MAX_WARP_DISTANCE {
                            true => MAX_WARP_DISTANCE.copysign(warp_distance.y),
                            false => warp_distance.y,

                        };

                        let new_pos = rigid_body.translation() + warp_distance;

                        rigid_body.set_translation(new_pos, true);
                        ability_info.ability_charge.reset();

                    },
                    Ability::Stim => {
                        if !ability_info.using_ability && ability_info.ability_charge.finished() {
                            speed_info.speed *= 2.0;
                            ability_info.ability_completed.reset();
                            ability_info.using_ability = true;

                        }
                    },
                    // The engineer ability is passive, so when the use ability button is pressed nothing happens
                    Ability::Engineer => {},
                    // Inferno throws a molotov that lights an area on fire for a few seconds
                    Ability::Inferno => {
                        const PROJECTILE_SPEED: f32 = 20.0;

                        let event = ShootEvent {
                            start_pos: transform.translation,
                            player_id: ev_id.0,
                            pos_direction: mouse_pos.0,
                            health: health.0,
                            model: *model,
                            // The distance that the bullet will travel is just the distance between the mouse and the player
                            max_distance: mouse_pos.0.distance(transform.translation.truncate()),
                            recoil_vec: vec![0.0],
                            speed: PROJECTILE_SPEED,
                            projectile_type: ProjectileType::Molotov,
                            damage: Damage(5.0),
                            player_ability: ability_info.ability,
                            size: Vec2::new(7.0, 7.0),
                            reloading: reload_timer.reloading,

                        };

                        shoot_event.send(event);

                        ability_info.ability_charge.reset();

                    },
                    Ability::Cloak => {
                        if !player_is_local || (!ability_info.using_ability && ability_info.ability_charge.finished()) {
                            /*shader_phasing.value = match my_player_id.0 == ev_id.0 {
                                true => 0.25,
                                false => 0.0,
                            };*/

                            if player_is_local {
                                let message_array: [f32; 3] = [transform.translation.x, transform.translation.y, 0.0];
                                let message: ([u8; 2], [f32; 3]) = ([my_player_id.0, Ability::Cloak.into()], message_array);

                                net.broadcast_message(&message, &ABILITY_MESSAGE_CHANNEL);
                            }

                            ability_info.ability_completed.reset();
                            ability_info.using_ability = true;
                        }

                    },
                    Ability::PulseWave => {
                        const PROJECTILE_SPEED: f32 = 28.5;

                        let event = ShootEvent {
                            start_pos: transform.translation,
                            player_id: ev_id.0,
                            pos_direction: mouse_pos.0,
                            health: health.0,
                            model: *model,
                            max_distance: 2000.0,
                            recoil_vec: vec![0.0],
                            speed: PROJECTILE_SPEED,
                            projectile_type: ProjectileType::PulseWave,
                            damage: Damage(15.0),
                            player_ability: Ability::PulseWave,
                            size: const_vec2!([100.0; 2]),
                            reloading: reload_timer.reloading,

                        };

                        shoot_event.send(event);

                        ability_info.ability_charge.reset();             

                    },
                    Ability::Ghost =>  {
                        if !player_is_local || (!ability_info.using_ability && ability_info.ability_charge.finished()) {
                            //shader_phasing.value = 0.5;

                                if player_is_local {
                                    let message_array: [f32; 3] = [transform.translation.x, transform.translation.y, 0.0];
                                    let message: ([u8; 2], [f32; 3]) = ([my_player_id.0, Ability::Ghost.into()], message_array);


                                net.broadcast_message(&message, &ABILITY_MESSAGE_CHANNEL);

                            }

                            let collider = collider_set.get_mut(collider_handle.0).unwrap();
                            collider.set_collision_groups(InteractionGroups::new(0b1000, 0b1011));

                            ability_info.using_ability = true;
                            ability_info.ability_completed.reset();
                        }
                    },
                    Ability::Brute => {
                        const PROJECTILE_SPEED: f32 = 75.0;

                        let event = ShootEvent {
                            start_pos: transform.translation,
                            player_id: ev_id.0,
                            pos_direction: mouse_pos.0,
                            health: health.0,
                            model: *model,
                            max_distance: 1000.0,
                            recoil_vec: vec![0.0],
                            speed: PROJECTILE_SPEED,
                            projectile_type: ProjectileType::TractorBeam,
                            damage: Damage(0.0),
                            player_ability: Ability::Brute,
                            size: Vec2::splat(50.0),
                            reloading: reload_timer.reloading,

                        };

                        shoot_event.send(event);

                        if ability_info.ability_charge.elapsed_secs() - 0.8 >= 0.0 {
                            let new_charge_f32 = ability_info.ability_charge.elapsed() - Duration::from_secs_f32(0.06);
                            ability_info.ability_charge.set_elapsed(new_charge_f32);

                        } else {
                            ability_info.ability_charge.set_elapsed(Duration::from_secs_f32(0.0));

                        }

                    }
                };


            }
        }
    }
}

pub fn reset_player_resources(mut query: Query<(&mut AmmoInMag, &MaxAmmo, &mut
TimeSinceStartReload, &mut Bursting, &mut AbilityInfo, &mut PlayerSpeedInfo, &ColliderHandleWrapper)>, mut collider_set: ResMut<ColliderSet>) {
    query.for_each_mut(|(mut ammo_in_mag, max_ammo, mut reload_timer, mut bursting, mut ability_info, mut speed_info, collider_handle)| {
        if reload_timer.reloading && reload_timer.timer.finished() {
            ammo_in_mag.0 = max_ammo.0;
            reload_timer.reloading = false;
            reload_timer.fast_reload = false;
            bursting.0 = false;

        }

        if ability_info.using_ability && ability_info.ability_completed.finished() {
            if ability_info.ability == Ability::Stim {
                speed_info.speed = DEFAULT_PLAYER_SPEED + 1.0;

            } else if ability_info.ability == Ability::Ghost {
                let collider = collider_set.get_mut(collider_handle.0).unwrap();
                collider.set_collision_groups(InteractionGroups::new(0b1000, 0b1111));

            }

            ability_info.using_ability = false;
            ability_info.ability_charge.reset();

        }

        if speed_info.dash_info.dashing && speed_info.dash_info.time_till_stop_dash.finished() {
            speed_info.speed = match ability_info.ability {
                Ability::Stim => DEFAULT_PLAYER_SPEED + 1.0,
                Ability::Brute => DEFAULT_PLAYER_SPEED * 1.4,
                _ => DEFAULT_PLAYER_SPEED,

            };

            speed_info.dash_info.dashing = false;
            speed_info.dash_info.time_till_can_dash.reset();

        }

    });
}
//TODO!!!!!!
pub fn reset_player_phasing(mut query: Query<(&PlayerID, &AbilityInfo)>, local_players: Res<LocalPlayers>) {
    /*query.for_each_mut(|(player_id, ability_info, mut shader_phasing)| {
        let player_is_local = local_players.0.contains(&player_id.0);

        if !ability_info.using_ability && ability_info.ability != Ability::Stim && player_is_local {
            shader_phasing.value = 1.0;

        }
    });*/
}

pub fn set_mouse_coords(wnds: Res<Windows>, camera: Query<&Transform, With<GameCamera>>, mut mouse_pos: ResMut<MousePosition>, gamepads: Res<Gamepads>, axes: Res<Axis<GamepadAxis>>) {
    // assuming there is exactly one main camera entity, so this is OK
    let camera_transform = camera.single();

    // get the size of the window that the event is for
    let wnd = wnds.get_primary().unwrap();
    let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

    // the default orthographic projection is in pixels from the center;
    // just undo the translation
    // If there's a gamepad, calculate where the "mouse" would be using the right stick
    let cursor_pos = match gamepads.iter().next() {
        Some(gamepad) => {
            let gamepad = gamepad.clone();

            let right_stick_x = axes.get(GamepadAxis(gamepad, GamepadAxisType::RightStickX)).unwrap();
            let right_stick_y = axes.get(GamepadAxis(gamepad, GamepadAxisType::RightStickY)).unwrap();
                    
            let angle = get_angle(right_stick_x, right_stick_y, 0.0, 0.0);
            Vec2::new(angle.cos(), angle.sin()) * size
        },
        None => wnd.cursor_position().unwrap_or(Vec2::ZERO),
    };

    let p = cursor_pos - size / 2.0;

    // apply the camera transform
    let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

    mouse_pos.0 = pos_wld.truncate().truncate();

}

pub fn set_player_sprite_direction(my_player_id: Res<MyPlayerID>, mouse_pos: Res<MousePosition>, mut player_query: Query<&mut Transform>, player_entity: Res<HashMap<u8, Entity>>, in_game_settings: Query<(Entity, &InGameSettings)>, gamepads: Res<Gamepads>, axes: Res<Axis<GamepadAxis>>) {
    if let Some(my_player_id) = &my_player_id.0 {
        if in_game_settings.is_empty() {
            let mut transform = player_query.get_mut(*player_entity.get(&my_player_id.0).unwrap()).unwrap();

            if gamepads.iter().next().is_some() {
                for gamepad in gamepads.iter().cloned() {
                    let right_stick_x = axes.get(GamepadAxis(gamepad, GamepadAxisType::RightStickX)).unwrap();
                    let right_stick_y = axes.get(GamepadAxis(gamepad, GamepadAxisType::RightStickY)).unwrap();
                    
                    if right_stick_x.abs() > 0.1 || right_stick_y.abs() > 0.1 {
                        transform.rotation = Quat::from_rotation_z(get_angle(right_stick_x, right_stick_y, 0.0, 0.0));

                    }

                }
            
            // TODO: UsingMouse Resource
            } else {
                transform.rotation = Quat::from_rotation_z(get_angle(mouse_pos.0.x, mouse_pos.0.y, transform.translation.x, transform.translation.y));

            }
        }
    }

}
