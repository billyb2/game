#![deny(clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

mod setup;

use std::net::SocketAddr;
use std::convert::TryInto;

//use crate::*;
use bevy_networking_turbulence::*;
use game_types::*;
use map::*;
use single_byte_hashmap::HashMap;

use bevy::prelude::*;
use bevy::utils::Duration;

use rapier2d::prelude::*;
use rapier2d::na::Vector2;

pub use net_tcp::*;
pub use setup::*;

#[cfg(feature = "graphics")]
pub fn send_stats(mut net: ResMut<NetworkResource>, players: Query<(&PlayerID, &Transform, &Health, &DamageSource, &Alpha, &Ability, &UsingAbility, &Model, &PlayerName)>, ready_to_send_packet: Res<ReadyToSendPacket>, local_players: Res<LocalPlayers>, my_player_id: Res<MyPlayerID>, tcp_res: Option<Res<TcpResourceWrapper>>) {
    // Only start sending packets when your ID is set
    if my_player_id.0.is_some() {
        // Rate limiting so that the game sends 66 updates every second
        if ready_to_send_packet.0.finished() {
            players.for_each(|(id, transform, health, damage_source, alpha, ability, using_ability, gun_model, player_name)| {
                if local_players.0.contains(&id.0) {
                    let quat_xyzw: [f32; 4] = transform.rotation.into();

                    let alpha = match *ability == Ability::Cloak && using_ability.0 {
                        true => 0.0,
                        false => alpha.value,

                    };

                    let gun_model: u8 = (*gun_model).into();
                    let ability: u8 = (*ability).into();

                    let message: ClientStateMessage = (id.0, [transform.translation.x, transform.translation.y], quat_xyzw, health.0, alpha, damage_source.0, (gun_model, ability), *player_name);


                    #[cfg(feature = "native")]
                    tcp_res.as_ref().unwrap().send_message(&message, &CLIENT_STATE_MESSAGE_CHANNEL).unwrap();

                    net.broadcast_message(message);

                }

            });

        }
    }
}

pub fn handle_stat_packets(mut net: ResMut<NetworkResource>, mut players: Query<(&mut Transform, &RigidBodyHandleWrapper, &mut Health, &mut Visible, &mut Alpha, &mut Model, &mut Ability, &mut PlayerName)>, my_player_id: Res<MyPlayerID>, _hosting: Res<Hosting>, mut online_player_ids: ResMut<OnlinePlayerIDs>, mut deathmatch_score: ResMut<DeathmatchScore>, player_entity: Res<HashMap<u8, Entity>>, mut death_event: EventWriter<DeathEvent>, mut rigid_body_set: ResMut<RigidBodySet>, tcp_res: Res<TcpResourceWrapper>) {
    #[cfg(feature = "native")]
    let mut messages_to_send: Vec<ClientStateMessage> = Vec::new();
    let my_id = my_player_id.0.unwrap();

    let mut stat_pack_logic = |(player_id, [x, y], [rot_x, rot_y, rot_z, rot_w], new_health, alpha, damage_source, (gun_model, new_ability), new_player_name): ClientStateMessage, handle: &u32| {
        // The host broadcasts the locations of all other players
        #[cfg(feature = "native")]
        if _hosting.0 {
            messages_to_send.push((player_id, [x, y], [rot_x, rot_y, rot_z, rot_w], new_health, alpha, damage_source, (gun_model, new_ability), new_player_name));

        }

        make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, player_id, handle);

        let (mut transform, rigid_body_handle, mut health, mut visible, mut player_alpha, mut model, mut ability, mut player_name) = players.get_mut(*player_entity.get(&player_id).unwrap()).unwrap();
        let rigid_body = rigid_body_set.get_mut(rigid_body_handle.0).unwrap();

        *model = gun_model.into();
        *ability = new_ability.into();

        transform.rotation = Quat::from_xyzw(rot_x, rot_y, rot_z, rot_w);

        if my_id.0 != player_id {
            // The player has died
            if new_health == 0.0 {
                if health.0 != 0.0 && damage_source.is_some() {
                    death_event.send(DeathEvent(player_id));
                    *deathmatch_score.0.get_mut(&damage_source.unwrap()).unwrap() += 1;
                }

            } else {
                visible.is_visible = true;
                player_alpha.value = alpha;   

            }


            health.0 = new_health;
            // Set the location of any local players to the location given
            rigid_body.set_translation(Vector2::new(x, y).component_div(&Vector2::new(250.0, 250.0)), false);
            *player_name = new_player_name;

        }

    };
    

    if let Some(my_id) = &my_player_id.0 {
        for (handle, connection) in net.connections.iter_mut() {
            if let Some(channels) = connection.channels() {
                while let Some(client_state_message) = channels.recv::<ClientStateMessage>() {
                    stat_pack_logic(client_state_message, handle);

                }
            }
        }

    }

    let tcp_messages = tcp_res.process_message_channel::<ClientStateMessage>(&CLIENT_STATE_MESSAGE_CHANNEL);

    match tcp_messages {
        Ok(messages) => {
            messages.into_iter().for_each(|message| {
                // Dummy value, TODO
                stat_pack_logic(message, &0);

            });
        },
        Err(ChannelProcessingError::ChannelNotFound) => (),
        e => panic!("Unhandled error: {:?}", e),
    };

    // Broadcast the location of all players to everyone
    #[cfg(feature = "native")]
    if _hosting.0 {
        for m in messages_to_send.into_iter() {
            tcp_res.send_message(&m, &CLIENT_STATE_MESSAGE_CHANNEL).unwrap();
            net.broadcast_message(m);

        }
    }
}

#[cfg(feature = "web")]
pub fn handle_score_packets(mut net: ResMut<NetworkResource>, mut score: ResMut<DeathmatchScore>) {
    for (_handle, connection) in net.connections.iter_mut() {
        if let Some(channels) = connection.channels() {
            while let Some(new_score) = channels.recv::<HashMap<u8, u8>>() {
                *score = DeathmatchScore(new_score);

            }
        }
    }
}

#[cfg(feature = "native")]
#[inline]
pub fn send_score(mut net: ResMut<NetworkResource>, score: Res<DeathmatchScore>, ready_to_send_packet: Res<ReadyToSendPacket>) {
    if ready_to_send_packet.0.finished() {
        net.broadcast_message(score.0.clone());
    }
}

pub fn handle_ability_packets(mut net: ResMut<NetworkResource>, mut players: Query<(&mut Ability, &RigidBodyHandleWrapper)>, my_player_id: Res<MyPlayerID>, _hosting: Res<Hosting>,  mut ev_use_ability: EventWriter<AbilityEvent>, mut online_player_ids: ResMut<OnlinePlayerIDs>, mut deathmatch_score: ResMut<DeathmatchScore>, player_entity: Res<HashMap<u8, Entity>>, mut rigid_body_set: ResMut<RigidBodySet>) {
    #[cfg(feature = "native")]
    let mut messages_to_send: Vec<AbilityMessage> = Vec::new();

    if let Some(my_id) = &my_player_id.0 {
        for (handle, connection) in net.connections.iter_mut() {
            if let Some(channels) = connection.channels() {
                while let Some(([player_id, ability], [player_x, player_y, angle])) = channels.recv::<AbilityMessage>() {
                    if player_id != my_id.0 {
                        // The host broadcasts the locations of all other players
                        #[cfg(feature = "native")]
                        if _hosting.0 {
                            messages_to_send.push(([player_id, ability], [player_x, player_y, angle]))

                        }

                    }

                    let ability: Ability = ability.into();

                    if player_id != my_id.0 {
                        make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, player_id, handle);
                        let (mut old_ability, rigid_body_handle) = players.get_mut(*player_entity.get(&player_id).unwrap()).unwrap();

                        *old_ability = ability;

                        let rigid_body = rigid_body_set.get_mut(rigid_body_handle.0).unwrap();

                        rigid_body.set_translation(Vector2::new(player_x, player_y).component_div(&Vector2::new(250.0, 250.0)), true);
                        rigid_body.set_rotation(angle, true);

                        if ability == Ability::Wall || ability == Ability::Cloak || ability == Ability::Ghost {
                            ev_use_ability.send(AbilityEvent(player_id));

                        }
                    }
                }
            }
        }
    }

    // Broadcast the location of all players to everyone
    #[cfg(feature = "native")]
    for m in messages_to_send.into_iter() {
        net.broadcast_message(m);

    }
}

pub fn handle_projectile_packets(mut net: ResMut<NetworkResource>, mut shoot_event: EventWriter<ShootEvent>, mut players: Query<&mut Transform>, _hosting: Res<Hosting>, my_player_id: Res<MyPlayerID>, mut online_player_ids: ResMut<OnlinePlayerIDs>, mut deathmatch_score: ResMut<DeathmatchScore>, player_entity: Res<HashMap<u8, Entity>>) {
    #[cfg(feature = "native")]
    let mut messages_to_send: Vec<ShootEvent> = Vec::new();

    if let Some(my_id) = &my_player_id.0 {
        for (handle, connection) in net.connections.iter_mut() {
            if let Some(channels) = connection.channels() {
                while let Some(event) = channels.recv::<ShootEvent>() {
                    if my_id.0 !=  event.player_id {
                        make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, event.player_id, handle);

                        let mut transform = players.get_mut(*player_entity.get(&event.player_id).unwrap()).unwrap();
                        transform.translation = event.start_pos;

                        // The host broadcasts the shots fired of all other players
                        #[cfg(feature = "native")]
                        if _hosting.0 {
                            messages_to_send.push(event.clone())

                        }

                        shoot_event.send(event);

                    }
                }
            }
        }

        #[cfg(feature = "native")]
        if _hosting.0 {
            for m in messages_to_send.into_iter() {
                net.broadcast_message(m);

            }
        }
    }
}


pub fn request_player_info(hosting: Res<Hosting>, my_player_id: Res<MyPlayerID>, my_ability: Res<Ability>, mut net: ResMut<NetworkResource>, mut ready_to_send_packet: ResMut<ReadyToSendPacket>, ability_set: Res<SetAbility>, mut app_state: ResMut<State<AppState>>, mut net_conn_state_text: Query<&mut Text, With<NetConnStateText>>, server_ip: Option<Res<SocketAddr>>, tcp_res: Option<Res<TcpResourceWrapper>>) {
    if !hosting.0 {
        #[cfg(feature = "native")]
        let tcp_res = tcp_res.as_ref().unwrap();

        #[cfg(feature = "native")]
        let connected = tcp_res.is_connected();

        #[cfg(feature = "web")]
        let connected = net.connections.len() > 0;

        // Every few seconds, the client requests an ID from the host server until it gets one
        if ready_to_send_packet.0.finished() && server_ip.is_some() && connected {
            let net_conn_state_text = &mut net_conn_state_text.single_mut().sections[0].value;

            if my_player_id.0.is_none() && !ability_set.0 {
                const REQUEST_ID_MESSAGE: InfoMessage = [0; 3];
                net_conn_state_text.str_write("Requesting ID from server...");
        
                #[cfg(feature = "native")]
                tcp_res.send_message(&REQUEST_ID_MESSAGE, &INFO_MESSAGE_CHANNEL).unwrap();

                #[cfg(feature = "web")]
                net.broadcast_message(REQUEST_ID_MESSAGE);
        
                ready_to_send_packet.0.set_duration(Duration::from_secs(5));
        
            } else if my_player_id.0.is_some() {
                if !ability_set.0 {
                    net_conn_state_text.str_write("Requesting ability from server...");
            
                    let set_ability_message: InfoMessage = [1, (*my_ability).into(), my_player_id.0.as_ref().unwrap().0];

                    #[cfg(feature = "native")]
                    tcp_res.send_message(&set_ability_message, &INFO_MESSAGE_CHANNEL).unwrap();

                    #[cfg(feature = "web")]
                    net.broadcast_message(set_ability_message);

                } else {
                    println!("Starting game");
                    net_conn_state_text.str_write("Starting game!");

                    // Once the client gets an ID and an ability, it starts sending location data every 15 miliseconds
                    ready_to_send_packet.0.set_duration(Duration::from_millis(15));
            
                    app_state.set(AppState::InGame).unwrap();

                }
        
            }
        }
    }

    #[cfg(debug_assertions)]
    if hosting.0 {
        panic!("Should not be hosting at this code point");

    }
}

#[cfg(feature = "native")]
pub fn handle_server_commands(mut net: ResMut<NetworkResource>, mut available_ids: ResMut<Vec<PlayerID>>, hosting: Res<Hosting>, mut players: Query<(&PlayerID, &mut Ability)>, mut online_player_ids: ResMut<OnlinePlayerIDs>, mut log_event: EventWriter<LogEvent>, mut deathmatch_score: ResMut<DeathmatchScore>, map_crc32: Res<MapCRC32>, tcp_res: Res<TcpResourceWrapper>) {
    if !hosting.0 {
        return;
    }
    // First item is the handle, the second is the ID
    let mut messages_to_send: Vec<(u32, InfoMessage)> = Vec::new();

    let mut handle_server_commands_logic = |command: InfoMessage, handle: &u32| {
        // Send a player ID as well as an ability back
        if command[0] == 0 {
            if available_ids.len() > 0 {
                    let player_id = available_ids.remove(0);

                    // Sending back the player id
                    messages_to_send.push((*handle, [0, player_id.0, 0]));

            } else {
                //TODO: Send back a lobby full command
                println!("Lobby full");

            }
        // Respond to the player's ability request
        } else if command[0] == 1 {
            let player_id = command[2];
            let player_ability: Ability = command[1].into();

            make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, player_id, handle);

            println!("Player {} has joined!", player_id);
            log_event.send(LogEvent(format!("Player {} has joined!", player_id)));

            let (_id, mut ability) = players.iter_mut().find(|(id, _ability)| id.0 == player_id).expect(&format!("ID {} not found!", player_id));
            *ability = player_ability;
            // Send the abilities of all players
            messages_to_send.push((*handle, [1, (*ability).into(), player_id]));

        }
    };


    for (handle, connection) in net.connections.iter_mut() {            
        let channels = connection.channels().unwrap();

        while let Some(command) = channels.recv::<InfoMessage>() {
            handle_server_commands_logic(command, handle);

        }
    }

    let tcp_messages: Result<Vec<InfoMessage>, _> = tcp_res.process_message_channel(&INFO_MESSAGE_CHANNEL);

    match tcp_messages {
        // Since the TCP server (currently) cannot send messages to specific clients, we just use the dummy value of 0
        //TODO: Send messages to specific clients for TcpServer
        Ok(messages) => messages.into_iter().for_each(|command| handle_server_commands_logic(command, &0)),
        Err(ChannelProcessingError::ChannelNotFound) => (),
        Err(e) => panic!("Unhandled error: {:?}", e),
    };

    for (handle, message) in messages_to_send.into_iter() {
        #[cfg(feature = "native")]
        {
            tcp_res.send_message(&message, &INFO_MESSAGE_CHANNEL).unwrap();
            tcp_res.send_message(&map_crc32.0, &SET_MAP_CHANNEL).unwrap();
        }

        #[cfg(feature = "web")]
        {
            net.send_message(handle, message).unwrap();
            net.send_message(handle, map_crc32.0).unwrap();
        }

    }
}

pub fn handle_client_commands(mut net: ResMut<NetworkResource>, hosting: Res<Hosting>, mut my_player_id: ResMut<MyPlayerID>, mut players: Query<(&PlayerID, &mut Ability, &mut Handle<ColorMaterial>)>, mut ability_set: ResMut<SetAbility>, mut online_player_ids: ResMut<OnlinePlayerIDs>, mut deathmatch_score: ResMut<DeathmatchScore>, mut map_crc32: ResMut<MapCRC32>, player_entity: Res<HashMap<u8, Entity>>, materials: Res<Skin>, mut maps: ResMut<Maps>, mut app_state: ResMut<State<AppState>>, mut local_players: ResMut<LocalPlayers>, tcp_res: Res<TcpResourceWrapper>) {    
    if hosting.0 {
        return;
    }

    let mut info_message_logic = |command: InfoMessage, handle: &u32| {
        // The set player ID command
        if command[0] == 0 {
            let id = command[1];
            println!("Setting ID");

            my_player_id.0 = Some(PlayerID(id));
            make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, id, handle);
            local_players.0.push(id);


        // The set player ability command
        } else if command[0] == 1 {
            let [_command, ability, player_id] = command;
            let player_ability: Ability = ability.into();

            make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, player_id, handle);
            println!("Setting ability");

            players.for_each_mut(|(id, mut ability, _sprite)| {
                if id.0 == player_id {
                    println!("Set abiility!");
                    *ability = player_ability;

                    /*let (new_helmet_color, new_inner_suit_color) = set_player_colors(&player_ability);

                        *helmet_color = new_helmet_color;
                        *inner_suit_color = new_inner_suit_color;*/

                    ability_set.0 = true;

                }

            });

        }
    };

    let mut map_u32_logic = |new_crc32: u32| {
        if new_crc32 != map_crc32.0 {
            // If the map doensn't currently exist yet, start downloading a new one
            // This feature is still broken lol
            if maps.0.get(&new_crc32).is_none() {
                maps.0.insert(
                    new_crc32,
                    Map {
                        name: String::new(),
                        objects: Vec::new(),
                        size: Vec2::ZERO,
                        spawn_points: Vec::new(),
                        crc32: new_crc32,
                        background_color: Color::BLACK,

                    }
                );

                app_state.set(AppState::DownloadMapMenu).unwrap();

            } 

            map_crc32.0 = new_crc32;
        }

    };

    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();

        while let Some(command) = channels.recv::<InfoMessage>() {
            info_message_logic(command, handle);

        }

        while let Some(new_crc32) = channels.recv::<u32>() {
            map_u32_logic(new_crc32);

        }

    }


    #[cfg(feature = "native")]
    {
        use rayon::join;

        // Since these two tasks are completely independent, they can safely be run in parallel
        join(
            || {
                let info_messages: Result<Vec<InfoMessage>, _> = tcp_res.process_message_channel(&INFO_MESSAGE_CHANNEL);
                match info_messages {
                    // Dummy handle value
                    //TODO: feed real value
                    Ok(messages) => messages.into_iter().for_each(|m| info_message_logic(m, &0)),
                    Err(ChannelProcessingError::ChannelNotFound) => (),
                    Err(e) => panic!("Unhandled error: {:?}", e),
                }
            },
            || {
                let map_messages: Result<Vec<u32>, _> = tcp_res.process_message_channel(&SET_MAP_CHANNEL);

                match map_messages {
                    Ok(messages) => messages.into_iter().for_each(|m| map_u32_logic(m)),
                    Err(ChannelProcessingError::ChannelNotFound) => (),
                    Err(e) => panic!("Unhandled error: {:?}", e),
                }
            },

        );
        
    }

}

pub fn handle_map_object_request(mut net: ResMut<NetworkResource>, maps: Res<Maps>) {
    let mut messages_to_send = Vec::new();

    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();

        while let Some((crc32, index)) = channels.recv::<(u32, u64)>() {
            // Normally, I just unwrap when getting a map object, but in this case, sending a fake CRC32 would be a really easy way to crash any host or client, so I'm just preparing for that
            //TODO: Add sending network error messages for bad requests
            if let Some(map) = maps.0.get(&crc32) {
                let index_usize: usize = index.try_into().unwrap();
                let map_object = &map.objects[index_usize];

                messages_to_send.push((*handle, (crc32, index, map_object.to_bin())));
                println!("crc32: {}, index: {}", crc32, index);
            }

        }

    }

    messages_to_send.into_iter().for_each(|(handle, message)| {net.send_message(handle, message).unwrap();} );

}

pub fn handle_map_object_data(mut net: ResMut<NetworkResource>, mut maps: ResMut<Maps>) {
    const MAP_OBJECT_DEFAULT: MapObject = MapObject::default();

    for (_handle, connection) in net.connections.iter_mut() {
        if let Some(channels) = connection.channels() {
            while let Some((crc32, index, map_bin)) = channels.recv::<(u32, u64, [u8; 32])>() {
                // Once again, the if let Some is just to make it harder for malicious clients to crash the game
                // Firstly, check if the map actually exists

                if let Some(map) = maps.0.get_mut(&crc32) {
                    let index: usize = index.try_into().unwrap();

                    // If it does, check that the index is not out of bounds (which would result in a crash)
                    if let Some(map_object) = map.objects.get_mut(index) {
                        // Finally, make sure that a map object isn't being overwritten
                        if map_object != &MAP_OBJECT_DEFAULT {
                            let new_map_object = MapObject::from_bin(&map_bin); 
                            *map_object = new_map_object;


                        }                    
                    }
                }
            }
        }
    }

}

// To save an extra channel, map metadata sendinng and requesting uses the exact same structure
// Sending of course just sends the metadata, and requesting is just a null map metadata send but with the crc32 being not null
pub fn handle_map_metadata(mut net: ResMut<NetworkResource>, mut maps: ResMut<Maps>) {
    const MAP_OBJECT_DEFAULT: MapObject = MapObject::default();

    let mut messages_to_send = Vec::new();

    for (handle, connection) in net.connections.iter_mut() {
        if let Some(channels) = connection.channels() {
            while let Some((name, map_objects_len, background_color, map_size, crc32)) = channels.recv::<(String, u64, [f32; 3], [f32; 2], u32)>() {
                println!("Metadata");

                // If true, this is a map metadata request
                if map_objects_len == 0 {
                    if let Some(map) = maps.0.get(&crc32) {
                        let bg_c: [f32; 3] = map.background_color.as_rgba_f32()[0..3].try_into().unwrap();
                        let size: [f32; 2] = map.size.into();
                        let len: u64 = map.objects.len().try_into().unwrap();

                        messages_to_send.push((*handle, (map.name.clone(), len, bg_c, size, map.crc32)));

                    }


                // This is a map metadata send
                } else {
                    // Doing an if let Some so the game doesn't just crash if someone sends a non existent crc32
                    if let Some(map) = maps.0.get_mut(&crc32) {
                        // To make sure malicious net clients can't just override our metadata, it will only replace ours if the map_objects len is 0
                        if !map.objects.is_empty() {
                            let map_objects_len: usize = map_objects_len.try_into().unwrap();

                            map.objects = vec![MAP_OBJECT_DEFAULT; map_objects_len];
                            map.name = name;
                            map.background_color = Color::rgb(background_color[0], background_color[1], background_color[2]);
                            map.size = map_size.into();

                        }
                    }

                }

            }
        }


    }

    messages_to_send.into_iter().for_each(|(handle, message)| {net.send_message(handle, message).unwrap();} );

}


pub fn handle_debug_text(mut net: ResMut<NetworkResource>) {
    for (_handle, connection) in net.connections.iter_mut() {
        if let Some(channels) = connection.channels() {
            while let Some(text) = channels.recv::<String>() {
                println!("{}", text);

            }
        }
    }
}

pub fn handle_text_messages(mut net: ResMut<NetworkResource>, mut log_event: EventWriter<ChatEvent>, names: Query<&PlayerName>, player_entity: Res<HashMap<u8, Entity>>) {
    #[cfg(feature = "native")]
    let mut messages_to_send: Vec<TextMessage> = Vec::new();

    for (_handle, connection) in net.connections.iter_mut() {
        if let Some(channels) = connection.channels() {
            while let Some((player_id, message, time)) = channels.recv::<TextMessage>() {
                let player_name = names.get(*player_entity.get(&player_id).unwrap()).unwrap();

                #[cfg(feature = "native")]
                messages_to_send.push((player_id, message.clone(), time));

                log_event.send(ChatEvent(format!("{}: {}", player_name, message)));

            }
        }
    }

    #[cfg(feature = "native")]
    messages_to_send.into_iter().for_each(|m| {
        net.broadcast_message(m);

    });
}

// This function makes players that aren't online, online, if they aren't already
// Basically, the function just checks if the player is in online_player_ids, and if not, it inserts them into that and deathmatch score
// This function should be run on pretty much any net function that receives an ID
#[inline]
pub fn make_player_online(deathmatch_score: &mut HashMap<u8, u8>, online_player_ids: &mut HashMap<u8, Option<(u32, Timer)>>, player_id: u8, handle: &u32) {
    if !deathmatch_score.contains_key(&player_id) {
        deathmatch_score.insert(player_id, 0);

    }

    online_player_ids.insert(player_id, Some((*handle, Timer::from_seconds(15.0, false))));

}

// Literally just removes all connections from the connections HashMap.
#[inline(always)]
pub fn disconnect(mut net: ResMut<NetworkResource>) {
    net.connections.clear();

}
