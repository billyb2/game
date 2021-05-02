use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::utils::Duration;
use rand::Rng;

use crate::helper_functions::get_angle;

use crate::*;
use crate::components::*;
use crate::player_attributes::*;

// This just keeps the camera in sync with the player
pub fn move_camera(
    mut q: QuerySet<(
        Query<&mut Transform, With<GameCamera>>,
        Query<(&Transform, &PlayerID, Changed<Transform>)>)
    >) {
    let mut x =  q.q0_mut().single_mut().unwrap().translation.x;
    let mut y =  q.q0_mut().single_mut().unwrap().translation.y;


    for (player, id, _) in q.q1_mut().iter_mut() {
        if id.0 == 0 {
            x = player.translation.x;
            y= player.translation.y;

        }
    }

    q.q0_mut().single_mut().unwrap().translation.x = x;
    q.q0_mut().single_mut().unwrap().translation.y = y;
}


//TODO: Use EventReader<KeyboardInput> for more efficient input checking (https://bevy-cheatbook.github.io/features/input-handling.html)
pub fn player_1_keyboard_input(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut RequestedMovement, &PlayerID, &PlayerSpeed)>, mut ev_reload: EventWriter<ReloadEvent>, mut ev_use_ability: EventWriter<AbilityEvent>, keybindings: Res<KeyBindings>) {
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

    if keyboard_input.pressed(keybindings.use_ability) {
        ev_use_ability.send(AbilityEvent);

    }

    // Only do a change event if a key has been pressed
    if let Some(angle) = angle {
        for (mut requested_movement, id, speed) in query.iter_mut() {
            if id.0 == 0 {
                requested_movement.angle = angle;
                requested_movement.speed = speed.0;

                break;

            }
        }
    }
}

pub fn shooting_player_input(btn: Res<Input<MouseButton>>, mouse_pos: Res<MousePosition>,  mut shoot_event: EventWriter<ShootEvent>, query: Query<(&Bursting, &Transform, &PlayerID, &Health, &Model, &MaxDistance, &RecoilRange, &Speed, &ProjectileType, &Damage, &Ability, &Size, &TimeSinceStartReload)>, mut net: ResMut<NetworkResource>,) {
    for (bursting, transform, id, health, model, max_distance, recoil_range, speed, projectile_type, damage, player_ability, size, reload_timer) in query.iter() {
        if *id == PlayerID(0) {
            if btn.pressed(MouseButton::Left) || btn.just_pressed(MouseButton::Left) || bursting.0 {
                let event = ShootEvent {
                    start_pos: transform.translation,
                    player_id: id.0,
                    pos_direction: mouse_pos.0,
                    health: health.0,
                    model: *model,
                    max_distance: max_distance.0,
                    recoil_range: recoil_range.0,
                    speed: speed.0,
                    projectile_type: *projectile_type,
                    damage:*damage,
                    player_ability: *player_ability,
                    size: Vec2::new(size.width, size.height),
                    reloading: reload_timer.reloading,


                };

                shoot_event.send(event);
                //net.broadcast_message(event);

            }
            break;

        }

    }

}

pub fn spawn_projectile(mut shoot_event: EventReader<ShootEvent>, mut commands: Commands, materials: Res<ProjectileMaterials>,  mut query: Query<(&PlayerID, &mut Bursting, &mut TimeSinceLastShot, &mut AmmoInMag)>, mut ev_reload: EventWriter<ReloadEvent>) {
    for ev in shoot_event.iter() {
        if ev.health != 0 {
            let angle = get_angle(ev.pos_direction.x, ev.pos_direction.y, ev.start_pos.x, ev.start_pos.y);

            let mut shooting = false;

            let mut speed = ev.speed;

            let mut num_of_bullets = 1;

            let player_id = ev.player_id;

            for (id, mut bursting, mut time_since_last_shot, mut ammo_in_mag) in query.iter_mut() {
                // Checks that player 1 can shoot, and isnt reloading
                if id.0 == player_id {
                    if time_since_last_shot.0.finished() && ammo_in_mag.0 > 0 && !ev.reloading {
                        shooting = true;

                        if ev.model == Model::Shotgun {
                            num_of_bullets = 12;

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


                        if shooting {
                            ammo_in_mag.0 -= 1;

                        }

                        time_since_last_shot.0.reset();

                    } else if ammo_in_mag.0 == 0 {
                        // Reload automatically if the player tries to shoot with no ammo
                        ev_reload.send(ReloadEvent);

                    }


                    // Bullets need to travel "backwards" when moving to the left
                    if ev.pos_direction.x <= ev.start_pos.x {
                        speed = -speed;

                    }

                    break;

                }

            }

            if shooting {
                let mut rng = rand::thread_rng();

                for _ in 0..num_of_bullets {
                    let recoil =
                        if ev.recoil_range == 0.0 {
                            0.0

                        } else {
                            rng.gen_range(-ev.recoil_range..ev.recoil_range)

                    };

                    let movement = RequestedMovement::new(angle + recoil, speed);

                    let material =
                        if ev.player_ability == Ability::Engineer {
                            materials.engineer.clone()

                        } else if ev.projectile_type == ProjectileType::Regular {
                            materials.regular.clone()

                        } else {
                            materials.speedball.clone()

                        };

                    commands
                        .spawn_bundle(Projectile::new(movement, ev.projectile_type, ev.max_distance, Size::new(ev.size.x, ev.size.y), player_id, ev.damage))
                        .insert_bundle(SpriteBundle {
                            material,
                            sprite: Sprite::new(Vec2::new(5.0, 5.0)),
                            transform: Transform::from_xyz(ev.start_pos.x + 2.5, ev.start_pos.y + 2.5, 0.0),
                            ..Default::default()
                        });
                }
            }
        }
    }
}

pub fn start_reload(mut query: Query<(&AmmoInMag, &MaxAmmo, &PlayerID, &mut TimeSinceStartReload)>, mut ev_reload: EventReader<ReloadEvent>) {
    // Only start a reload if the reload event is read
    for _ in ev_reload.iter() {
        for (ammo_in_mag, max_ammo, id, mut reload_timer) in query.iter_mut() {
            if *id == PlayerID(0) && ammo_in_mag.0 < max_ammo.0 && !reload_timer.reloading {
                reload_timer.reloading = true;
                reload_timer.timer.reset();

            }

        }
    }
}

pub fn use_ability(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, mut query: Query<(&Transform, &mut RequestedMovement, &Ability, &mut AbilityCharge, &mut AbilityCompleted, &mut PlayerSpeed, &mut UsingAbility), With<PlayerID>>, mut ev_use_ability: EventReader<AbilityEvent>, mut map: ResMut<Map>) {
    for _ in ev_use_ability.iter() {
        for (transform, mut requested_movement, ability, mut ability_charge, mut ability_completed, mut speed, mut using_ability) in query.iter_mut() {
            if ability_charge.0.finished() {
                match ability {
                    Ability::Wall => {
                        if requested_movement.speed != 0.0 {
                            let color = Color::rgb_u8(255, 255, 0);

                            let color_handle: Handle<ColorMaterial> = {
                                let mut color_to_return = None;

                                for (id, material_to_return) in materials.iter() {
                                    if color == material_to_return.color {
                                        color_to_return = Some(materials.get_handle(id));

                                    }

                                }


                                if let Some(color) = color_to_return {
                                    color

                                } else {
                                    materials.add(color.into())

                                }
                            };

                            let coords = transform.translation + Vec3::new(25.0 * requested_movement.angle.cos(), 25.0 * requested_movement.angle.sin(), 0.0);

                            let size =
                            if requested_movement.angle.abs() == PI / 2.0 {
                                Vec2::new(50.0, 25.0)

                            } else {
                                Vec2::new(25.0, 50.0)

                            };


                            commands
                                .spawn_bundle(SpriteBundle {
                                    material: color_handle.clone(),
                                    sprite: Sprite::new(size),
                                    transform: Transform {
                                        translation: coords,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                });

                            map.objects.push(
                                MapObject {
                                    coords,
                                    size,
                                    color,
                                    collidable: true,
                                    player_spawn: false,
                                    health: Some(30),

                                }
                            );

                            ability_charge.0.reset();

                        }
                    },
                    Ability::Phase => {
                        requested_movement.speed += 500.0;
                        ability_charge.0.reset();

                    },
                    Ability::Stim => {
                        if !using_ability.0 && ability_charge.0.finished() {
                            speed.0 *= 2.0;
                            ability_completed.0.reset();
                            using_ability.0 = true;

                        }
                    },
                    _ => {},
                }
            }
        }
    }
}

pub fn reset_player_resources(mut query: Query<(&mut AmmoInMag, &MaxAmmo, &mut TimeSinceStartReload, &mut Bursting, &AbilityCompleted, &Ability, &mut UsingAbility, &mut AbilityCharge, &mut PlayerSpeed)>) {
    for (mut ammo_in_mag, max_ammo, mut reload_timer, mut bursting, ability_completed, ability, mut using_ability, mut ability_charge, mut speed) in query.iter_mut() {
        if reload_timer.reloading && reload_timer.timer.finished() {
            ammo_in_mag.0 = max_ammo.0;
            reload_timer.reloading = false;
            bursting.0 = false;


        }

        if using_ability.0 && ability_completed.0.finished() {
            if *ability == Ability::Stim {
                speed.0 /= 2.0;

            }

            using_ability.0 = false;
            ability_charge.0.reset();
        }
    }
}


pub fn set_mouse_coords(mut commands: Commands, wnds: Res<Windows>, camera: Query<&Transform, With<GameCamera>> ) {
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

    // apply the camera transform
    let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

    commands.insert_resource(MousePosition(pos_wld.into()));

}
