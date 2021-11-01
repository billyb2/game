// This is basically main.rs, but it's in a seperate file so that the functionality can be shared between main.rs and server_cli.rs
#![feature(variant_count)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(core_intrinsics)]
#![feature(destructuring_assignment)]
#![feature(drain_filter)]
#![feature(option_result_unwrap_unchecked)]
#![feature(stmt_expr_attributes)]
#![feature(slice_as_chunks)]
#![feature(format_args_capture)]
#![feature(adt_const_params)]

#![deny(clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(incomplete_features)]

pub mod system_labels;
#[cfg(feature = "graphics")]
pub mod menus;
#[cfg(feature = "graphics")]
pub mod player_input;
#[cfg(feature = "graphics")]
pub mod shaders;

use bevy_networking_turbulence::*;

use rapier2d::na::Vector2;

use bevy::prelude::*;
use bevy::utils::Duration;


use rapier2d::prelude::*;

//use bevy_kira_audio::AudioPlugin;

#[cfg(all(feature = "parallel", feature = "graphics"))]
use rayon::prelude::*;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

use arrayvec::ArrayVec;

use map::*;
use game_types::*;
use game_types::Size;
use single_byte_hashmap::*;
use net::*;

// Sets up logging for WASM
#[wasm_bindgen]
#[cfg(feature = "web")]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

}

#[cfg(feature = "web")]
#[wasm_bindgen(inline_js = "export function screen_width(){return window.innerWidth}export function screen_height(){return window.innerHeight}")]
extern "C" {
    pub fn screen_width() -> f32;
    pub fn screen_height() -> f32;
}

#[derive(Bundle)]
pub struct Projectile {
    pub distance_traveled: DistanceTraveled,
    pub projectile_type: ProjectileType,
    // A general purpose identifier for projectiles, to distinguish between guns and projectiles
    pub projectile: ProjectileIdent,
    pub projectile_size: Size,
    pub damage: Damage,

}

pub trait Logs {
    fn new() -> Self;
    fn is_full(&self) -> bool;
    fn first_mut(&mut self) -> Option<&mut GameLog>;
    fn push_unchecked(&mut self, element: GameLog);
    fn retain<F>(&mut self, f: F)
        where F: FnMut(&mut GameLog) -> bool;
    fn iter(&self) -> std::slice::Iter<'_, GameLog>;

}

#[derive(Component)]
pub struct GameLogs(pub ArrayVec<GameLog, 10>);


#[derive(Component)]
pub struct ChatLogs(pub ArrayVec<GameLog, 10>);

impl Logs for GameLogs {
    fn new() -> Self {
        GameLogs(ArrayVec::new_const())

    }

    fn is_full(&self) -> bool {
        self.0.is_full()

    }

    fn first_mut(&mut self) -> Option<&mut GameLog> {
        self.0.first_mut()
    }

    fn push_unchecked(&mut self, element: GameLog) {
        unsafe { self.0.push_unchecked(element) }

    }

    fn retain<F>(&mut self, f: F)
        where F: FnMut(&mut GameLog) -> bool {
        self.0.retain(f)

    }

    fn iter(&self) -> std::slice::Iter<'_, GameLog> {
        self.0.iter()
    }
}

impl Logs for ChatLogs {
    fn new() -> Self {
        ChatLogs(ArrayVec::new_const())

    }

    fn is_full(&self) -> bool {
        self.0.is_full()

    }

    fn first_mut(&mut self) -> Option<&mut GameLog> {
        self.0.first_mut()
    }

    fn push_unchecked(&mut self, element: GameLog) {
        unsafe { self.0.push_unchecked(element) }

    }

    fn retain<F>(&mut self, f: F)
        where F: FnMut(&mut GameLog) -> bool {
        self.0.retain(f)

    }

    fn iter(&self) -> std::slice::Iter<'_, GameLog> {
        self.0.iter()
    }
}

#[derive(Clone)]
pub struct GameLog {
    pub text: TextSection,
    pub timer: Timer,

}

impl GameLog {
    pub fn new(text: String, size: Option<f32>, text_screen_time: f32, asset_server: &AssetServer) -> Self {
        GameLog {
            text: TextSection {
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    // The text size becomes smaller as the actual text becomes larger, so that it will always fit on the screen
                    font_size: size.unwrap_or(35.0 * (20.0 / text.len() as f32)),
                    color: Color::WHITE,
                },
                value: text,
            },
            timer: Timer::from_seconds(text_screen_time, false),

        }
        
    }
}

impl Projectile {
    pub const fn new(projectile_type: ProjectileType, size: Size, player_id: u8, damage: Damage) -> Self {
        Projectile {
            distance_traveled: DistanceTraveled(0.0),
            projectile_type,
            projectile: ProjectileIdent(player_id),
            projectile_size: size,
            damage,

        }
    }
}

#[derive(Debug, PartialEq)]
pub enum GameMode {
    Deathmatch,

}


// If a player gets a score of 15 kills, the game ends
const SCORE_LIMIT: u8 = 15;


pub fn death_event_system(mut death_events: EventReader<DeathEvent>, mut players: Query<(&mut Visible, &mut RespawnTimer, &ColliderHandleWrapper)>, mut log_event: EventWriter<LogEvent>, player_entity: Res<HashMap<u8, Entity>>, mut collider_set: ResMut<ColliderSet>) {
    death_events.iter().for_each(|ev| {
        let num = fastrand::u8(0..=3);

        let message = match num {
            0 => format!("Player {} got murked", ev.0),
            1 => format!("Player {} got gulaged", ev.0),
            2 => format!("Player {} got sent to the shadow realm", ev.0),
            3 => format!("Player {} died", ev.0),
            _ => unimplemented!(),

        };

        let (mut visible, mut respawn_timer, collider_handle) = players.get_mut(*player_entity.get(&ev.0).unwrap()).unwrap();
        visible.is_visible = false;

        let collider = collider_set.get_mut(collider_handle.0).unwrap();
        collider.set_collision_groups(InteractionGroups::none());


        log_event.send(LogEvent(message));
        respawn_timer.0.reset();

    });
}

// This system just deals respawning players
pub fn dead_players(mut players: Query<(&mut Health, &RigidBodyHandleWrapper, &ColliderHandleWrapper, &mut Visible, &mut RespawnTimer, &Perk, &PlayerID)>, game_mode: Res<GameMode>, online_player_ids: Res<OnlinePlayerIDs>, maps: Res<Maps>, map_crc32: Res<MapCRC32>, mut rigid_body_set: ResMut<RigidBodySet>, mut collider_set: ResMut<ColliderSet>) {
    players.for_each_mut(|(mut health, rigid_body_handle, collider_handle, mut visibility, mut respawn_timer, perk, player_id)| {
        if respawn_timer.0.finished() && *game_mode == GameMode::Deathmatch && online_player_ids.0.contains_key(&player_id.0) {
            let spawn_points = &maps.0.get(&map_crc32.0).unwrap().spawn_points;

            let rigid_body = rigid_body_set.get_mut(rigid_body_handle.0).unwrap();
            let collider = collider_set.get_mut(collider_handle.0).unwrap();


            let new_pos = spawn_points.get(fastrand::usize(..spawn_points.len())).unwrap();

            rigid_body.set_translation(Vector2::new(new_pos.x, new_pos.y).component_div(&Vector2::new(250.0, 250.0)), true);

            collider.set_collision_groups(InteractionGroups::new(0b1000, 0b1111));

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

#[cfg(feature = "graphics")]
pub fn score_system(mut commands: Commands, deathmatch_score: Res<DeathmatchScore>, mut champion_text: Query<(&mut Text, &mut Visible), With<ChampionText>>, player_continue_timer: Query<&PlayerContinueTimer>, mut app_state: ResMut<State<AppState>>) {
    let deathmatch_score = &deathmatch_score.0;

    let display_win_text = 
    #[inline]
    |player_id| {
        let champion_string = format!("Player {} wins!", player_id);
        let (mut text, mut visible) = champion_text.single_mut();

        text.sections[0].value = champion_string;
        visible.is_visible = true;

        if player_continue_timer.is_empty() {
            commands
                .spawn()
                .insert(PlayerContinueTimer(Timer::from_seconds(5.0, false)))
                .insert(GameRelated);

        } else if player_continue_timer.single().0.finished() {
            commands.insert_resource(RigidBodySet::new());
            commands.insert_resource(ColliderSet::new());

            setup_systems::setup_physics(commands);

            app_state.set(AppState::GameMenu).unwrap();

        }
    };

    #[cfg(feature = "parallel")]
    if let Some((player_id, _score)) = deathmatch_score.into_par_iter().find_any(|(_player_id, score)| **score >= SCORE_LIMIT) {
        display_win_text(player_id);

    }

    #[cfg(not(feature = "parallel"))]
    if let Some((player_id, _score)) = deathmatch_score.into_iter().find(|(_player_id, score)| **score >= SCORE_LIMIT) {
        display_win_text(player_id);

    }

}

/// This system ticks all the `Timer` components on entities within the scene
/// using bevy's `Time` resource to get the delta between each update.
// Also adds ability charge to each player
pub fn tick_timers(mut commands: Commands, time: Res<Time>, mut player_timers: Query<(Entity, &Ability, &mut AbilityCharge, &mut AbilityCompleted, &UsingAbility, &Health, &mut TimeSinceLastShot, &mut TimeSinceStartReload, &mut RespawnTimer, &mut DashingInfo, &mut PlayerSpeed, Option<&mut SlowedDown>, &mut CanMelee, &PlayerID)>, mut projectile_timers: Query<&mut DestructionTimer>, mut logs: ResMut<GameLogs>, mut chat: ResMut<ChatLogs>, game_mode: Res<GameMode>, mut player_continue_timer: Query<&mut PlayerContinueTimer>, mut damage_text_timer: Query<&mut DamageTextTimer>, mut explode_timers: Query<&mut ExplodeTimer>, mut ready_to_send_packet: ResMut<ReadyToSendPacket>, mut online_player_ids: ResMut<OnlinePlayerIDs>, mut deathmatch_score: ResMut<DeathmatchScore>, mut available_player_ids: ResMut<Vec<PlayerID>>, mut local_players: ResMut<LocalPlayers>) {
    let delta = time.delta();

    player_timers.for_each_mut(|(entity, ability, mut ability_charge, mut ability_completed, using_ability, health, mut time_since_last_shot, mut time_since_start_reload, mut respawn_timer, mut dashing_info, mut player_speed, slowed_down, mut can_melee, _player_id)| {
        time_since_last_shot.0.tick(delta);

        // If the player is reloading
        if time_since_start_reload.reloading {
            time_since_start_reload.timer.tick(delta);

        }

        can_melee.0.tick(delta);

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

    online_player_ids.0.drain_filter(|id, handle_and_timer| {
        if let Some((handle, timer)) = handle_and_timer {
            timer.tick(delta);
            let timer_finished = timer.finished();

            if timer_finished {
                println!("Player {id} at handle {handle} has timed out!");
                
                let (entity, _, _, _, _, _, _, _, _, _, _, _, _, _) = player_timers.iter_mut().find(|(_entity, _ability, _ability_charge, _ability_completed, _using_ability, _health, _time_since_last_shot, _time_since_start_reload, _respawn_timer, _dashing_info, _player_speed, _slowed_down, _can_melee, player_id)| player_id.0 == *id).unwrap();
                commands.entity(entity).despawn_recursive();
                deathmatch_score.0.remove(id);
                // TODO: Switch to VecDequeue
                available_player_ids.push(PlayerID(*id));

                // TODO: Use CountingSort?

                #[cfg(not(feature = "parallel"))]
                available_player_ids.sort_unstable();

                #[cfg(feature = "parallel")]
                available_player_ids.par_sort_unstable();

                if let Some((index, _l_id)) = local_players.0.iter().enumerate().find(|(_index, l_id)| **l_id == *id) {
                    local_players.0.remove(index);
                }
                
            }

            // Remove players who's timers have finished
            timer_finished
            
         } else {
             false
             
         }
    });

    logs.0.iter_mut().for_each(|l| {l.timer.tick(delta);});
    chat.0.iter_mut().for_each(|l| {l.timer.tick(delta);});

    explode_timers.for_each_mut(|mut t| {t.0.tick(delta);});


    projectile_timers.for_each_mut(|mut destruction_timer| {
        destruction_timer.0.tick(delta);

    });

    if let Ok(mut player_continue_timer) = player_continue_timer.get_single_mut() {
        player_continue_timer.0.tick(delta);

    }

    damage_text_timer.for_each_mut(|mut damage_text_timer| {
        damage_text_timer.0.tick(delta);

    });

    ready_to_send_packet.0.tick(delta);

}

pub fn despawn_game_entities(mut commands: Commands, query: Query<(Entity, &GameRelated), Without<PlayerID>>, player_query: Query<Entity, With<PlayerID>>, projectile_query: Query<(Entity, &ProjectileIdent)>, ui_query: Query<(Entity, &Node)>, sprites: Query<Entity, With<Sprite>>) {
    query.for_each(|q| {
        commands.entity(q.0).despawn_recursive();

    });

    player_query.for_each(|q| commands.entity(q).despawn_recursive());

    sprites.for_each(|q| commands.entity(q).despawn_recursive());

    projectile_query.for_each(|q| {
        commands.entity(q.0).despawn_recursive();

    });

    ui_query.for_each(|q| {
        commands.entity(q.0).despawn_recursive();

    });
}

pub fn reset_game(commands: Commands, mut deathmatch_score: ResMut<DeathmatchScore>, mut my_player_id: ResMut<MyPlayerID>, mut available_player_ids: ResMut<Vec<PlayerID>>, mut online_player_ids: ResMut<OnlinePlayerIDs>, mut local_players: ResMut<LocalPlayers>, mut game_logs: ResMut<GameLogs>, mut chat_logs: ResMut<ChatLogs>, mut typing: ResMut<Typing>) {
    if cfg!(feature = "graphics") || deathmatch_score.0.iter().any(|(_id, score)| *score >= SCORE_LIMIT) {
        deathmatch_score.0.clear();

    }

    available_player_ids.clear();
    online_player_ids.0.clear();
    local_players.0.clear();

    game_logs.0.clear();
    chat_logs.0.clear();

    typing.0 = false;

    setup_systems::setup_physics(commands);

    #[cfg(feature = "graphics")]
    {my_player_id.0 = None;}

}
