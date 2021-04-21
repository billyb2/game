use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::utils::Duration;
use rand::Rng;

use crate::helper_functions::get_angle;

use crate::*;
use crate::components::*;

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
pub fn player_1_keyboard_input(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut RequestedMovement, &PlayerID)>, mut ev_reload: EventWriter<ReloadEvent>) {
    let mut angle = None;

    if keyboard_input.pressed(KeyCode::A) && angle.is_none() {
        match keyboard_input.pressed(KeyCode::W) {
            true => { angle = Some(PI  * 0.75); }
            false => {
                match keyboard_input.pressed(KeyCode::S) {
                    true => { angle = Some(PI * 1.25); }
                    false => { angle = Some(PI); }

                }

            }

        }

    }

    if keyboard_input.pressed(KeyCode::D) && angle.is_none() {
        match keyboard_input.pressed(KeyCode::W) {
            true => { angle = Some(PI  * 0.25); }
            false => {
                match keyboard_input.pressed(KeyCode::S) {
                    true => { angle = Some(PI * 1.75); }
                    false => { angle = Some(0.0); }

                }

            }

        }

    }

    if keyboard_input.pressed(KeyCode::S) && angle.is_none() {
        angle = Some(-PI / 2.0);

    }

    if keyboard_input.pressed(KeyCode::W) && angle.is_none() {
       angle = Some(PI / 2.0);

    }

    if keyboard_input.pressed(KeyCode::R) {
        ev_reload.send(ReloadEvent);

    }

    // Only do a change event if a key has been pressed
    if let Some(angle) = angle {
        for (mut requested_movement, id) in query.iter_mut() {
            if id.0 == 0 {
                requested_movement.angle = angle;
                requested_movement.speed = 10.0;

                break;

            }
        }
    }
}

pub fn shoot(mut commands: Commands, btn: Res<Input<MouseButton>>, materials: Res<ProjectileMaterials>, mouse_pos: Res<MousePosition>, mut query: Query<(&Transform, &PlayerID, &Model, &MaxDistance, &ProjectileType, &RecoilRange, &ProjectileSpeed, &mut TimeSinceLastShot, &mut AmmoInMag, &TimeSinceStartReload, &mut Bursting, &Size)>, mut ev_reload: EventWriter<ReloadEvent>) {
    //TODO: Use just_pressed for non automatic weapons
    // The or shooting bit is for burst rifles
    let mut angle = PI;
    let mut speed = 15.0;
    let mut projectile_type = ProjectileType::Regular;
    let mut max_distance = 900.0;

    let mut shooting = false;

    let mut start_pos_x = mouse_pos.0.x;
    let mut start_pos_y = mouse_pos.0.y;

    let mut recoil_range = 0.0;
    let mut num_of_bullets = 1;

    let mut projectile_size = Size::new(0.5, 0.5);

    for (player, id, gun_model, projectile_max_distance, bullet_type, gun_recoil_range, bullet_speed, mut time_since_last_shot, mut ammo_in_mag, reload_timer, mut bursting, bullet_size) in query.iter_mut() {
        // Checks that player 1 can shoot, and isnt reloading
        if *id == PlayerID(0) {
            if time_since_last_shot.0.finished() && ammo_in_mag.0 > 0 && !reload_timer.reloading && (btn.pressed(MouseButton::Left) || bursting.0) {
                angle = get_angle(mouse_pos.0.x, mouse_pos.0.y, player.translation.x, player.translation.y);

                start_pos_x = player.translation.x;
                start_pos_y = player.translation.y;

                max_distance = projectile_max_distance.0;
                projectile_type = *bullet_type;
                speed = bullet_speed.0;
                recoil_range = gun_recoil_range.0;

                shooting = true;

                projectile_size = *bullet_size;

                if *gun_model == Model::Shotgun {
                    num_of_bullets = 12;

                } else if *gun_model == Model::BurstRifle {
                    if !bursting.0 {
                        bursting.0 = true;
                        time_since_last_shot.0.set_duration(Duration::from_millis(60));

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
            if mouse_pos.0.x <= player.translation.x {
                speed = -speed;

            }

            break;

        }

    }

    if shooting {
        let mut rng = rand::thread_rng();

        for _ in 0..num_of_bullets {
            let recoil =
                if recoil_range == 0.0 {
                    0.0

                } else {
                    rng.gen_range(-recoil_range..recoil_range)

            };

            let movement = RequestedMovement::new(angle + recoil, speed);

            commands
                .spawn_bundle(Projectile::new(movement, projectile_type, max_distance, projectile_size))
                .insert_bundle(SpriteBundle {
                    material: materials.regular.clone(),
                    sprite: Sprite::new(Vec2::new(5.0, 5.0)),
                    transform: Transform::from_xyz(start_pos_x + 2.5, start_pos_y + 2.5, 0.0),
                    ..Default::default()
                });
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

pub fn reset_mag(mut query: Query<(&mut AmmoInMag, &MaxAmmo, &mut TimeSinceStartReload, &mut Bursting)>) {
    for (mut ammo_in_mag, max_ammo, mut reload_timer, mut bursting) in query.iter_mut() {
        if reload_timer.reloading && reload_timer.timer.finished() {
            ammo_in_mag.0 = max_ammo.0;
            reload_timer.reloading = false;
            bursting.0 = false;


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
