// This is basically main.rs, but it's in a seperate file so that the functionality can be shared between main.rs and server_cli.rs
#![feature(variant_count)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(core_intrinsics)]
#![feature(destructuring_assignment)]
#![feature(drain_filter)]
#![feature(portable_simd)]
#![feature(option_result_unwrap_unchecked)]
#![feature(stmt_expr_attributes)]

#![deny(clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub mod components;
pub mod system_labels;
pub mod map;
pub mod menus;
pub mod player_input;
pub mod player_attr;
pub mod setup_systems;
pub mod shaders;
pub mod net;

use std::collections::BTreeSet;
use std::ops::{Deref, DerefMut};
use core_simd::*;

use bevy_networking_turbulence::*;

use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::renderer::RenderResources;
use bevy::tasks::TaskPool;
use bevy::utils::Duration;
#[cfg(feature = "native")]
use bevy::render::draw::OutsideFrustum;

//use bevy_kira_audio::AudioPlugin;

use serde::{Deserialize, Serialize};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

use rand::seq::SliceRandom;

//use bots::*;
use map::*;

use components::*;
use player_attr::*;
use setup_systems::*;
use shaders::*;
use net::*;
use single_byte_hashmap::*;

use rand::Rng;

// Sets up logging for WASM
#[wasm_bindgen]
#[cfg(feature = "web")]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

}

pub struct GameCamera;

pub struct AmmoText;
pub struct AbilityChargeText;
pub struct GameLogText;
pub struct HealthText;

pub struct GameRelated;

pub struct ScoreUI;
pub struct ChampionText;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Connecting,
    MainMenu,
    GameMenu,
    ContinuePlaying,
    CustomizePlayerMenu,
    InGame,
    Settings,
    CustomizeGame,
    DownloadMapMenu,

}


#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ProjectileType {
    Regular,
    Speedball,
    PulseWave,
    TractorBeam,
    Flame,
    Molotov,
    MolotovFire,
    MolotovLiquid,
    Melee,

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

pub struct GameLogs(Vec<GameLog>);

impl GameLogs {
    pub fn new() -> GameLogs {
        GameLogs(Vec::with_capacity(10))

    }
}

pub struct GameLog {
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
    pub beam: Handle<ColorMaterial>,
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
pub struct MousePosition(pub Vec2);

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4b8a-d555-4fc2-ba9f-4c880063ba92"]
pub struct ShaderMousePosition {
    pub value: Vec2,
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4c8b-d555-4fc2-ba9f-4c880063ba92"]
pub struct WindowSize {
    value: Vec2,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShootEvent {
    pub start_pos: Vec3,
    pub player_id: u8,
    pub pos_direction: Vec2,
    pub health: f32,
    pub model: Model,
    pub max_distance: f32,
    pub recoil_vec: Vec<f32>,
    pub speed: f32,
    pub projectile_type: ProjectileType,
    pub damage: Damage,
    pub player_ability: Ability,
    pub size: Vec2,
    pub reloading: bool,

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
pub struct DeathmatchScore(pub HashMap<u8, u8>);

pub struct MyPlayerID(pub Option<PlayerID>);

pub struct LogEvent(pub String);

pub struct DeathEvent(pub u8);

pub struct OnlinePlayerIDs(pub BTreeSet<u8>);

// The identifier for the map
pub struct MapCRC32(pub u32);

// If a player gets a score of 15 kills, the game ends
const SCORE_LIMIT: u8 = 15;

// Despawns walls that have been destroyed
pub fn despawn_destroyed_walls(mut commands: Commands, mut wall_event: EventReader<DespawnWhenDead>, mut walls: Query<(Entity, &mut Health, &Transform), With<WallMarker>>) {
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

pub fn death_event_system(mut death_events: EventReader<DeathEvent>, mut players: Query<&mut Visible>, mut log_event: EventWriter<LogEvent>, player_entity: Res<HashMap<u8, Entity>>) {
    for ev in death_events.iter() {
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0..=2);

        let message = match num {
            0 => format!("Player {} got murked", ev.0 + 1),
            1 => format!("Player {} got gulaged", ev.0 + 1),
            2 => format!("Player {} got sent to the shadow realm", ev.0 + 1),
            _ => String::new(),

        };

        let mut visible = players.get_mut(*player_entity.get(&ev.0).unwrap()).unwrap();
        visible.is_visible = false;

        log_event.send(LogEvent(message));

    }
}

// This system just deals respawning players
pub fn dead_players(mut players: Query<(&mut Health, &mut Transform, &mut Visible, &mut RespawnTimer, &Perk, &PlayerID)>, game_mode: Res<GameMode>, online_player_ids: Res<OnlinePlayerIDs>, task_pool: Res<TaskPool>, maps: Res<Maps>, map_crc32: Res<MapCRC32>) {
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

pub fn score_system(deathmatch_score: Res<DeathmatchScore>, mut champion_text: Query<(&mut Text, &mut Visible), With<ChampionText>>, player_continue_timer: Query<&PlayerContinueTimer>, mut commands: Commands, mut app_state: ResMut<State<AppState>>) {
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
    if let Some((player_id, _score)) = deathmatch_score.into_par_iter().find_any(|(_player_id, score)| **score >= SCORE_LIMIT) {
        display_win_text((player_id, _score));

    }

    #[cfg(not(feature = "parallel"))]
    if let Some((player_id, _score)) = deathmatch_score.into_iter().find(|(_player_id, score)| **score >= SCORE_LIMIT) {
        display_win_text((player_id, _score));

    }

}

/// This system ticks all the `Timer` components on entities within the scene
/// using bevy's `Time` resource to get the delta between each update.
// Also adds ability charge to each player
pub fn tick_timers(mut commands: Commands, time: Res<Time>, mut player_timers: Query<(Entity, &Ability, &mut AbilityCharge, &mut AbilityCompleted, &UsingAbility, &Health, &mut TimeSinceLastShot, &mut TimeSinceStartReload, &mut RespawnTimer, &mut DashingInfo, &mut PlayerSpeed, Option<&mut SlowedDown>)>, mut projectile_timers: Query<&mut DestructionTimer>, mut logs: ResMut<GameLogs>, game_mode: Res<GameMode>, mut ready_to_send_packet: ResMut<ReadyToSendPacket>, mut player_continue_timer: Query<&mut PlayerContinueTimer>, mut damage_text_timer: Query<&mut DamageTextTimer>) {
    let delta = time.delta();

    player_timers.for_each_mut(|(entity, ability, mut ability_charge, mut ability_completed, using_ability, health, mut time_since_last_shot, mut time_since_start_reload, mut respawn_timer, mut dashing_info, mut player_speed, slowed_down)| {
        time_since_last_shot.0.tick(delta);

        // If the player is reloading
        if time_since_start_reload.reloading {
            time_since_start_reload.timer.tick(delta);

        }



        match *ability == Ability::Brute {
            false => match using_ability.0 {
                true => {ability_completed.0.tick(delta);},
                false => {ability_charge.0.tick(delta);},
            },
            // Brute players constantly recharge their abilities, even when using it
            true => {
                if ability_charge.0.elapsed_secs() <= 8.0 {
                    let elapsed_secs = ability_charge.0.elapsed_secs();
                    ability_charge.0.set_elapsed(Duration::from_secs_f32(delta.as_secs_f32() + elapsed_secs))

                }

            },
        };


        if health.0 == 0.0 && *game_mode == GameMode::Deathmatch {
            respawn_timer.0.tick(delta);

        }

        match dashing_info.dashing {
            false => dashing_info.time_till_can_dash.tick(delta),
            true => dashing_info.time_till_stop_dash.tick(delta),
        };

        if let Some(mut slowed_down_timer) = slowed_down {
            slowed_down_timer.0.tick(delta);

            if slowed_down_timer.0.finished() {
                player_speed.0 = match ability {
                    Ability::Stim => DEFAULT_PLAYER_SPEED + 1.0,
                    Ability::Brute => DEFAULT_PLAYER_SPEED * 1.4,
                    _ => DEFAULT_PLAYER_SPEED,

                };

                commands.entity(entity).remove::<SlowedDown>();

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
pub fn update_game_ui(query: Query<(&AbilityCharge, &AmmoInMag, &MaxAmmo, &TimeSinceStartReload, &Health), With<Model>>, mut ammo_style: Query<&mut Style, With<AmmoText>>, mut ammo_text: Query<&mut Text, (With<AmmoText>, Without<AbilityChargeText>)>, mut ability_charge_text: Query<&mut Text, (With<AbilityChargeText>, Without<HealthText>)>, mut health_text: Query<&mut Text, (With<HealthText>, Without<AmmoText>)>,
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

pub fn damage_text_system(mut commands: Commands, mut texts: Query<(Entity, &mut Text, &DamageTextTimer)>) {
    texts.for_each_mut(|(entity, mut text, timer)| {
        if timer.0.finished() {
            commands.entity(entity).despawn_recursive();

        } else {
            let text = &mut text.deref_mut().sections[0];
            text.style.color.set_a(timer.0.percent_left());

        }

    });
}

pub fn log_system(mut logs: ResMut<GameLogs>, mut game_log: Query<&mut Text, With<GameLogText>>, asset_server: Res<AssetServer>, mut log_event: EventReader<LogEvent>) {
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
pub fn sprite_culling(mut commands: Commands, camera: Query<&Transform, With<GameCamera>>, query: Query<(Entity, &Transform, &Sprite), Without<GameCamera>>, wnds: Res<Windows>, culled_sprites: Query<&OutsideFrustum, With<Sprite>>) {
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
