#![allow(clippy::type_complexity)]

mod components;
mod system_labels;
mod map;
mod helper_functions;

use std::f32::consts::PI;

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::sprite::SpriteSettings;

use helper_functions::*;
use map::*;

use components::*;
use system_labels::*;

// The game will always run at 60 fps
//TODO: Make this a setting
const TIME_STEP: f32 = 1.0 / 60.0;

//Each player has a unique player id
#[derive(Bundle, Debug, PartialEq)]
struct Player {
    id: PlayerID,
    health: Health,
    requested_movement: RequestedMovement,
    movement_type: MovementType,
    distance_traveled: DistanceTraveled,

}

impl Player {
    fn new(id: u8) -> Player {
        Player {
            id: PlayerID(id),
            health: Health(100),
            requested_movement: RequestedMovement::new(0.0, 0.0),
            movement_type: MovementType::SingleFrame,
            distance_traveled: DistanceTraveled(0.0),

        }
    }
}

#[derive(Bundle, Debug, PartialEq)]
struct Projectile {
    distance_traveled: DistanceTraveled,
    requested_movement: RequestedMovement,
    movement_type: MovementType,

}

impl Projectile {
    fn new(requested_movement: RequestedMovement) -> Projectile {
        Projectile {
            distance_traveled: DistanceTraveled(0.0),
            requested_movement,
            movement_type: MovementType::StopAfterDistance(300.0),

        }
    }
}

struct Skins {
    phase: Handle<ColorMaterial>,
    projectile: Handle<ColorMaterial>,

}

// The mouse's position in 2D world coordinates
struct MousePosition(Vec2);

fn main() {
    let mut app = App::build();
        //Antialiasing
        app.insert_resource(Msaa { samples: 1 })
        .insert_resource( WindowDescriptor {
            vsync: true,
            ..Default::default()

        })
        // Sprite culling doesn't render sprites outside of the camera viewport when enabled
        // It's fairly buggy when rendering many many  sprites (thousands) at the same time, however
        // Frustum culling also doesn't work with more than 1 camera, so it needs to be disabled for split screen
        // Though it does give a performance boost, especially where there are many sprites to render
        .insert_resource(SpriteSettings { frustum_culling_enabled: true })
        //Just checks for possible ambiguouty issue
        //.insert_resource(ReportExecutionOrderAmbiguities)
        .insert_resource(Map::from_bin(include_bytes!("../tiled/map1.custom")))
        .insert_resource(MousePosition(Vec2::new(0.0, 0.0)))
        .add_plugins(DefaultPlugins);

        //The WebGL2 plugin is only added if we're compiling to WASM
        #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);

        app
        .add_startup_system(setup_graphics.system().label("setup_graphics"))
        //Spawning players happens in its own stage since setup_graphics needs to happen first
        .add_startup_stage("setup_game",
        SystemStage::parallel()
            //Players should be draw on on top of objects
            .with_system(draw_map.system())
            .with_system(add_players.system())
            // Set the mouse coordinates initially
            .with_system(set_mouse_coords.system())
        )
        .add_system(set_mouse_coords.system().label("mouse"))
        .add_system_set(
            // Anything that needds to run at a set framerate goes here (so basically everything in game)
            SystemSet::new()
                .after("mouse")
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_player.system().label(MoveReq))
                .with_system(shoot.system().label(MoveReq))
                .with_system(move_objects.system().after(MoveReq))
                .with_system(move_camera.system().after(MoveReq))
        )
        .run();
}

fn setup_graphics(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Skins {
        phase: materials.add(Color::rgb_u8(100, 242, 84).into()),
        projectile: materials.add(Color::rgb_u8(255, 255, 255).into()),

    });

}

fn add_players(mut commands: Commands, materials: Res<Skins>, _asset_server: Res<AssetServer>) {
    for i in 0..=0 {
        commands
            .spawn_bundle(Player::new(i))
            /*.insert_bundle(Text2dBundle {
                text: Text::with_section(
                    100.to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 14.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Top,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                transform: Transform::from_xyz(i as f32 * 25.0, 100.0, 1.0),
                ..Default::default()
            })*/
            .insert_bundle(SpriteBundle {
                material: materials.phase.clone(),
                sprite: Sprite::new(Vec2::new(15.0, 15.0)),
                transform: Transform::from_xyz(i as f32 * 25.0, 100.0, 0.0),
                ..Default::default()
            });

    }


}

fn draw_map(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, map: Res<Map>) {

    // Set the background color to the map's specified color
    commands.insert_resource(ClearColor((*map).background_color));

    let mut i = 0;

    while i < (*map).objects.len() {
        let map_coords = (*map).objects[i].coords;
        let map_object_size =  (*map).objects[i].size;
        let color = (*map).objects[i].color;

        //Either create a new material, or grab a currently existing one
        let color: Handle<ColorMaterial> = {
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

        commands
            .spawn_bundle(SpriteBundle {
                material: color.clone(),
                sprite: Sprite::new(map_object_size),
                transform: Transform {
                    translation: map_coords,
                    ..Default::default()
                },
                ..Default::default()
            });

        i += 1;
    }
}

// Mobe objects will first validate whether a movement can be done, and if so move them
fn move_objects(mut movements: Query<(&mut Transform, &mut RequestedMovement, &MovementType, &mut DistanceTraveled, &Sprite, Changed<RequestedMovement>)>, mut map: ResMut<Map>) {
    for (mut object, mut movement, movement_type, mut distance_traveled, sprite, _) in movements.iter_mut() {
        // Only do any math if a change has been detected, in order to avoid triggering this event without need
        // Only lets you move if the movement doesn't bump into a wall
        let next_potential_movement = Vec3::new(movement.speed * movement.angle.cos(), movement.speed * movement.angle.sin(), 0.0);

        if movement.speed != 0.0 {
            if !map.collision(object.translation + next_potential_movement, sprite.size, 0) {
                object.translation.x += movement.speed * movement.angle.cos();
                object.translation.y += movement.speed * movement.angle.sin();

            } else {
                movement.speed = 0.0;
                println!("Collision!");

            }


            match movement_type {
                // The object moves one frame, and then stops
                MovementType::SingleFrame => {
                    movement.speed = 0.0;

                },

                MovementType::StopAfterDistance(distance_to_stop_at) => {
                    // If an object uses the StopAfterDistance movement type, it MUST have the distance traveled component, or it will crash
                    // Need to get the absolute value of the movement speed, since speed can be negative (backwards)
                    distance_traveled.0 += movement.speed.abs();

                    if distance_traveled.0 >= *distance_to_stop_at {
                        movement.speed = 0.0;

                    }
                },
            }
        }
    }
}

fn move_camera(
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
fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut RequestedMovement, &PlayerID)>) {
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

fn shoot(mut commands: Commands, btn: Res<Input<MouseButton>>, materials: Res<Skins>, mouse_pos: Res<MousePosition>, players: Query<(&Transform, &PlayerID)>) {
    if btn.just_pressed(MouseButton::Left) {
        let mut angle = PI;
        let mut speed = 15.0;

        let mut start_pos_x = mouse_pos.0.x;
        let mut start_pos_y = mouse_pos.0.y;

        for (player, id) in players.iter() {
            if *id == PlayerID(0) {
                angle = get_angle(mouse_pos.0.x, mouse_pos.0.y, player.translation.x, player.translation.y);

                start_pos_x = player.translation.x;
                start_pos_y = player.translation.y;

                // Bullets need to travel "backwards" when moving to the left
                if mouse_pos.0.x <= player.translation.x {
                    speed = -speed;

                }

                break;
            }

        }

        let movement = RequestedMovement::new(angle, speed);

        commands
            .spawn_bundle(Projectile::new(movement))
            .insert_bundle(SpriteBundle {
                material: materials.projectile.clone(),
                sprite: Sprite::new(Vec2::new(5.0, 5.0)),
                transform: Transform::from_xyz(start_pos_x + 2.5, start_pos_y + 2.5, 0.0),
                ..Default::default()
            });

    }
}

/*fn add_projectile(mut commands: Commands, materials: Res<Skins>,) {
    commands
        .spawn_bundle(Projectile::new())
        .insert_bundle(SpriteBundle {
            material: materials.projectile.clone(),
            sprite: Sprite::new(Vec2::new(5.0, 5.0)),
            transform: Transform::from_xyz(50.0, 100.0, 0.0),
            ..Default::default()
        });
}*/

fn set_mouse_coords(mut commands: Commands,
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    camera: Query<&Transform, With<Camera>>
) {
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



