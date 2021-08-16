#![feature(variant_count)]
#![feature(const_fn_union)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(core_intrinsics)]
#![feature(drain_filter)]
#![feature(portable_simd)]
#![feature(option_result_unwrap_unchecked)]
#![feature(stmt_expr_attributes)]

#![deny(clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

//mod bots;
mod components;
mod system_labels;
mod map;
mod helper_functions;
mod menus;
mod player_input;
mod player_attr;
mod setup_systems;
mod shaders;
mod net;

use std::collections::BTreeSet;
use std::ops::{Deref, DerefMut};
use std::intrinsics::*;
use core_simd::*;

use bevy_networking_turbulence::*;

use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::renderer::RenderResources;
use bevy::tasks::TaskPool;
#[cfg(feature = "native")]
use bevy::render::draw::OutsideFrustum;

// use bevy_kira_audio::AudioPlugin;

use serde::{Deserialize, Serialize};

use rustc_hash::FxHashMap;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

use rand::seq::SliceRandom;

//use bots::*;
use map::*;
use player_input::*;
use helper_functions::{collide, collide_rect_circle, out_of_bounds};

use components::*;
use menus::*;
use player_attr::*;
use system_labels::*;
use setup_systems::*;
use shaders::*;
use single_byte_hashmap::*;
use net::*;

use rand::Rng;

const DESIRED_TICKS_PER_SECOND: f32 = 60.0;

// Sets up logging for WASM
#[wasm_bindgen]
#[cfg(feature = "web")]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

}

pub struct GameCamera;

struct AmmoText;
struct AbilityChargeText;
struct GameLogText;
struct HealthText;

pub struct GameRelated;

pub struct ScoreUI;
pub struct ChampionText;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Connecting,
    MainMenu,
    GameMenu,
    ContinuePlaying,
    DownloadMapMenu,
    CustomizePlayerMenu,
    InGame,
    Settings,
    CustomizeGame,

}


#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ProjectileType {
    Regular,
    Speedball,
    PulseWave,
    Flame,
    Molotov,
    MolotovFire,
    MolotovLiquid,

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

pub struct Skin {
    player: Handle<ColorMaterial>,
    enemy: Handle<ColorMaterial>,

}

pub struct ProjectileMaterials {
    pub regular: Handle<ColorMaterial>,
    pub speedball: Handle<ColorMaterial>,
    pub engineer: Handle<ColorMaterial>,
    pub molotov: Handle<ColorMaterial>,
    pub molotov_fire: Handle<ColorMaterial>,
    pub molotov_liquid: Handle<ColorMaterial>,

    pub flamethrower1: Handle<ColorMaterial>,
    pub flamethrower2: Handle<ColorMaterial>,
    pub flamethrower3: Handle<ColorMaterial>,
    pub pulsewave: Handle<ColorMaterial>,
}

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,

}

pub struct GameMenuButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,

}


// The mouse's position in world coordinates
pub struct MousePosition(Vec2);

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4b8a-d555-4fc2-ba9f-4c880063ba92"]
pub struct ShaderMousePosition {
    value: Vec2,
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4c8b-d555-4fc2-ba9f-4c880063ba92"]
pub struct WindowSize {
    value: Vec2,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShootEvent {
    start_pos: Vec3,
    player_id: u8,
    pos_direction: Vec2,
    health: f32,
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

//impl Into<(Vec3, u8, Vec2, f32, Model, f32, Vec<f32>, f32, ProjectileType, Damage)

#[derive(Debug)]
pub struct KeyBindings {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub use_ability: KeyCode,
    pub reload: KeyCode,
    pub show_score: KeyCode,
    pub dash: KeyCode,

}

#[derive(Debug, PartialEq)]
pub enum KeyBindingButtons {
    Up,
    Down,
    Left,
    Right,
    UseAbility,
    Reload,
    ShowScore,
}

#[derive(Debug, PartialEq)]
pub struct SelectedKeyButton(Option<KeyBindingButtons>);

#[derive(Debug, PartialEq)]
pub enum GameMode {
    Deathmatch,

}

// The first item of the HashMap is the id of the player, the second is said player's score
pub struct DeathmatchScore(HashMap<u8, u8>);

pub struct MyPlayerID(Option<PlayerID>);

pub struct LogEvent(String);

pub struct DeathEvent(u8);

pub struct OnlinePlayerIDs(BTreeSet<u8>);

// The identifier for the map
pub struct MapCRC32(u32);

pub struct ScoreLimit(u8);

fn main() {
    let mut app = App::new();

    let mut rng = rand::thread_rng();

    let map1 = Map::from_bin(include_bytes!("../tiled/map1.custom"));
    let map2 = Map::from_bin(include_bytes!("../tiled/map2.custom"));
    let map3 = Map::from_bin(include_bytes!("../tiled/map3.custom"));

    #[cfg(debug_assertions)]
    app
    // Antialiasing is lower for debug buidls
    .insert_resource(Msaa { samples: 4 });

    #[cfg(not(debug_assertions))]
    app
    // Antialiasing
    .insert_resource(Msaa { samples: 8 });

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

    app
    //Start in the main menu
    .add_state(AppState::MainMenu)

    .insert_resource(TaskPool::new())
    .insert_resource(MapCRC32(map2.crc32))
    // Embed the map into the binary
    .insert_resource({
        let mut maps = Maps(FxHashMap::default());

        maps.0.insert(map1.crc32, map1);
        maps.0.insert(map2.crc32, map2);
        maps.0.insert(map3.crc32, map3);

        maps
    })
    // Gotta initialize the mouse position with something, or else the game crashes
    .insert_resource(MousePosition(Vec2::ZERO))
    // Used to make searches through queries for 1 player much quicker, with some overhead in the beginning of the program
    .insert_resource(MyPlayerID(None))
    // If a player gets a score of 15 kills, the game ends
    .insert_resource(ScoreLimit(15))
    .insert_resource(GameMode::Deathmatch)
    .insert_resource(GameLogs::new())
    // Randomly generate some aspects of the player
    .insert_resource(rng.gen::<Model>())
    .insert_resource(rng.gen::<Ability>())
    .insert_resource(rng.gen::<Perk>())
    .insert_resource(DeathmatchScore(HashMap::with_capacity_and_hasher(256, BuildHasher::default())));

    app.add_plugins(DefaultPlugins)
    // Using this only temporarily to quit apps on escape
    //.add_system(bevy::input::system::exit_on_esc_system)
    .add_plugin(NetworkingPlugin::default())
    //.add_plugin(AudioPlugin)
    .add_event::<NetworkEvent>()
    // Adds some possible events, like reloading and using your ability
    .add_event::<ReloadEvent>()
    .add_event::<ShootEvent>()
    .add_event::<AbilityEvent>()
    .add_event::<DespawnWhenDead>()
    .add_event::<DeathEvent>()
    .add_event::<LogEvent>();

    //The WebGL2 plugin is only added if we're compiling to WASM
    #[cfg(feature = "web")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app
    // All the materials of the game NEED to be added before everything else
    .add_startup_system(setup_materials)
    // The cameras also need to be added first as well
    .add_startup_system(setup_cameras)
    .add_startup_system(setup_default_controls)
    // Hot asset reloading
    .add_startup_system(setup_asset_loading)
    .add_system(check_assets_ready)
    .add_system(handle_map_object_request)
    .add_system(handle_map_object_data)
    .add_system(handle_map_metadata);

    #[cfg(feature = "native")]
    app.insert_resource(Hosting(true));
    #[cfg(feature = "web")]
    app.insert_resource(Hosting(false));


    #[cfg(feature = "native")]
    app.add_startup_system(setup_listening);

    // Sprite culling
    // For some reason, sprite culling fails on WASM
    #[cfg(feature = "native")]
    app.add_system_to_stage(
        CoreStage::PostUpdate,
        sprite_culling,
    );

    app.add_system_set(
        SystemSet::on_enter(AppState::Connecting)
            .with_system(setup_players)
            .with_system(setup_networking)
            .with_system(setup_id)
            .with_system(setup_connection_menu)

    );

    app.add_system_set(
        SystemSet::on_update(AppState::Connecting)
            .with_system(tick_timers)

    );

    app.add_system_set(
        SystemSet::on_exit(AppState::Connecting)
            .with_system(exit_menu)

    );

    // Initialize InGame
    app.add_system_set(
        SystemSet::on_enter(AppState::InGame)
            .with_system(setup_game_ui)
            // Set the mouse coordinates initially
            .with_system(set_mouse_coords)
            .with_system(draw_map)

    )

    // Run every tick when InGame
    .add_system_set(
        SystemSet::on_update(AppState::InGame)
            // Timers should be ticked first
            .with_system(tick_timers.before("player_attr").before(InputFromPlayer))
            .with_system(set_mouse_coords.label(InputFromPlayer).before("player_attr").before("shoot"))
            .with_system(send_stats.label(InputFromPlayer).before("player_attr"))
            .with_system(handle_stat_packets.label(InputFromPlayer).before("player_attr"))
            .with_system(handle_projectile_packets.label(InputFromPlayer).before("player_attr").before("spawn_projectiles"))
            //.with_system(bots.label(InputFromPlayer).before("player_attr"))
            .with_system(my_keyboard_input.label(InputFromPlayer).before("player_attr"))
            .with_system(set_player_sprite_direction.after(InputFromPlayer))
            .with_system(shooting_player_input.label(InputFromPlayer).label("shoot"))
            .with_system(spawn_projectile.label(InputFromPlayer).label("spawn_projectiles").after("shoot"))
            .with_system(reset_player_resources.label(InputFromPlayer).label("player_attr"))
            .with_system(start_reload.label(InputFromPlayer).label("player_attr"))
            .with_system(use_ability.label(InputFromPlayer).label("player_attr"))
            .with_system(handle_ability_packets.label(InputFromPlayer).label("player_attr"))
            .with_system(reset_player_phasing.after(InputFromPlayer))
            .with_system(move_objects.after(InputFromPlayer).label("move_objects"))
            .with_system(in_game_settings_menu_system.after(InputFromPlayer))
            .with_system(damage_text_system.after("move_objects"))
            .with_system(score_system.after("move_objects"))
            .with_system(handle_damage_packets.label("handle_damage").before("move_objects"))
            .with_system(despawn_destroyed_walls.after("move_objects"))
            .with_system(death_event_system.after("handle_damage").after("move_objects").after(InputFromPlayer).before("dead_players"))
            .with_system(dead_players.after("move_objects").label("dead_players"))
            .with_system(log_system.after("dead_players"))
            .with_system(move_camera.after(InputFromPlayer).after("move_objects"))
            .with_system(update_game_ui.after(InputFromPlayer).after("move_objects"))
    );
    app.add_system_set(
        SystemSet::on_exit(AppState::InGame)
            .with_system(exit_in_game)
            .with_system(disconnect)

    );


    #[cfg(feature = "native")]
    app.add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(handle_server_commands)

    );

    #[cfg(feature = "web")]
    app.add_system_set(
        SystemSet::on_update(AppState::Connecting)
            .with_system(request_player_info)
            .with_system(handle_client_commands)

    );

    #[cfg(feature = "web")]
    app.add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(handle_client_commands.before("player_attr").before(InputFromPlayer))

    );

    app.add_system_set(
        SystemSet::on_enter(AppState::MainMenu)
            .with_system(setup_main_menu)

    )
    .add_system_set(
        SystemSet::on_update(AppState::MainMenu)
            .with_system(main_menu_system)

    )
    .add_system_set(
        SystemSet::on_exit(AppState::MainMenu)
            .with_system(exit_menu)

    )
    .add_system_set(
        SystemSet::on_enter(AppState::GameMenu)
            .with_system(setup_game_menu)

    )
    .add_system_set(
        SystemSet::on_update(AppState::GameMenu)
            .with_system(game_menu_system)

    )
    .add_system_set(
        SystemSet::on_exit(AppState::GameMenu)
            .with_system(exit_menu)

    )
    .add_system_set(
        SystemSet::on_enter(AppState::CustomizePlayerMenu)
            .with_system(setup_customize_player)

    )
    .add_system_set(
        SystemSet::on_update(AppState::CustomizePlayerMenu)
            .with_system(customize_player_system)

    )
    .add_system_set(
        SystemSet::on_exit(AppState::CustomizePlayerMenu)
            .with_system(exit_menu)

    )
    .add_system_set(
        SystemSet::on_enter(AppState::Settings)
            .with_system(setup_settings)

    )

    .add_system_set(
        SystemSet::on_update(AppState::Settings)
            .with_system(settings_system)

    )

    .add_system_set(
        SystemSet::on_exit(AppState::Settings)
            .with_system(exit_menu)
            .with_system(remove_selected)

    )

    .add_system_set(
        SystemSet::on_enter(AppState::CustomizeGame)
            .with_system(setup_customize_game)

    )

    .add_system_set(
        SystemSet::on_update(AppState::CustomizeGame)
            .with_system(customize_game_system)

    )

    .add_system_set(
        SystemSet::on_exit(AppState::CustomizeGame)
            .with_system(exit_menu)
    )

    .add_system_set(
        SystemSet::on_enter(AppState::DownloadMapMenu)
            .with_system(setup_download_map_menu)

    )

    .add_system_set(
        SystemSet::on_update(AppState::DownloadMapMenu)
            .with_system(download_map_system)

    )

    .add_system_set(
        SystemSet::on_exit(AppState::DownloadMapMenu)
            .with_system(exit_menu)
    )

    .run();
}

//TODO: Turn RequestedMovement into an event
//TODO: Maybe make all the bullet collisions into its own seperate system? (for readability and maybe performance)
//TODO: Make it so molotovs are map objects and not bullets
//TODO: Potentially move this fn into it's own module? Like it takes up a good 3/4 of the main.rs file, maybe something called logic.rs or something
// Move objects will first validate whether a movement can be done, and if so move them
// Probably the biggest function in the entire project, since it's a frankenstein amalgamation of multiple different functions from the original ggez version. It basically does damage for bullets, and moves any object that requested to be moved
#[allow(clippy::too_many_arguments)]
fn move_objects(mut commands: Commands, mut player_movements: Query<(Entity, &mut Transform, &mut RequestedMovement, &MovementType, Option<&mut DistanceTraveled>, &Sprite, &PlayerID, &mut Health, &Ability, &mut Visible, &mut PlayerSpeed, &Phasing, &mut Alpha), Without<ProjectileIdent>>, mut projectile_movements: Query<(Entity, &mut Transform, &mut RequestedMovement, &MovementType, Option<&mut DistanceTraveled>, &mut Sprite, &mut ProjectileType, &ProjectileIdent, &mut Damage, &mut Handle<ColorMaterial>, Option<&DestructionTimer>), (Without<PlayerID>, With<ProjectileIdent>)>, mut maps: ResMut<Maps>, map_crc32: Res<MapCRC32>, time: Res<Time>, mut death_event: EventWriter<DeathEvent>, materials: Res<ProjectileMaterials>, mut wall_event: EventWriter<DespawnWhenDead>, mut deathmatch_score: ResMut<DeathmatchScore>, my_player_id: Res<MyPlayerID>, mut net: ResMut<NetworkResource>, player_entity: Res<HashMap<u8, Entity>>, asset_server: Res<AssetServer>) {

    let mut liquid_molotovs: Vec<(Vec2, f32)> = Vec::with_capacity(5);

    let map = maps.0.get_mut(&map_crc32.0).unwrap();


    let stop_after_distance = 
    #[inline(always)]
    |movement_speed: &mut f32, distance_traveled: &mut f32, distance_to_stop_at: f32| {
        *distance_traveled = unsafe { fadd_fast(movement_speed.abs(), *distance_traveled) };

        if *distance_traveled >= distance_to_stop_at {
            *movement_speed = 0.0;

        }
    };

    player_movements.for_each_mut(|(_entity, mut object, mut movement, movement_type, mut distance_traveled, sprite, _player_id, health, _ability, _visible, _player_speed, phasing, _alpha)| {
        if movement.speed != 0.0 && health.0 != 0.0 {
            // The next potential movement is multipled by the amount of time that's passed since the last frame times how fast I want the game to be, so that the game doesn't run slower even with lag or very fast PC's, so the game moves at the same frame rate no matter the power of each device
            let mut lag_compensation = unsafe { fmul_fast(DESIRED_TICKS_PER_SECOND, time.delta_seconds()) };


            if lag_compensation > 4.0 {
                lag_compensation = 4.0;

            }

            let speed = unsafe { fmul_fast(movement.speed, lag_compensation) };

            let angle_trig = f32x2::from_array([movement.angle.cos(), movement.angle.sin()]);
            let translation = f32x2::from_array(object.translation.truncate().to_array());

            let speed_simd = f32x2::splat(speed);

            let next_potential_pos = speed_simd.mul_add(angle_trig, translation);
            let collision = map.collision_no_damage(translation, sprite.size, speed, angle_trig);

            if phasing.0 || (!out_of_bounds(next_potential_pos, sprite.size, map.size)) {
                let potential_pos_array = next_potential_pos.to_array();

                if collision.0 {
                    object.translation.x = potential_pos_array[0];

                }

                if collision.1 {
                    object.translation.y = potential_pos_array[1];

                }

                match movement_type {
                    // The object moves one frame, and then stops
                    MovementType::SingleFrame => {
                        movement.speed = 0.0;

                    },

                    MovementType::StopAfterDistance(distance_to_stop_at) => {
                        stop_after_distance(&mut movement.speed, unsafe { &mut distance_traveled.as_mut().unwrap_unchecked().0 }, *distance_to_stop_at);

                    },
                }

            } else {
                movement.speed = 0.0;

            }
        }
    });

    projectile_movements.for_each_mut(|(_, mut object, mut movement, movement_type, mut distance_traveled, mut sprite, projectile_type, shot_from, mut damage, _, _)| {
        if movement.speed != 0.0 || *projectile_type == ProjectileType::MolotovFire || *projectile_type == ProjectileType::MolotovLiquid {
            if *projectile_type == ProjectileType::MolotovLiquid {
                liquid_molotovs.push((object.translation.truncate(), sprite.size.x));

            }

            let lag_compensation = unsafe { fmul_fast(DESIRED_TICKS_PER_SECOND, time.delta_seconds()) };

            let speed = unsafe { fmul_fast(movement.speed, lag_compensation) };


            let angle_trig = f32x2::from_array([movement.angle.cos(), movement.angle.sin()]);
            let translation = f32x2::from_array(object.translation.truncate().to_array());

            let speed_simd = f32x2::splat(speed);

            let next_potential_pos = speed_simd.mul_add(angle_trig, translation);
            let mut player_collision = false;

            // Check to see if a player-projectile collision takes place
            player_movements.for_each_mut(|(entity, player, _, _, _, player_sprite, player_id, mut health, ability, _visible, mut player_speed, _phasing, mut alpha) |{
                // Player bullets cannot collide with the player who shot them (thanks @Susorodni for the idea)
                // Checks that players aren't already dead as well lol
                // Check to see if a player-projectile collision takes place

                let translation = f32x2::from_array(object.translation.truncate().to_array());

                let collision = collide(translation, sprite.size, player.translation.truncate(), player_sprite.size, movement.speed, angle_trig);

                if health.0 > 0.0 && ((*projectile_type != ProjectileType::MolotovFire && *projectile_type != ProjectileType::MolotovLiquid && (collision.0 || collision.1)) || (*projectile_type == ProjectileType::MolotovFire && collide_rect_circle(player.translation.truncate(), player_sprite.size, next_potential_pos, sprite.size.x))) && (player_id.0 != shot_from.0 || *projectile_type == ProjectileType::MolotovFire) {

                    if *ability == Ability::Cloak && alpha.value != 1.0 {
                        alpha.value = 1.0;

                    }

                    let player_died = (health.0 - damage.0) <= 0.0;

                    commands.spawn_bundle(Text2dBundle {
                        text: Text {
                            sections: vec![
                                TextSection {
                                    value: format!("{:.0}", damage.0),
                                    style: TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 11.0,
                                        color: match player_died {
                                            false => Color::WHITE,
                                            true => Color::RED,

                                        },
                                    },
                                },
                            ],
                            ..Default::default()
                        },
                        transform: Transform::from_translation(Vec2::from_slice(&next_potential_pos.to_array()).extend(5.0)),
                        ..Default::default()

                    })
                    .insert(DamageTextTimer(Timer::from_seconds(2.0, false)));

                    // Players can only do damage to other players if they receive a network event about it, if they don't, then damage can only happen to themselves
                    if let Some(my_player_id) = &my_player_id.0 {
                        if my_player_id.0 == player_id.0 {
                            net.broadcast_message(([my_player_id.0, shot_from.0], damage.0));
                            
                            if player_died {
                                health.0 = 0.0;
                                death_event.send(DeathEvent(player_id.0));
                                // The player who shot the bullet has their score increased 
                                *deathmatch_score.0.get_mut(&shot_from.0).unwrap() += 1;

                            } else {
                                health.0 -= damage.0;

                                if *projectile_type == ProjectileType::PulseWave {
                                    player_speed.0 =  unsafe { fmul_fast(player_speed.0, 0.25) };

                                    commands.entity(entity).insert(SlowedDown(Timer::from_seconds(2.0, false)));

                                }

                            }

                        }
                    }

                    player_collision = true;

                }

            });

            let (wall_collision, health_and_coords) = map.collision(translation, sprite.size, damage.0, speed, f32x2::from_array([movement.angle.cos(), movement.angle.sin()]));

            if let Some((health, coords)) = health_and_coords {
                wall_event.send(DespawnWhenDead {
                    health,
                    coords,

                });

            }

            // Pulsewaves move through walls, but not players
            if !wall_collision && !player_collision || (*projectile_type == ProjectileType::PulseWave && !player_collision) {
                object.translation = Vec2::from_slice(&next_potential_pos.to_array()).extend(3.0);

                // Gotta make sure that it's both a projectile and has a projectile type, since guns also have a projectile type
                // If you don't do the is_projectile bit, you get a great bug where a player's size will increase as it moves (if they're using the speedball weapon)
                // The speedball's weapon speeds up and gets bigger
                if *projectile_type == ProjectileType::Speedball {
                    movement.speed = unsafe { fmul_fast(movement.speed, 1.1) };
                    sprite.size = unsafe { Vec2::new(fmul_fast(sprite.size.x, 1.03), fmul_fast(sprite.size.y, 1.03))};

                    if damage.0 <= 80.0 {
                        damage.0 += distance_traveled.as_ref().unwrap().0  / 60.0;

                    }

                } else if *projectile_type == ProjectileType::Flame && sprite.size.x <= 20.0 {
                    sprite.size *= 1.3;

                }

                match movement_type {
                    // The object moves one frame, and then stops
                    MovementType::SingleFrame => {
                        movement.speed = 0.0;

                    },

                    MovementType::StopAfterDistance(distance_to_stop_at) => {
                        stop_after_distance(&mut movement.speed, unsafe { &mut distance_traveled.as_mut().unwrap_unchecked().0 }, *distance_to_stop_at)

                    },
                }

            } else {
                // Stop any bullets that hit players or walls
                movement.speed = 0.0;

            }

        }
    });


    // Remove all stopped bullets
    projectile_movements.for_each_mut(|(entity, _, req_mov, _, _, mut sprite, mut projectile_type, _, _, mut material, destruction_timer)| {
        if req_mov.speed == 0.0 {
            if *projectile_type == ProjectileType::Molotov {
                // Once the molotov reaches it's destination, or hits a player, it becomes molotov liquid, waiting to be lit by an Inferno player
                *projectile_type.deref_mut() = ProjectileType::MolotovLiquid;
                *material.deref_mut() = materials.molotov_liquid.clone();
                sprite.deref_mut().size = Vec2::new(175.0, 175.0);
                // Molotov liquid disappears after a little while
                commands.entity(entity).insert(DestructionTimer(Timer::from_seconds(45.0, false)));

            } else if likely(*projectile_type != ProjectileType::MolotovLiquid && *projectile_type != ProjectileType::MolotovFire || ((*projectile_type == ProjectileType::MolotovLiquid || *projectile_type == ProjectileType::MolotovFire) && destruction_timer.unwrap().0.finished())) {
                commands.entity(entity).despawn_recursive();

            }
        }
    });

    let mut molotovs_to_be_lit_on_fire: Vec<(Vec2, f32)> = Vec::new();

    // Find molotovs that are to be lit on fire
    projectile_movements.for_each_mut(|(_, proj_coords, _, _, _, sprite, projectile_type, shot_from, _, _, _) |{
        if *projectile_type != ProjectileType::MolotovFire && *projectile_type != ProjectileType::MolotovLiquid {
            // Firstly, find if the player ID is that of an inferno
            let (_entity, _, _, _, _, _, _, _, ability, _, _player_speed, _phasing, _alpha) = player_movements.get_mut(*player_entity.get(&shot_from.0).unwrap()).unwrap();

            for (coords, radius) in liquid_molotovs.iter() {
                if collide_rect_circle(proj_coords.translation.truncate(), sprite.size, f32x2::from_array(coords.to_array()), *radius) {
                    molotovs_to_be_lit_on_fire.push((*coords, *radius));

                }

            }

        }

    });

    // Finally, light any molotovs on fire that need to be lit
    projectile_movements.for_each_mut(|(entity, proj_coords, _, _, _, mut sprite, mut projectile_type, _, mut damage, mut material, _) |{
        if *projectile_type == ProjectileType::MolotovLiquid {

            molotovs_to_be_lit_on_fire.drain_filter(|potential_molotov| {
                let should_light_molotov = proj_coords.translation.truncate() == potential_molotov.0 && (sprite.size.x - potential_molotov.1).abs() < f32::EPSILON;

                if should_light_molotov {
                    // Does 75 damage every second (since there are 60 frames per second)
                    // This might seem excessive, but most players have the sense to run if they catch on fire, so the high damage done forces them to take the fire as a threat instead of just running through it to engage the slow and weak Inferno
                    // Once the molotov is hit by a bullet, it becomes molotov fire

                    *projectile_type.deref_mut() = ProjectileType::MolotovFire;
                    *material.deref_mut() = materials.molotov_fire.clone();
                    damage.deref_mut().0 = 75.0 / 60.0;
                    sprite.deref_mut().size = Vec2::new(250.0, 250.0);
                    commands.entity(entity).insert(DestructionTimer(Timer::from_seconds(5.0, false)));

                }

                // Remove any lit molotovs w. drain filter
                should_light_molotov

            });

        }

    });
}

// Despawns walls that have been destroyed
fn despawn_destroyed_walls(mut commands: Commands, mut wall_event: EventReader<DespawnWhenDead>, mut walls: Query<(Entity, &mut Health, &Transform), With<WallMarker>>) {
    for ev in wall_event.iter() {
        walls.for_each_mut(|(entity, mut health, transform)| {
            if ev.coords == transform.translation.truncate() {
                if ev.health != 0.0 {
                    health.0 = ev.health;

                } else {
                    commands.entity(entity).despawn_recursive();

                }

            }
        });
    }
}

fn death_event_system(mut death_events: EventReader<DeathEvent>, mut players: Query<&mut Visible>, mut log_event: EventWriter<LogEvent>, player_entity: Res<HashMap<u8, Entity>>) {
    for ev in death_events.iter() {
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0..=2);

        let message = match num {
            0 => format!("Player {} got murked", ev.0 + 1),
            1 => format!("Player {} got gulaged", ev.0 + 1),
            2 => format!("Player {} got sent to the shadow realm", ev.0 + 1),
            _ => unimplemented!(),

        };

        let mut visible = players.get_mut(*player_entity.get(&ev.0).unwrap()).unwrap();
        visible.is_visible = false;

        log_event.send(LogEvent(message));

    }
}

// This system just deals respawning players
fn dead_players(mut players: Query<(&mut Health, &mut Transform, &mut Visible, &mut RespawnTimer, &Perk, &PlayerID)>, game_mode: Res<GameMode>, online_player_ids: Res<OnlinePlayerIDs>, task_pool: Res<TaskPool>, maps: Res<Maps>, map_crc32: Res<MapCRC32>) {
    players.par_for_each_mut(&task_pool, 1, |(mut health, mut transform, mut visibility, mut respawn_timer, perk, player_id)| {
        if respawn_timer.0.finished() && *game_mode == GameMode::Deathmatch && online_player_ids.0.contains(&player_id.0) {
            let mut rng = rand::thread_rng();
            let spawn_points = &maps.0.get(&map_crc32.0).unwrap().spawn_points;

            transform.translation = spawn_points.choose(&mut rng).unwrap().extend(100.0);

            health.0 = match perk {
                Perk::HeavyArmor => 125.0,
                Perk::LightArmor => 80.0,
                _ => 100.0,

            };
            respawn_timer.0.reset();
            visibility.is_visible = true;

        }

    });

}

fn score_system(deathmatch_score: Res<DeathmatchScore>, mut champion_text: Query<(&mut Text, &mut Visible), With<ChampionText>>, player_continue_timer: Query<&PlayerContinueTimer>, mut commands: Commands, mut app_state: ResMut<State<AppState>>, score_limit: Res<ScoreLimit>) {
    let deathmatch_score = &deathmatch_score.deref().0;

    let mut display_win_text = |(player_id, _score)| {
        let champion_string = format!("Player {} wins!", player_id + 1);
        let (mut text, mut visible) = champion_text.single_mut().unwrap();

        text.sections[0].value = champion_string;
        visible.is_visible = true;

        if player_continue_timer.is_empty() {
            commands
                .spawn()
                .insert(PlayerContinueTimer(Timer::from_seconds(5.0, false)))
                .insert(GameRelated);

        } else if player_continue_timer.single().unwrap().0.finished() {
            app_state.set(AppState::GameMenu).unwrap();

        }
    };

    #[cfg(feature = "parallel")]
    if let Some((player_id, _score)) = deathmatch_score.into_par_iter().find_any(|(_player_id, score)| **score >= score_limit.0) {
        display_win_text((player_id, _score));

    }

    #[cfg(not(feature = "parallel"))]
    if let Some((player_id, _score)) = deathmatch_score.into_iter().find(|(_player_id, score)| **score >= score_limit.0) {
        display_win_text((player_id, _score));

    }

}

/// This system ticks all the `Timer` components on entities within the scene
/// using bevy's `Time` resource to get the delta between each update.
// Also adds ability charge to each player
fn tick_timers(mut commands: Commands, time: Res<Time>, mut player_timers: Query<(Entity, &Ability, &mut AbilityCharge, &mut AbilityCompleted, &UsingAbility, &Health, &mut TimeSinceLastShot, &mut TimeSinceStartReload, &mut RespawnTimer, &mut DashingInfo, &mut PlayerSpeed, Option<&mut SlowedDown>)>, mut projectile_timers: Query<&mut DestructionTimer>, mut logs: ResMut<GameLogs>, game_mode: Res<GameMode>, mut ready_to_send_packet: ResMut<ReadyToSendPacket>, mut player_continue_timer: Query<&mut PlayerContinueTimer>, mut damage_text_timer: Query<&mut DamageTextTimer>) {
    let delta = time.delta();

    player_timers.for_each_mut(|(entity, ability, mut ability_charge, mut ability_completed, using_ability, health, mut time_since_last_shot, mut time_since_start_reload, mut respawn_timer, mut dashing_info, mut player_speed, slowed_down)| {
        time_since_last_shot.0.tick(delta);

        if health.0 == 0.0 && *game_mode == GameMode::Deathmatch {
            respawn_timer.0.tick(delta);

        } else {
            match dashing_info.dashing {
                false => dashing_info.time_till_can_dash.tick(delta),
                true => dashing_info.time_till_stop_dash.tick(delta),
            };

            match using_ability.0 {
                false => ability_charge.0.tick(delta),
                true => ability_completed.0.tick(delta),
            };

            // If the player is reloading
            if time_since_start_reload.reloading {
                time_since_start_reload.timer.tick(delta);

            }


            if let Some(mut slowed_down_timer) = slowed_down {
                slowed_down_timer.0.tick(delta);

                if slowed_down_timer.0.finished() {
                    player_speed.0 = match ability {
                        Ability::Stim => DEFAULT_PLAYER_SPEED + 1.0,
                        _ => DEFAULT_PLAYER_SPEED,

                    };

                    commands.entity(entity).remove::<SlowedDown>();

                }

            }

        }

    });

    ready_to_send_packet.0.tick(delta);

    for game_log in logs.0.iter_mut() {
        game_log.timer.tick(delta);

    }

    projectile_timers.for_each_mut(|mut destruction_timer| {
        destruction_timer.0.tick(delta);

    });

    if let Ok(mut player_continue_timer) = player_continue_timer.single_mut() {
        player_continue_timer.0.tick(delta);

    }

    damage_text_timer.for_each_mut(|mut damage_text_timer| {
        damage_text_timer.0.tick(delta);

    });
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
fn update_game_ui(query: Query<(&AbilityCharge, &AmmoInMag, &MaxAmmo, &TimeSinceStartReload, &Health), With<Model>>, mut ammo_style: Query<&mut Style, With<AmmoText>>, mut ammo_text: Query<&mut Text, (With<AmmoText>, Without<AbilityChargeText>)>, mut ability_charge_text: Query<&mut Text, (With<AbilityChargeText>, Without<HealthText>)>, mut health_text: Query<&mut Text, (With<HealthText>, Without<AmmoText>)>,
    my_player_id: Res<MyPlayerID>, player_entity: Res<HashMap<u8, Entity>>) {
    if let Some(my_id) = &my_player_id.0 {
        let (ability_charge, player_ammo_count, player_max_ammo, reload_timer, player_health) = query.get(*player_entity.get(&my_id.0).unwrap()).unwrap();

        let ammo_in_mag = (*player_ammo_count).0;
        let max_ammo = (*player_max_ammo).0;

        let ability_charge_percent = ability_charge.0.percent() * 100.0;

        let reloading = reload_timer.reloading;
        let health = player_health.0;

        let mut ammo_text = ammo_text.single_mut().unwrap();
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

        let mut ability_charge_text = ability_charge_text.single_mut().unwrap();
        ability_charge_text.sections[0].value = format!("{:.0}%", ability_charge_percent);

        let ability_charge_percent = ability_charge_percent as u8;

        if ability_charge_percent < 50 {
            ability_charge_text.sections[0].style.color = Color::RED;

        } else if (50..100).contains(&ability_charge_percent) {
            ability_charge_text.sections[0].style.color = Color::YELLOW;

        } else if ability_charge_percent == 100 {
            ability_charge_text.sections[0].style.color = Color::GREEN;

        }

        let mut health_text = health_text.single_mut().unwrap();
        health_text.sections[0].value = format!("Health: {:.0}%", health);

    }
}

fn damage_text_system(mut commands: Commands, mut texts: Query<(Entity, &mut Text, &DamageTextTimer)>) {
    texts.for_each_mut(|(entity, mut text, timer)| {
        if timer.0.finished() {
            commands.entity(entity).despawn_recursive();

        } else {
            let text = &mut text.deref_mut().sections[0];
            text.style.color.set_a(timer.0.percent_left());

        }

    });
}

fn log_system(mut logs: ResMut<GameLogs>, mut game_log: Query<&mut Text, With<GameLogText>>, asset_server: Res<AssetServer>, mut log_event: EventReader<LogEvent>) {
    for log_text in log_event.iter() {
        logs.0.truncate(9);

        logs.0.insert(0,
            GameLog {
                text: TextSection {
                    value: format!("{}\n", log_text.0.clone()),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        // The text size becomes smaller as the actual text becomes larger, so that it will always fit on the screen
                        font_size: 35.0 * (20.0 / log_text.0.len() as f32),
                        color: Color::WHITE,
                    }
                },
                timer: Timer::from_seconds(8.0, false),

            }
        );

    }

    let mut text_vec = Vec::with_capacity(10);

    let mut new_len = logs.0.len();

    for log in logs.0.iter().rev() {
        if !log.timer.finished() {
            let mut text = log.text.clone();
            // Sets the transparency of the text
            text.style.color.set_a(log.timer.percent_left());
            text_vec.push(text);

        } else {
            new_len -= 1;

        }

    }

    logs.0.truncate(new_len);

    let mut game_log = game_log.single_mut().unwrap();

    game_log.sections = text_vec;

}

#[cfg(feature = "native")]
struct MyRect {
    position: Vec2,
    size: Vec2,
}

#[cfg(feature = "native")]
impl MyRect {
    #[inline]
    pub fn is_intersecting(&self, other: MyRect) -> bool {
        self.position.distance(other.position) < (self.get_radius() + other.get_radius())
    }

    #[inline]
    pub fn get_radius(&self) -> f32 {
        let half_size = self.size / Vec2::splat(2.0);
        (half_size.x.powf(2.0) + half_size.y.powf(2.0)).sqrt()
    }
}

// Sprite culling doesn't render sprites outside of the camera viewport when enabled
// Culling doesn't work for WASM builds, atm
// Adapted from Bevy, https://github.com/bevyengine/bevy/blob/cf221f9659127427c99d621b76c8085c4860e2ef/crates/bevy_sprite/src/frustum_culling.rs
/*
MIT License

Copyright (c) 2020 Carter Anderson
*/

#[cfg(feature = "native")]
fn sprite_culling(mut commands: Commands, camera: Query<&Transform, With<GameCamera>>, query: Query<(Entity, &Transform, &Sprite), Without<GameCamera>>, wnds: Res<Windows>, culled_sprites: Query<&OutsideFrustum, With<Sprite>>) {
    let wnd = wnds.get_primary().unwrap();

    let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

    let camera = camera.single().unwrap();

    let camera_size = window_size * camera.scale.truncate();

    let rect = MyRect {
        position: camera.translation.truncate(),
        size: camera_size,
    };

    query.for_each(|(entity, transform, sprite)| {
        let sprite_rect = MyRect {
            position: transform.translation.truncate(),
            size: sprite.size,
        };

        if rect.is_intersecting(sprite_rect) {
            if culled_sprites.get(entity).is_ok() {
                commands.entity(entity).remove::<OutsideFrustum>();

            }

        } else if culled_sprites.get(entity).is_err() {
            commands.entity(entity).insert(OutsideFrustum);

        }

    });

}

pub fn exit_in_game(mut commands: Commands, query: Query<(Entity, &GameRelated)>, player_query: Query<(Entity, &PlayerID)>, projectile_query: Query<(Entity, &ProjectileIdent)>, ui_query: Query<(Entity, &Node)>) {
    query.for_each(|q| {
        commands.entity(q.0).despawn_recursive();

    });

    player_query.for_each(|q| {
        commands.entity(q.0).despawn_recursive();

    });

    projectile_query.for_each(|q| {
        commands.entity(q.0).despawn_recursive();

    });

    ui_query.for_each(|q| {
        commands.entity(q.0).despawn_recursive();

    });
}
