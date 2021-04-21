#![allow(clippy::type_complexity)]

mod components;
mod system_labels;
mod map;
mod helper_functions;
mod player_input;

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::sprite::SpriteSettings;

use map::*;
use player_input::*;

use components::*;
use system_labels::*;

// The game will always run at 60 fps
//TODO: Make this a setting
const TIME_STEP: f32 = 1.0 / 60.0;

pub struct GameCamera;

struct AmmoText;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Model {
    Pistol,
    Shotgun,
    Speedball,
    BurstRifle,
    AssaultRifle,
}

#[derive(Bundle, Debug)]
pub struct Gun {
    pub model: Model,
    pub time_since_last_shot: TimeSinceLastShot,
    pub time_since_start_reload: TimeSinceStartReload,
    pub ammo_in_mag: AmmoInMag,
    pub max_ammo: MaxAmmo,
    pub reload_time: ReloadTime,
/*
    pub time_since_start_reload: u128,
    // Shooting's,arguments are the arguments it had previously from the last frame, used for guns that don't just shoot one bullet at a time (like the burst rifle)
    pub shooting: Option<(f32, f32, bool, f32)>,
    pub projectiles_fired: u8,
    pub reloading: bool,
    // Reload time is in miliseconds
    pub reload_time: u16,
    pub damage: u8,
    pub max_distance: f32,
*/

}

/*impl Gun {
    pub fn new(model: Model) -> Gun {
        Gun {
            model,
            // The time since the last shot is set as 0 so that you can start shooting as the start of the game
            time_since_last_shot: 0,
            time_since_start_reload: 0,
            reloading: false,
            reload_time: match model {
                Model::Pistol => 2000,
                Model::Shotgun => 5000,
                Model::Speedball => 3000,
                Model::BurstRifle => 3250,
                Model::AssaultRifle => 3750,
            },
            // Some guns have special shooting behaviors that last over the course of mutliple ticks, which shooting and projectiles_fired take advantage of
            shooting: None,
            projectiles_fired: 0,
            ammo_in_mag: match model {
                Model::Pistol=> 16,
                Model::Shotgun => 8,
                Model::Speedball => 6,
                Model::BurstRifle => 21,
                Model::AssaultRifle => 25,

            },
            max_ammo: match model {
                Model::Pistol => 16,
                Model::Shotgun => 8,
                Model::Speedball => 6,
                Model::BurstRifle => 21,
                Model::AssaultRifle => 25,

            },
            damage: match model {
                Model::Pistol => 45,
                Model::Shotgun => 25,
                Model::Speedball => 1,
                Model::BurstRifle => 13,
                Model::AssaultRifle => 15,

            },
            max_distance: match model {
                Model::Pistol => 900.0,
                Model::Shotgun => 300.0,
                Model::Speedball => 3000.0,
                Model::BurstRifle => 1000.0,
                Model::AssaultRifle => 1000.0,

            }

        }
    }
}*/
//Each player has a unique player id
#[derive(Bundle, Debug)]
pub struct Player {
    pub id: PlayerID,
    pub health: Health,
    pub requested_movement: RequestedMovement,
    pub movement_type: MovementType,
    pub distance_traveled: DistanceTraveled,
    #[bundle]
    pub gun: Gun,

}

impl Player {
    pub fn new(id: u8) -> Player {
        Player {
            id: PlayerID(id),
            health: Health(100),
            requested_movement: RequestedMovement::new(0.0, 0.0),
            movement_type: MovementType::SingleFrame,
            distance_traveled: DistanceTraveled(0.0),
            gun: Gun {
                model: Model::Pistol,
                time_since_last_shot: TimeSinceLastShot(Timer::from_seconds(0.3, false)),
                time_since_start_reload: TimeSinceStartReload {
                    timer: Timer::from_seconds(2.0, false),
                    reloading: false,

                },
                ammo_in_mag: AmmoInMag(16),
                max_ammo: MaxAmmo(16),
                reload_time: ReloadTime(3.0),
            },

        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ProjectileType {
    Regular,
    Speedball,

}

#[derive(Bundle, Debug, PartialEq)]
pub struct Projectile {
    pub distance_traveled: DistanceTraveled,
    pub requested_movement: RequestedMovement,
    pub movement_type: MovementType,
    pub projectile_type: ProjectileType,

}


impl Projectile {
    pub fn new(requested_movement: RequestedMovement, projectile_type: ProjectileType, max_distance: f32) -> Projectile {
        Projectile {
            distance_traveled: DistanceTraveled(0.0),
            requested_movement,
            movement_type: MovementType::StopAfterDistance(max_distance),
            projectile_type

        }
    }
}

pub struct Skins {
    phase: Handle<ColorMaterial>,

}

pub struct ProjectileMaterials {
    regular: Handle<ColorMaterial>,
    //speedball: Handle<ColorMaterial>,

}

// The mouse's position in 2D world coordinates
pub struct MousePosition(Vec2);

fn main() {
    let mut app = App::build();
        // Antialiasing
        app.insert_resource(Msaa { samples: 1 });
        // Since text looks like garbage in browsers without antialiasing, it's higher for WASM by default
        #[cfg(target_arch = "wasm32")]
        app.insert_resource(Msaa { samples: 8 });

        app.insert_resource( WindowDescriptor {
            title: String::from("Necrophaser"),
            vsync: true,
            ..Default::default()

        })
        // Sprite culling doesn't render sprites outside of the camera viewport when enabled
        // It's fairly buggy when rendering many many  sprites (thousands) at the same time, however
        // Frustum culling also doesn't work with more than 1 camera, so it needs to be disabled for split screen
        // Though it does give a performance boost, especially where there are many sprites to render
        // Currently it's disable, since we use the UI camera and the game camera
        .insert_resource(SpriteSettings { frustum_culling_enabled: false })
        //Just checks for possible ambiguouty issue
        //.insert_resource(ReportExecutionOrderAmbiguities)
        .insert_resource(Map::from_bin(include_bytes!("../tiled/map1.custom")))
        // Gotta initialize the mouse position with something, or else the game crashes
        .insert_resource(MousePosition(Vec2::new(0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_event::<ReloadEvent>();

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
        .add_system(timer_system.system().label("tick_timers"))
        .add_system_set(
            // Anything that needds to run at a set framerate goes here (so basically everything in game)
            SystemSet::new()
                .after("tick_timers")
                // Run the game at TIME_STEP per seconds (currently 60)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_1_keyboard_input.system().label(InputFromPlayer).before("reload"))
                .with_system(shoot.system().label(InputFromPlayer))
                .with_system(set_mouse_coords.system().label(InputFromPlayer))
                .with_system(reset_mag.system().label(InputFromPlayer).label("reload"))
                .with_system(start_reload.system().label(InputFromPlayer).label("reload"))
                .with_system(move_objects.system().after(InputFromPlayer))
                .with_system(move_camera.system().after(InputFromPlayer))
                .with_system(update_ui.system().after(InputFromPlayer))
        )
        .run();
}

fn setup_graphics(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(GameCamera);

    commands.insert_resource(Skins {
        phase: materials.add(Color::rgb_u8(100, 242, 84).into()),

    });

    commands.insert_resource(ProjectileMaterials {
        regular: materials.add(Color::rgb_u8(255, 255, 255).into()),
        //speedball: materials.add(Color::rgb_u8(68, 86, 90).into()),

    });

    //Setup the UI
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position: Rect {
                    left: Val::Percent(90.0),

                    ..Default::default()
                },

                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "16".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 45.0,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        value: "/".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 45.0,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        value: "16".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 45.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AmmoText);

}

fn add_players(mut commands: Commands, materials: Res<Skins>) {
    for i in 0..=0 {
        commands
            .spawn_bundle(Player::new(i))
            .insert_bundle(SpriteBundle {
                material: materials.phase.clone(),
                sprite: Sprite::new(Vec2::new(15.0, 15.0)),
                transform: Transform::from_xyz(i as f32 * 25.0 + 1000.0, -750.0, 0.0),
                ..Default::default()
            });

    }
}

// Move objects will first validate whether a movement can be done, and if so move them
fn move_objects(mut commands: Commands, mut movements: Query<(Entity, &mut Transform, &mut RequestedMovement, &MovementType, &mut DistanceTraveled, &Sprite, Option<&ProjectileType>)>, mut map: ResMut<Map>) {
    for (_, mut object, mut movement, movement_type, mut distance_traveled, sprite, _) in movements.iter_mut() {
        // Only do any math if a change has been detected, in order to avoid triggering this event without need
        // Only lets you move if the movement doesn't bump into a wall
        let next_potential_movement = Vec3::new(movement.speed * movement.angle.cos(), movement.speed * movement.angle.sin(), 0.0);

        if movement.speed != 0.0 {
            if !map.collision(object.translation + next_potential_movement, sprite.size, 0) {
                object.translation.x += movement.speed * movement.angle.cos();
                object.translation.y += movement.speed * movement.angle.sin();

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

            } else {
                movement.speed = 0.0;

            }
        }
    }

    // Remove all stopped bullets
    for object in movements.iter_mut() {
        if object.2.speed == 0.0 && object.6.is_some() {
            commands.entity(object.0).despawn_recursive();


        }
    }

}

/// This system ticks all the `Timer` components on entities within the scene
/// using bevy's `Time` resource to get the delta between each update.
fn timer_system(time: Res<Time>, mut timers: Query<(&mut TimeSinceLastShot, &mut TimeSinceStartReload)>) {
    for (mut time_since_last_shot, mut time_since_start_reload) in timers.iter_mut() {
        time_since_last_shot.0.tick(time.delta());

        // If the player is reloading
        if time_since_start_reload.reloading {
            time_since_start_reload.timer.tick(time.delta());

        }

    }
}

fn update_ui(query: Query<(&AmmoInMag, &MaxAmmo, &PlayerID, &TimeSinceStartReload), With<Model>>, mut ammo_text: Query<&mut Text, With<AmmoText>>, mut ammo_style: Query<&mut Style, With<AmmoText>>) {
    let mut ammo_in_mag = 0;
    let mut max_ammo = 0;

    let mut reloading = false;

    for (player_ammo_count, player_max_ammo, id, reload_timer) in query.iter() {
        if *id == PlayerID(0) {
            ammo_in_mag = (*player_ammo_count).0;
            max_ammo = (*player_max_ammo).0;

            reloading = reload_timer.reloading;

            break;

        }
    }

    let mut ammo_text = ammo_text.single_mut().unwrap();
    let mut ammo_pos = ammo_style.single_mut().unwrap();

    if !reloading {
        ammo_text.sections[0].value = ammo_in_mag.to_string();
        ammo_text.sections[1].value = " / ".to_string();
        ammo_text.sections[2].value = max_ammo.to_string();

    } else {
        ammo_text.sections[0].value = "Reloading...".to_string();
        ammo_text.sections[1].value = "".to_string();
        ammo_text.sections[2].value = "".to_string();

        // Since the Reloading text is pretty big, I need to shift it left slightly
        ammo_pos.position.left = Val::Percent(83.0)

    }

}
