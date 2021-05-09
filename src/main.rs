#![allow(clippy::type_complexity)]

//mod bots;
mod components;
mod system_labels;
mod map;
mod helper_functions;
mod menus;
mod player_input;
mod player_attributes;
mod setup_systems;

mod net;

use bevy_networking_turbulence::*;
use bevy::prelude::*;
use bevy::sprite::SpriteSettings;

use serde::{Deserialize, Serialize};

//use bots::*;
use map::*;
use player_input::*;
use helper_functions::{collide, out_of_bounds};

use components::*;
use menus::*;
use player_attributes::*;
use system_labels::*;
use setup_systems::*;

use net::*;
use rand::{thread_rng, Rng};

pub struct GameCamera;

struct AmmoText;
struct AbilityChargeText;
struct GameLogText;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    Settings,

}


#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
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
    pub damage: Damage,

}

struct GameLogs(Vec<GameLog>);

impl GameLogs {
    fn new() -> GameLogs {
        GameLogs(Vec::with_capacity(10))

    }
}

struct GameLog {
    text: TextSection,
    timer: Timer,

}

impl Projectile {
    pub fn new(requested_movement: RequestedMovement, projectile_type: ProjectileType, max_distance: f32, size: Size, player_id: u8, damage: Damage) -> Projectile {
        Projectile {
            distance_traveled: DistanceTraveled(0.0),
            requested_movement,
            movement_type: MovementType::StopAfterDistance(max_distance),
            projectile_type,
            projectile: ProjectileIdent(player_id),
            projectile_size: size,
            damage,

        }
    }
}

pub struct Skins {
    phase: Handle<ColorMaterial>,
    engineer: Handle<ColorMaterial>,
    stim: Handle<ColorMaterial>,
    wall: Handle<ColorMaterial>,

}

pub struct ProjectileMaterials {
    pub regular: Handle<ColorMaterial>,
    pub speedball: Handle<ColorMaterial>,
    // The engineer's bullets are a different color
    pub engineer: Handle<ColorMaterial>,

}

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,

}


// The mouse's position in 2D world coordinates
pub struct MousePosition(Vec2);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShootEvent {
    start_pos: Vec3,
    player_id: u8,
    pos_direction: Vec2,
    health: u8,
    model: Model,
    max_distance: f32,
    recoil_vec: Vec<f32>,
    speed: f32,
    projectile_type: ProjectileType,
    damage: Damage,
    player_ability: Ability,
    size: Vec2,
    reloading: bool,

}

#[derive(Debug)]
pub struct KeyBindings {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub use_ability: KeyCode,
    pub reload: KeyCode,

}

#[derive(Debug, PartialEq)]
pub enum KeyBindingButtons {
    Up,
    Down,
    Left,
    Right,
    UseAbility,
    Reload,
}

#[derive(Debug, PartialEq)]
pub struct SelectedKeyButton(Option<KeyBindingButtons>);

#[derive(Debug, PartialEq)]
pub enum GameMode {
    Deathmatch,

}

pub struct MyPlayerID(Option<PlayerID>);

pub struct LogEvent(String);

fn main() {
    let mut app = App::build();

        #[cfg(feature = "web")]
        {

        };

        // Antialiasing
        app.insert_resource(Msaa { samples: 1 });

        // Since text looks like garbage in browsers without antialiasing, it's higher for WASM by default
        #[cfg(feature = "web")]
        app.insert_resource(Msaa { samples: 8 });

        app.insert_resource( WindowDescriptor {
            title: String::from("Necrophaser"),
            vsync: true,
            ..Default::default()

        });

        // I want the screen size to be smaller on wasm
        #[cfg(feature = "web")]
        app.insert_resource( WindowDescriptor {
            title: String::from("Necrophaser"),
            vsync: true,
            width: 1366.0 * 0.85,
            height: 768.0 * 0.85,
            ..Default::default()

        });
        // Sprite culling doesn't render sprites outside of the camera viewport when enabled
        // It's fairly buggy when rendering many many  sprites (thousands) at the same time, however
        // Frustum culling also doesn't work with more than 1 camera, so it needs to be disabled for split screen
        // Though it does give a performance boost, especially where there are many sprites to render
        // Currently it's disabled, since we use the UI camera and the game camera
        app.insert_resource(SpriteSettings { frustum_culling_enabled: false })

        //Start in the main menu
        .add_state(AppState::MainMenu)

        // Embed the map into the binary
        .insert_resource(Map::from_bin(include_bytes!("../tiled/map1.custom")))
        // Gotta initialize the mouse position with something, or else the game crashes
        .insert_resource(MousePosition(Vec2::new(0.0, 0.0)))
        .insert_resource(MyPlayerID(None))
        .insert_resource(GameMode::Deathmatch)
        .insert_resource(GameLogs::new());

        app.add_plugins(DefaultPlugins)
        .add_plugin(NetworkingPlugin::default())
        .add_event::<NetworkEvent>()
        // Adds some possible events, like reloading and using your ability
        .add_event::<ReloadEvent>()
        .add_event::<ShootEvent>()
        .add_event::<AbilityEvent>()
        .add_event::<LogEvent>();

        //The WebGL2 plugin is only added if we're compiling to WASM
        #[cfg(feature = "web")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);

        app
        // All the materials of the game NEED to be added before everything else
        .add_startup_system(setup_materials.system())
        // The cameras also need to be added first as well
        .add_startup_system(setup_cameras.system())
        .add_startup_system(setup_default_controls.system());
        //.add_startup_system();

        // Initialize InGame
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .label("setup_game_stuff")
                .with_system(setup_game_ui.system())
                .with_system(draw_map.system())
                .with_system(setup_players.system())
                // Set the mouse coordinates initially
                .with_system(set_mouse_coords.system())
                .with_system(setup_networking.system().label("setup_networking"))
                .with_system(setup_id.system().system().label("setup_id"))

        )

        // Run every tick when InGame
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                // Timers should be ticked first
                .with_system(tick_timers.system().before("player_attr").before(InputFromPlayer))
                .with_system(set_mouse_coords.system().label(InputFromPlayer).before("player_attr").before("shoot"))
                .with_system(send_location.system().label(InputFromPlayer).before("player_attr"))
                .with_system(handle_movement_packets.system().label(InputFromPlayer).before("player_attr"))
                .with_system(handle_projectile_packets.system().label(InputFromPlayer).before("player_attr").before("spawn_projectiles"))
                //.with_system(bots.system().label(InputFromPlayer).before("player_attr"))
                .with_system(my_keyboard_input.system().label(InputFromPlayer).before("player_attr"))
                .with_system(shooting_player_input.system().label(InputFromPlayer).label("shoot"))
                .with_system(spawn_projectile.system().label(InputFromPlayer).label("spawn_projectiles").after("shoot"))
                .with_system(reset_player_resources.system().label(InputFromPlayer).label("player_attr"))
                .with_system(start_reload.system().label(InputFromPlayer).label("player_attr"))
                .with_system(use_ability.system().label(InputFromPlayer).label("player_attr"))
                .with_system(move_objects.system().after(InputFromPlayer).label("move_objects"))
                .with_system(dead_players.system().after("move_objects").label("dead_players"))
                .with_system(log_system.system().after("dead_players"))
                .with_system(move_camera.system().after(InputFromPlayer).after("move_objects"))
                .with_system(update_game_ui.system().after(InputFromPlayer).after("move_objects"))
        );

        #[cfg(feature = "native")]
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(handle_server_commands.system())

        );

        #[cfg(feature = "web")]
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(handle_client_commands.system().before("player_attr").before(InputFromPlayer))
                .with_system(request_id.system())

        );

        app.add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup_main_menu.system())

        )
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu)
                .with_system(main_menu_system.system())

        )
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu)
                .with_system(exit_menu.system())

        )
        .add_system_set(
            SystemSet::on_enter(AppState::Settings)
                .with_system(setup_settings.system())

        )

        .add_system_set(
            SystemSet::on_update(AppState::Settings)
                .with_system(settings_system.system())

        )


        .add_system_set(
            SystemSet::on_exit(AppState::Settings)
                .with_system(exit_menu.system())
                .with_system(remove_selected.system())

        )

        .run();
}

//TODO: Turn RequestedMovement into an event
// Move objects will first validate whether a movement can be done, and if so move them
// Probably the biggest function in the entire project, since it's a frankenstein amalgamation of multiple different functions from the original ggez version. It basically does damage for bullets, and moves any object that requested to be moved
fn move_objects(mut commands: Commands, mut player_movements: Query<(&mut Transform, &mut RequestedMovement, &MovementType, &mut DistanceTraveled, &Sprite, &PlayerID, &mut Health, &mut Visible), Without<ProjectileIdent>>, mut projectile_movements: Query<(Entity, &mut Transform, &mut RequestedMovement, &MovementType, &mut DistanceTraveled, &mut Sprite, &ProjectileType, &ProjectileIdent, &mut Damage), (Without<PlayerID>, With<ProjectileIdent>)>,mut map: ResMut<Map>, time: Res<Time>, mut log_event: EventWriter<LogEvent>) {
    let desired_ticks_per_second: f32 = 60.0;

    for (mut object, mut movement, movement_type, mut distance_traveled, sprite, _player_id, health, _visibility) in player_movements.iter_mut() {
        if movement.speed != 0.0 && *health != Health(0){
            // Only lets you move if the movement doesn't bump into a wall
            let next_potential_movement = Vec3::new(movement.speed * movement.angle.cos(), movement.speed * movement.angle.sin(), 0.0);
            // The next potential movement is multipled by the amount of time that's passed since the last frame times how fast I want the game to be, so that the game doesn't run slower even with lag or very fast PC's, so the game moves at the same frame rate no matter the power of each device
            let next_potential_pos = object.translation + (next_potential_movement * desired_ticks_per_second * time.delta_seconds());

            if !map.collision(next_potential_pos, sprite.size, 0)  && !out_of_bounds(next_potential_pos, sprite.size, map.size) {
                object.translation = next_potential_pos;

                match movement_type {
                    // The object moves one frame, and then stops
                    MovementType::SingleFrame => {
                        movement.speed = 0.0;

                    },

                    MovementType::StopAfterDistance(distance_to_stop_at) => {
                        // If an object uses the StopAfterDistance movement type, it MUST have the distance traveled component, or it will crash
                        // Need to get the absolute value of the movement speed, since speed can be negative (backwards)
                        distance_traveled.0 += movement.speed.abs() * desired_ticks_per_second * time.delta_seconds();

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

    for (_, mut object, mut movement, movement_type, mut distance_traveled, mut sprite, projectile_type, shot_from, mut damage) in projectile_movements.iter_mut() {
        if movement.speed != 0.0 {
            // Only lets you move if the movement doesn't bump into a wall
            let next_potential_movement = Vec3::new(movement.speed * movement.angle.cos(), movement.speed * movement.angle.sin(), 0.0);
            // The next potential movement is multipled by the amount of time that's passed since the last frame times how fast I want the game to be, so that the game doesn't run slower even with lag or very fast PC's, so the game moves at the same frame rate no matter the power of each device
            let next_potential_pos = object.translation + (next_potential_movement * desired_ticks_per_second * time.delta_seconds());

            let mut player_collision = false;

            // Check to see if a player-projectile collision takes place
            for (player, _, _, _, player_sprite, player_id, mut health, mut visibility) in player_movements.iter_mut() {
                // Player bullets cannot collide with the player who shot them (thanks @Susorodni for the idea)
                // Checks that players aren't already dead as well lol
                if collide(player.translation, player_sprite.size, next_potential_pos, sprite.size) && player_id.0 != shot_from.0 && *health != Health(0) {
                    if (health.0 as i8 - damage.0 as i8) < 0 {
                        health.0 = 0;
                        visibility.is_visible = false;

                        let mut rng = rand::thread_rng();
                        let pog = rng.gen_range(0..3);
                        if pog == 0 {
                            log_event.send(LogEvent(format!("Player {} got murked", player_id.0 + 1)));
                        } else if pog == 1 {
                            log_event.send(LogEvent(format!("Player {} got gulaged", player_id.0 + 1)));
                        } else {
                            log_event.send(LogEvent(format!("Player {} got sent to the shadow realm", player_id.0 + 1)));
                        }

                        // log_event.send(LogEvent(format!("Player {} got murked", player_id.0 + 1)));

                    } else {
                        health.0 -= damage.0;

                    }

                    player_collision = true;
                    break;

                }

            }

            if !map.collision(next_potential_pos, sprite.size, damage.0) && !player_collision {
                object.translation = next_potential_pos;

                // Gotta make sure that it's both a projectile and has a projectile type, since guns also have a projectile type
                // If you don't do the is_projectile bit, you get a great bug where a player's size will increase as it moves (if they're using the speedball weapon)
                // The speedball's weapon speeds up and gets bigger
                if *projectile_type == ProjectileType::Speedball {
                    movement.speed *= 1.1;
                    sprite.size *= 1.03;

                    if damage.0 <= 75 {
                        damage.0 += (distance_traveled.0 / 60.0 ) as u8;

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
                        distance_traveled.0 += movement.speed.abs() * 60.0 * time.delta_seconds();

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
    for object in projectile_movements.iter_mut() {
        if object.2.speed == 0.0 {
            commands.entity(object.0).despawn_recursive();

        }
    }

}

// This system just deals respawning players
fn dead_players(mut players: Query<(&mut Health, &mut Visible, &mut RespawnTimer), With<PlayerID>>, game_mode: Res<GameMode>) {
    for (mut health, mut visibility, mut respawn_timer) in players.iter_mut() {
        if respawn_timer.0.finished() && *game_mode == GameMode::Deathmatch {
            health.0 = 100;
            respawn_timer.0.reset();
            visibility.is_visible = true;

        }

    }

}

/// This system ticks all the `Timer` components on entities within the scene
/// using bevy's `Time` resource to get the delta between each update.
// Also adds ability charge to each player
fn tick_timers(time: Res<Time>, mut timers: Query<(&mut AbilityCharge, &mut AbilityCompleted, &UsingAbility, &Health, &mut TimeSinceLastShot, &mut TimeSinceStartReload, &mut RespawnTimer)>, mut logs: ResMut<GameLogs>, game_mode: Res<GameMode>, mut ready_to_send_packet: ResMut<ReadyToSendPacket>) {
    for (mut ability_charge, mut ability_completed, using_ability, health, mut time_since_last_shot, mut time_since_start_reload, mut respawn_timer) in timers.iter_mut() {
        time_since_last_shot.0.tick(time.delta());
        ability_charge.0.tick(time.delta());

        // If the player is reloading
        if time_since_start_reload.reloading {
            time_since_start_reload.timer.tick(time.delta());

        }

        if using_ability.0 {
            ability_completed.0.tick(time.delta());

        }

        if *health == Health(0) && *game_mode == GameMode::Deathmatch {
            respawn_timer.0.tick(time.delta());

        }

        for game_log in logs.0.iter_mut() {
            game_log.timer.tick(time.delta());

        }

        ready_to_send_packet.0.tick(time.delta());

    }
}

/*fn bots(mut player_query: Query<(&Transform, &Sprite, &PlayerID, &mut RequestedMovement, &PlayerSpeed)>, mut map: ResMut<Map>) {
    for (coords, sprite, id, mut requested_movement, speed) in player_query.iter_mut() {
        if *id == PlayerID(2) {
            let res = bounce(coords.translation, sprite.size, requested_movement.angle, &mut map);

            requested_movement.angle = res;
            requested_movement.speed = speed.0;

        }

    }

}*/

//TODO: Change this to seperate queries using Without
fn update_game_ui(query: Query<(&AbilityCharge, &AmmoInMag, &MaxAmmo, &PlayerID, &TimeSinceStartReload), With<Model>>, mut ammo_style: Query<&mut Style, With<AmmoText>>,
    mut t: QuerySet<(
        Query<&mut Text, With<AmmoText>>,
        Query<&mut Text, With<AbilityChargeText>>
    )>,
    my_player_id: Res<MyPlayerID>
) {
    if let Some(my_id) = &my_player_id.0 {
        let mut ammo_in_mag = 0;
        let mut max_ammo = 0;

        let mut ability_charge_percent = 0.0;

        let mut reloading = false;

        for (ability_charge, player_ammo_count, player_max_ammo, id, reload_timer) in query.iter() {
            if id.0 == my_id.0 {
                ammo_in_mag = (*player_ammo_count).0;
                max_ammo = (*player_max_ammo).0;

                ability_charge_percent = ability_charge.0.percent() * 100.0;

                reloading = reload_timer.reloading;

                break;

            }
        }

        let mut ammo_text = t.q0_mut().single_mut().unwrap();
        let mut ammo_pos = ammo_style.single_mut().unwrap();

        if !reloading {
            ammo_text.sections[0].value = ammo_in_mag.to_string();
            ammo_text.sections[1].value = " / ".to_string();
            ammo_text.sections[2].value = max_ammo.to_string();

            ammo_pos.position.left = Val::Percent(90.0);

        } else {
            ammo_text.sections[0].value = "Reloading...".to_string();
            ammo_text.sections[1].value = "".to_string();
            ammo_text.sections[2].value = "".to_string();

            // Since the Reloading text is pretty big, I need to shift it left slightly
            ammo_pos.position.left = Val::Percent(83.0);

        }

        let mut ability_charge_text = t.q1_mut().single_mut().unwrap();
        ability_charge_text.sections[0].value = format!("{:.0}%", ability_charge_percent);

        let ability_charge_percent = ability_charge_percent as u8;

        if ability_charge_percent < 50 {
            ability_charge_text.sections[0].style.color = Color::RED;

        } else if (50..100).contains(&ability_charge_percent) {
            ability_charge_text.sections[0].style.color = Color::YELLOW;

        } else if ability_charge_percent == 100 {
            ability_charge_text.sections[0].style.color = Color::GREEN;

        }

    }
}

fn log_system(mut logs: ResMut<GameLogs>, mut game_log: Query<&mut Text, With<GameLogText>>, asset_server: Res<AssetServer>, mut log_event: EventReader<LogEvent>) {
    for log_text in log_event.iter() {
        if logs.0.len() >= 9 {
            logs.0.pop();

        }

        logs.0.insert(0,
            GameLog {
                text: TextSection {
                    value: format!("{}\n", log_text.0.clone()),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 35.0,
                        color: Color::WHITE,
                    }
                },
                timer: Timer::from_seconds(8.0, false),

            }
        );

    }

    let mut text_vec = Vec::with_capacity(10);

    let mut num_of_pops: u8 = 0;

    for log in logs.0.iter().rev() {
        if !log.timer.finished() {
            let mut text = log.text.clone();
            text.style.color.set_a(log.timer.percent_left());

            text_vec.push(text);

        } else {
            num_of_pops += 1;

        }

    }

    while num_of_pops != 0 {
        logs.0.pop();
        num_of_pops -= 1;

    }

    game_log.single_mut().unwrap().sections = text_vec;

}
