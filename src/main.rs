#![feature(drain_filter)]
#![feature(stmt_expr_attributes)]

#![deny(clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(incomplete_features)]

use std::time::Instant;

use bevy::prelude::*;

use rand::Rng;

use rustc_hash::FxHashMap;
use single_byte_hashmap::*;

#[cfg(feature = "native")]
use helper_functions::aabb_check;

use game_lib::*;
use game_lib::player_input::*;
use game_lib::system_labels::*;

use game_logic::*;
use setup_systems::*;
use game_types::*;
use map::*;
use menus::*;
use config::*;
use bots::*;
use net::*;

fn main() {
    let mut app = App::new();

    let mut rng = rand::thread_rng();

    let map1 = Map::from_bin(include_bytes!("../tiled/map1.custom"));
    let map2 = Map::from_bin(include_bytes!("../tiled/map2.custom"));

    #[cfg(debug_assertions)]
    app
    // Antialiasing
    .insert_resource(Msaa { samples: 1 });

    #[cfg(not(debug_assertions))]
    app
    // Antialiasing is higher for release builds
    .insert_resource(Msaa { samples: 1 });

    app.insert_resource( WindowDescriptor {
        title: String::from("Necrophaser"),
        vsync: true,
        ..Default::default()

    });

    #[cfg(feature = "web")]
    let res_scale = (screen_width() as f32 / 1366.0).min(screen_height() as f32 / 768.0) * 0.95;

    // I want the screen size to be smaller on wasm
    #[cfg(feature = "web")]
    app.insert_resource( WindowDescriptor {
        title: String::from("Necrophaser"),
        vsync: true,
        width: 1366.0 * res_scale,
        height: 768.0 * res_scale,
        ..Default::default()

    });

    //TOOD replace these with unwrap_or_else or somethign similar
    let model = match get_data("model") {
        Some(object) => object,
        None => {
            let model = rng.gen::<Model>();
            write_data("model", model);

            model

        }
    };

    // If the player has played the game before, this gets their previous ability/perk/gun. If they haven't, it just randomly generates a new one
    let ability = match get_data("ability") {
        Some(object) => object,
        None => {
            let ability = rng.gen::<Ability>();
            write_data("ability", ability);

            ability

        }
    };

    let perk = match get_data("perk") {
        Some(object) => object,
        None => {
            let perk = rng.gen::<Perk>();
            write_data("perk", perk);

            perk

        }
    };

    let name: PlayerName = match get_data("name") {
        Some(object) => object,
        None => {
            let name = PlayerName::get_random_name();

            write_data("name", name);

            name

        }
    };

    app
    //Start in the main menu
    .add_state(AppState::MainMenu)
    .insert_resource(MapCRC32(map2.crc32))
    // Embed the map into the binary
    .insert_resource({
        let mut maps = Maps(FxHashMap::default());

        maps.0.insert(map1.crc32, map1);
        maps.0.insert(map2.crc32, map2);

        maps
    })
    // Gotta initialize the mouse position with something, or else the game crashes
    .insert_resource(MousePosition(Vec2::ZERO))
    // Used to make searches through queries for 1 player much quicker, with some overhead in the beginning of the program
    .insert_resource(MyPlayerID(None))
    .insert_resource(GameMode::Deathmatch)
    .insert_resource(GameLogs::new())
    .insert_resource(ChatLogs::new())
    .insert_resource(Typing(false))
    .insert_resource(ability)
    .insert_resource(model)
    .insert_resource(perk)
    .insert_resource(name)
    .insert_resource(NumOfBots(0))
    .insert_resource(TickRate {
        last_tick: Instant::now(),
    })
    .insert_resource(DeathmatchScore(HashMap::with_capacity_and_hasher(10, BuildHasher::default())))
    .add_plugins(DefaultPlugins);

    //#[cfg(feature = "graphics")]
    //app
    //.add_plugin(MaterialPlugin::<PlayerMaterial>::default());

    app
    .add_plugin(NetworkingPlugin)
    //.add_plugin(AudioPlugin)
    // Adds some possible events, like reloading and using your ability
    .add_event::<ReloadEvent>()
    .add_event::<ShootEvent>()
    .add_event::<AbilityEvent>()
    .add_event::<DeathEvent>()
    .add_event::<LogEvent>()
    .add_event::<ChatEvent>();

    app
    // All the materials of the game NEED to be added before everything else
    .add_startup_system(setup_materials)
    // The cameras also need to be added first as well
    .add_startup_system(setup_cameras)
    .add_startup_system(setup_default_controls)
    .add_startup_system(setup_physics);

    #[cfg(feature = "native")]
    app.insert_resource(Hosting(true));
    #[cfg(feature = "web")]
    app.insert_resource(Hosting(false));

    // Sprite culling
    // For some reason, sprite culling fails on WASM
    /*#[cfg(feature = "native")]
    app.add_system_to_stage(
        CoreStage::PostUpdate,
        sprite_culling,
    );*/

    app.add_system_set(
        SystemSet::on_enter(AppState::Connecting)
            .with_system(setup_players)
            .with_system(setup_networking)
            .with_system(setup_connection_menu)

    );

    app.add_system_set(
        SystemSet::on_update(AppState::Connecting)
            .with_system(tick_timers)
            .with_system(request_player_info)
            .with_system(handle_client_commands)
            .with_system(connection_menu)

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
            .with_system(add_player_name_text)

    )

    // Run every tick when InGame
    .add_system_set(
        SystemSet::on_update(AppState::InGame)
            // Timers should be ticked first
            .with_system(tick_timers.before("player_attr").before(InputFromPlayer).label("tick_timers"))
            .with_system(explode_grenades.after("tick_timers"))
            .with_system(handle_text_messages)
            .with_system(set_mouse_coords.label(InputFromPlayer).before("player_attr").before("shoot"))
            .with_system(send_stats.label(InputFromPlayer).before("player_attr"))
            .with_system(handle_stat_packets.label(InputFromPlayer).before("player_attr"))
            .with_system(handle_projectile_packets.label(InputFromPlayer).before("player_attr").before("spawn_projectiles"))
            .with_system(handle_client_commands.before("player_attr").before(InputFromPlayer))
            .with_system(handle_score_packets)
            .with_system(my_keyboard_input.label(InputFromPlayer).before("player_attr"))
            .with_system(score_input.label(InputFromPlayer).before("player_attr"))
            .with_system(chat_input.label(InputFromPlayer).before("player_attr"))
            .with_system(handle_bots.label(InputFromPlayer).before("player_attr"))
            .with_system(set_player_sprite_direction.after(InputFromPlayer))
            .with_system(set_player_materials.after(InputFromPlayer))
            .with_system(shooting_player_input.label(InputFromPlayer).label("shoot"))
            .with_system(spawn_projectile.label(InputFromPlayer).label("spawn_projectiles").after("shoot"))
            .with_system(reset_player_resources.label(InputFromPlayer).label("player_attr"))
            .with_system(start_reload.label(InputFromPlayer).label("player_attr"))
            .with_system(use_ability.label(InputFromPlayer).label("player_attr"))
            .with_system(handle_ability_packets.label(InputFromPlayer).label("player_attr"))
            .with_system(reset_player_phasing.after(InputFromPlayer))
            .with_system(sync_physics_pos.before("move_objects").label("sync_physics_pos"))
            .with_system(move_camera.after("sync_physics_pos"))
            .with_system(move_objects.after(InputFromPlayer).label("move_objects"))
            .with_system(calc_tick_rate.after("move_objects"))
            .with_system(proj_distance.after("move_objects"))
            .with_system(increase_speed_and_size.after("move_objects"))
            .with_system(heal_widowmaker_shots.after("move_objects"))
            .with_system(destruction_timer.after("move_objects"))
            .with_system(in_game_settings_menu_system.after(InputFromPlayer))
            .with_system(damage_text_system.after("move_objects"))
            .with_system(score_system.after("move_objects"))
            .with_system(despawn_destroyed_walls.after("move_objects"))
            .with_system(death_event_system.after("move_objects").after(InputFromPlayer).before("dead_players"))
            .with_system(respawn_players.after("move_objects").label("dead_players"))
            .with_system(generic_log_system::<GameLogs, GameLogText, { None }, 8.0, LogEvent>.after("dead_players"))
            .with_system(generic_log_system::<ChatLogs, ChatLogText, { Some(20.0) }, 20.0, ChatEvent>.after(InputFromPlayer))
            .with_system(update_game_ui.after(InputFromPlayer).after("move_objects"))
    );
    app.add_system_set(
        SystemSet::on_exit(AppState::InGame)
            .with_system(despawn_game_entities.label("destroy_entities"))
            .with_system(reset_game)
            .with_system(disconnect)

    );


    #[cfg(feature = "native")]
    app.add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(handle_server_commands)
            .with_system(send_score)
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

    );

    #[cfg(feature = "native")]
    app.add_system_set(
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
    );

    app.run();
}
