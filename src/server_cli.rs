#![feature(option_result_unwrap_unchecked)]
#![feature(format_args_capture)]

#![deny(clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use game_lib::*;
use game_types::*;
use game_lib::net::*;
use game_lib::setup_systems::setup_players;
use map::*;

use bevy::prelude::*;
use bevy::tasks::TaskPool;
use bevy::core::FixedTimestep;

use bevy_networking_turbulence::*;

use rustc_hash::FxHashMap;

use single_byte_hashmap::BuildHasher;
use single_byte_hashmap::HashMap;

const SIXTY_FRAMES: f64 = 1.0 / 60.0;

fn main() {
    let map1 = Map::from_bin(include_bytes!("../tiled/map1.custom"));
    let map2 = Map::from_bin(include_bytes!("../tiled/map2.custom"));

    #[cfg(feature = "native")]
    App::new()
    .insert_resource(MapCRC32(map2.crc32))
    // Embed the map into the binary
    .insert_resource({
        let mut maps = Maps(FxHashMap::default());

        maps.0.insert(map1.crc32, map1);
        maps.0.insert(map2.crc32, map2);

        maps
    })
    .insert_resource(MyPlayerID(None))
    .insert_resource(GameMode::Deathmatch)
    .add_plugins(MinimalPlugins)
    .insert_resource(DeathmatchScore(HashMap::with_capacity_and_hasher(256, BuildHasher::default())))
    .add_plugin(NetworkingPlugin::default())
    .add_event::<NetworkEvent>()
    .add_event::<LogEvent>()
    .add_event::<AbilityEvent>()
    .add_event::<ShootEvent>()
    .insert_resource(Hosting(true))
    .insert_resource(TaskPool::new())
    .insert_resource(GameLogs::new())
    .add_startup_system(setup_networking)
    .add_startup_system(setup_listening)
    .add_startup_system(setup_players)
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(SIXTY_FRAMES))
            .with_system(tick_timers)
            .with_system(handle_stat_packets)
            .with_system(handle_server_commands)
            .with_system(handle_ability_packets)
            .with_system(handle_projectile_packets)
            .with_system(handle_debug_text)
            .with_system(send_score)
    )
    .run();

}

fn handle_stat_packets(mut net: ResMut<NetworkResource>, mut players: Query<(&mut Transform, &mut Health)>, mut online_player_ids: ResMut<OnlinePlayerIDs>, mut deathmatch_score: ResMut<DeathmatchScore>, player_entity: Res<HashMap<u8, Entity>>) {
    let mut messages_to_send: Vec<(u8, [f32; 2], [f32; 4], f32, f32, Option<u8>)> = Vec::with_capacity(255);
    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();

        while let Some((player_id, [x, y], [rot_x, rot_y, rot_z, rot_w], new_health, alpha, damage_source)) = channels.recv::<(u8, [f32; 2], [f32; 4], f32, f32, Option<u8>)>() {
            // The host broadcasts the locations of all other players
            messages_to_send.push((player_id, [x, y], [rot_x, rot_y, rot_z, rot_w], new_health, alpha, damage_source));

            make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, player_id, handle);

            // Set the location of any local players to the location given
            let (mut transform, mut health) = players.get_mut(*player_entity.get(&player_id).unwrap()).unwrap();

            transform.rotation = Quat::from_xyzw(rot_x, rot_y, rot_z, rot_w);

            transform.translation.x = x;
            transform.translation.y = y;

            // The player has died                    
            if new_health == 0.0 && health.0 != 0.0 && damage_source.is_some() {
                unsafe { *deathmatch_score.0.get_mut(&damage_source.unwrap_unchecked()).unwrap_unchecked() += 1 };

            }

            health.0 = new_health;

        }

    }

    messages_to_send.into_iter().for_each(|m| {net.broadcast_message(m)});
}

fn handle_projectile_packets(mut net: ResMut<NetworkResource>, mut shoot_event: EventWriter<ShootEvent>, mut players: Query<&mut Transform>, mut online_player_ids: ResMut<OnlinePlayerIDs>, mut deathmatch_score: ResMut<DeathmatchScore>, player_entity: Res<HashMap<u8, Entity>>) {
    let mut messages_to_send: Vec<ShootEvent> = Vec::with_capacity(255);

    for (handle, connection) in net.connections.iter_mut() {
        if let Some(channels) = connection.channels() {
            while let Some(event) = channels.recv::<ShootEvent>() {
                make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, event.player_id, handle);

                let mut transform = players.get_mut(*player_entity.get(&event.player_id).unwrap()).unwrap();
                transform.translation = event.start_pos;

                // The host broadcasts the shots fired of all other players
                messages_to_send.push(event.clone());

                shoot_event.send(event);

            }
        }
    }

    for m in messages_to_send.into_iter() {
        net.broadcast_message(m);

    }
}

fn handle_debug_text(mut net: ResMut<NetworkResource>) {
    for (_handle, connection) in net.connections.iter_mut() {
        if let Some(channels) = connection.channels() {
            while let Some(event) = channels.recv::<String>() {   
                println!("{}", event);
            }
        }
    }
}
