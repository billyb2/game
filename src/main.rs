#![deny(clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;
use bevy::tasks::TaskPool;

use bevy_networking_turbulence::*;

use rand::Rng;

use rustc_hash::FxHashMap;

use single_byte_hashmap::*;

use game_lib::*;
use game_lib::map::*;
use game_lib::player_input::*;
use game_lib::menus::*;
use game_lib::net::*;
use game_lib::system_labels::*;
use game_lib::player_attr::*;
use game_lib::setup_systems::*;
use game_lib::components::*;
use game_lib::shaders::*;
use game_lib::logic::move_objects;


fn main() {
    let mut app = App::new();

    let mut rng = rand::thread_rng();

    let map1 = Map::from_bin(include_bytes!("../tiled/map1.custom"));
    let map2 = Map::from_bin(include_bytes!("../tiled/map2.custom"));

    #[cfg(debug_assertions)]
    app
    // Antialiasing
    .insert_resource(Msaa { samples: 8 });

    #[cfg(not(debug_assertions))]
    app
    // Antialiasing is lower for debug builds
    .insert_resource(Msaa { samples: 4 });

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

        maps
    })
    // Gotta initialize the mouse position with something, or else the game crashes
    .insert_resource(MousePosition(Vec2::ZERO))
    // Used to make searches through queries for 1 player much quicker, with some overhead in the beginning of the program
    .insert_resource(MyPlayerID(None))
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
    .add_system(check_assets_ready);

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
    );

    app.run();
}
