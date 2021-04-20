use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::render::camera::Camera;

use crate::helper_functions::get_angle;

use crate::*;
use crate::components::RequestedMovement;

pub fn move_camera(
    mut q: QuerySet<(
        Query<&mut Transform, With<Camera>>,
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
pub fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut RequestedMovement, &PlayerID)>) {
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

    // Only do a change event if a key has been pressed
    if let Some(angle) = angle {
        for (mut requested_movement, id) in query.iter_mut() {
            if id.0 == 0 {
                requested_movement.angle = angle;
                requested_movement.speed = 15.0;

                break;

            }
        }
    }
}

pub fn shoot(mut commands: Commands, btn: Res<Input<MouseButton>>, materials: Res<Skins>, mouse_pos: Res<MousePosition>, mut players: Query<(&Transform, &PlayerID, &Model, &mut TimeSinceLastShot)>) {
    if btn.just_pressed(MouseButton::Left) {
        let mut angle = PI;
        let mut speed = 15.0;
        let mut projectile_type = ProjectileType::Regular;
        let mut max_distance = 900.0;

        let mut shooting = false;

        let mut start_pos_x = mouse_pos.0.x;
        let mut start_pos_y = mouse_pos.0.y;

        for (player, id, gun_model, mut time_since_last_shot) in players.iter_mut() {
            if *id == PlayerID(0) {
                angle = get_angle(mouse_pos.0.x, mouse_pos.0.y, player.translation.x, player.translation.y);

                start_pos_x = player.translation.x;
                start_pos_y = player.translation.y;


                if *gun_model == Model::Pistol && time_since_last_shot.0.finished() {
                    shooting = true;

                    speed = 12.0;
                    projectile_type = ProjectileType::Regular;

                    max_distance = 900.0;

                    time_since_last_shot.0.reset();

                }






                // Bullets need to travel "backwards" when moving to the left
                if mouse_pos.0.x <= player.translation.x {
                    speed = -speed;

                }

                break;
            }

        }

        if shooting {
            let movement = RequestedMovement::new(angle, speed);

            commands
                .spawn_bundle(Projectile::new(movement, projectile_type, max_distance))
                .insert_bundle(SpriteBundle {
                    material: materials.projectile.clone(),
                    sprite: Sprite::new(Vec2::new(5.0, 5.0)),
                    transform: Transform::from_xyz(start_pos_x + 2.5, start_pos_y + 2.5, 0.0),
                    ..Default::default()
                });
        }


    }
}


pub fn set_mouse_coords(mut commands: Commands, wnds: Res<Windows>, camera: Query<&Transform, With<Camera>> ) {
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
