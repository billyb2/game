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
    pub max_distance: MaxDistance,
    pub projectile_type: ProjectileType,
    pub projectile_speed: ProjectileSpeed,
    pub recoil_range: RecoilRange,
    pub bursting: Bursting,
    pub projectile_size: Size,
/*
    pub damage: u8,
*/

}

impl Gun {
    fn new(model: Model) -> Gun {
        Gun {
            model,
            time_since_last_shot: match model {
                Model::Pistol => TimeSinceLastShot(Timer::from_seconds(0.5, false)),
                Model::Shotgun => TimeSinceLastShot(Timer::from_seconds(1.5, false)),
                Model::Speedball => TimeSinceLastShot(Timer::from_seconds(1.5, false)),
                Model::BurstRifle => TimeSinceLastShot(Timer::from_seconds(0.5, false)),
                Model::AssaultRifle => TimeSinceLastShot(Timer::from_seconds(0.08, false)),

            },
            time_since_start_reload: TimeSinceStartReload {
                timer: match model {
                    Model::Pistol => Timer::from_seconds(2.0, false),
                    Model::Shotgun => Timer::from_seconds(5.0, false),
                    Model::Speedball => Timer::from_seconds(3.0, false),
                    Model::BurstRifle => Timer::from_seconds(3.25, false),
                    Model::AssaultRifle => Timer::from_seconds(3.75, false),

                },
                reloading: false,

            },
            ammo_in_mag: match model {
                Model::Pistol=> AmmoInMag(16),
                Model::Shotgun => AmmoInMag(8),
                Model::Speedball => AmmoInMag(6),
                Model::BurstRifle => AmmoInMag(21),
                Model::AssaultRifle => AmmoInMag(25),

            },
            max_ammo: match model {
                Model::Pistol=> MaxAmmo(16),
                Model::Shotgun => MaxAmmo(8),
                Model::Speedball => MaxAmmo(6),
                Model::BurstRifle => MaxAmmo(21),
                Model::AssaultRifle => MaxAmmo(25),

            },
            max_distance: match model {
                Model::Pistol => MaxDistance(900.0),
                Model::Shotgun => MaxDistance(300.0),
                Model::Speedball => MaxDistance(3000.0),
                Model::BurstRifle => MaxDistance(1000.0),
                Model::AssaultRifle => MaxDistance(1000.0),

            },

            recoil_range: match model {
                Model::Shotgun => RecoilRange(0.2),
                Model::Speedball => RecoilRange(0.0),
                Model::BurstRifle => RecoilRange(0.04),
                _ => RecoilRange(0.075),

            },
            projectile_type: match model {
                Model::Speedball => ProjectileType::Speedball,
                _ => ProjectileType::Regular,
            },
            projectile_speed: match model {
                Model::Pistol => ProjectileSpeed(12.0),
                Model::Shotgun => ProjectileSpeed(11.0),
                Model::Speedball => ProjectileSpeed(0.25),
                Model::BurstRifle => ProjectileSpeed(12.0),
                Model::AssaultRifle => ProjectileSpeed(12.0),

            },
            projectile_size: Size::new(0.5, 0.5),
            // The bursting component only matters for burst rifles
            bursting: Bursting(false),

        }
    }

}

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
            gun: Gun::new(Model::Speedball),

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
    // A general purpose identifier for projectiles, to distinguish between guns and projectiles
    pub projectile: ProjectileIdent,
    pub projectile_size: Size,

}


impl Projectile {
    pub fn new(requested_movement: RequestedMovement, projectile_type: ProjectileType, max_distance: f32, size: Size) -> Projectile {
        Projectile {
            distance_traveled: DistanceTraveled(0.0),
            requested_movement,
            movement_type: MovementType::StopAfterDistance(max_distance),
            projectile_type,
            projectile: ProjectileIdent,
            projectile_size: size

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
fn move_objects(mut commands: Commands, mut movements: Query<(Entity, &mut Transform, &mut RequestedMovement, &MovementType, &mut DistanceTraveled, &mut Sprite, Option<&ProjectileType>, Option<&ProjectileIdent>)>, mut map: ResMut<Map>) {
    for (_, mut object, mut movement, movement_type, mut distance_traveled, mut sprite, projectile_type, is_projectile) in movements.iter_mut() {
        // Only do any math if a change has been detected, in order to avoid triggering this event without need
        // Only lets you move if the movement doesn't bump into a wall
        let next_potential_movement = Vec3::new(movement.speed * movement.angle.cos(), movement.speed * movement.angle.sin(), 0.0);

        if movement.speed != 0.0 {
            if !map.collision(object.translation + next_potential_movement, sprite.size, 0) {
                object.translation.x += movement.speed * movement.angle.cos();
                object.translation.y += movement.speed * movement.angle.sin();

                // Gotta make sure that it's both a projectile and has a projectile type, since guns also have a projectile type
                // If you don't do the is_projectile bit, you get a great bug where a player's size will increase as it moves (if they're using the speedball weapon)
                if projectile_type.is_some() && is_projectile.is_some() {
                    let projectile_type = projectile_type.unwrap();

                    // The speedball's weapon speeds up and gets bigger
                    if *projectile_type == ProjectileType::Speedball {
                        movement.speed *= 1.1;
                        sprite.size *= 1.03;

                    }
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

            } else {
                movement.speed = 0.0;

            }
        }
    }

    // Remove all stopped bullets
    for object in movements.iter_mut() {
        if object.2.speed == 0.0 && object.7.is_some() {
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
