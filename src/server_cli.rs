#![feature(option_result_unwrap_unchecked)]

#![deny(clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use std::collections::BTreeSet;
use std::convert::TryInto;

use game_lib::*;
use game_types::*;
use game_lib::net::*;
use game_types::player_attr::*;
use map::*;

use bevy::prelude::*;
use bevy::utils::Duration;
use bevy::tasks::TaskPool;
use bevy::core::FixedTimestep;

use bevy_networking_turbulence::*;

use rustc_hash::FxHashMap;

//use rayon::prelude::*;

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
            //.with_system(send_score)
    )
    .run();

}

pub fn setup_networking(mut commands: Commands, mut net: ResMut<NetworkResource>, _hosting: Res<Hosting>) {
    // Registers message types
    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            // (Player ID, [X, y], [Rotation; 4], health, damage_source
            .register::<(u8, [f32; 2], [f32; 4], f32, Option<u8>)>(CLIENT_STATE_MESSAGE_SETTINGS)
            .unwrap();

        builder
            .register::<ShootEvent>(PROJECTILE_MESSAGE_SETTINGS)
            .unwrap();

        builder
            .register::<HashMap<u8, u8>>(SCORE_MESSAGE_SETTINGS)
            .unwrap();

        builder
            .register::<[u8; 3]>(INFO_MESSAGE_SETTINGS)
            .unwrap();

        builder
            .register::<([u8; 2], [f32; 3])>(ABILITY_MESSAGE_SETTINGS)
            .unwrap();

        builder
            .register::<u32>(SET_MAP_SETTINGS)
            .unwrap();

        builder
            .register::<String>(DEBUG_TEXT)
            .unwrap();

    });

    commands.insert_resource(ReadyToSendPacket(Timer::new(Duration::from_millis(15), true)));

}


fn setup_players(mut commands: Commands, maps: Res<Maps>, map_crc32: Res<MapCRC32>) {
    let mut availabie_player_ids: Vec<PlayerID> = Vec::with_capacity(256);
    let online_player_ids: BTreeSet<u8> = BTreeSet::new();
    let mut player_entities: HashMap<u8, Entity> = HashMap::with_capacity_and_hasher(256, BuildHasher::default());

    maps.0.get(&map_crc32.0).unwrap().spawn_points.iter().enumerate().for_each(|(i, coords)| {
        let entity = commands
            .spawn_bundle(Player::new(i.try_into().unwrap(), Ability::Engineer, Perk::ExtendedMag, false))
            .insert_bundle(Gun::new(Model::Pistol, Ability::Engineer, Perk::ExtendedMag))
            .insert(Transform::from_translation(coords.extend(101.0)))
            .id();

        player_entities.insert(i.try_into().unwrap(), entity);
        availabie_player_ids.push(PlayerID(i.try_into().unwrap()));
        
    });
    

    commands.insert_resource(availabie_player_ids);
    commands.insert_resource(OnlinePlayerIDs(online_player_ids));
    commands.insert_resource(player_entities);

}

fn handle_stat_packets(mut net: ResMut<NetworkResource>, mut players: Query<(&mut Transform, &mut Health)>, mut online_player_ids: ResMut<OnlinePlayerIDs>, mut deathmatch_score: ResMut<DeathmatchScore>, player_entity: Res<HashMap<u8, Entity>>) {
    let mut messages_to_send: Vec<(u8, [f32; 2], [f32; 4], f32, Option<u8>)> = Vec::with_capacity(255);
    for (_handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();

        while let Some((player_id, [x, y], [rot_x, rot_y, rot_z, rot_w], new_health, damage_source)) = channels.recv::<(u8, [f32; 2], [f32; 4], f32, Option<u8>)>() {
            // The host broadcasts the locations of all other players
            messages_to_send.push((player_id, [x, y], [rot_x, rot_y, rot_z, rot_w], new_health, damage_source));

            make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, player_id);

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

    for (_handle, connection) in net.connections.iter_mut() {
        if let Some(channels) = connection.channels() {
            while let Some(event) = channels.recv::<ShootEvent>() {
                make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, event.player_id);

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
