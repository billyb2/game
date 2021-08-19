#![deny(clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::collections::BTreeSet;
use std::convert::TryInto;

use crate::*;
use single_byte_hashmap::HashMap;
use crate::components::{AbilityEvent, RequestedMovement};

#[cfg(feature = "native")]
use crate::LogEvent;

#[cfg(feature = "web")]
use crate:: {
    log,
    setup_systems::set_player_colors
};

#[cfg(feature = "native")]
use crate::helper_functions::get_available_port;

use crate::helper_functions::vec_to_array;

use bevy_networking_turbulence::*;
use bevy::prelude::*;
use bevy::utils::Duration;

use lazy_static::lazy_static;

lazy_static! {
    static ref SERVER_ADDRESS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9363);
}

// Location data is unreliable, since its okay if we skip a few frame updates
const CLIENT_STATE_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 0,
    channel_mode: MessageChannelMode::Unreliable,
    // The message buffer size is kind of overkill, but it lets the game lag and not process a good amount of messages for a few seconds and still not be overwhelmed
    message_buffer_size: 1024,
    packet_buffer_size: 1024,
};

// Projectile updates are reliable, since when someone shoots a bullet, the server *must* shoot
const PROJECTILE_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 1,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 8192,
            recv_window_size: 4096,
            send_window_size: 4096,
            burst_bandwidth: 4096,
            init_send: 1024,
            wakeup_time: Duration::from_millis(15),
            initial_rtt: Duration::from_millis(160),
            // Bullet shots won't register if ping is above 10 seconds
            max_rtt: Duration::from_secs(10),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 128,
    },
    message_buffer_size: 2048,
    packet_buffer_size: 2048,
};

// Some abilities, such as the wall and hacker, need to send a message over the network, so this does that here
const ABILITY_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 2,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 256,
            recv_window_size: 2048,
            send_window_size: 2048,
            burst_bandwidth: 2048,
            init_send: 1024,
            wakeup_time: Duration::from_millis(15),
            initial_rtt: Duration::from_millis(160),
            max_rtt: Duration::from_secs(4),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 128,
    },
    message_buffer_size: 128,
    packet_buffer_size: 128,
};

// When requesting or sending meta data about the game, such as the assigned player ids or abilities, it's fine to have up to a 10 second delay before getting a response
const INFO_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 3,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 8,
            recv_window_size: 2048,
            send_window_size: 2048,
            burst_bandwidth: 2048,
            init_send: 1024,
            wakeup_time: Duration::from_millis(50),
            initial_rtt: Duration::from_millis(200),
            // Info requests time out after 10 seconds
            max_rtt: Duration::from_secs(10),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 128,
    },
    message_buffer_size: 16,
    packet_buffer_size: 16,
};

// Damage is also reliable, with even more leeway since damage not registering is bad
const DAMAGE_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 4,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 256,
            recv_window_size: 2048,
            send_window_size: 2048,
            burst_bandwidth: 2048,
            init_send: 1024,
            wakeup_time: Duration::from_millis(15),
            initial_rtt: Duration::from_millis(160),
            // Damage won't register if ping is above 10 seconds
            max_rtt: Duration::from_secs(10),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 128,
    },
    message_buffer_size: 512,
    packet_buffer_size: 512,
};

const SET_MAP_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 5,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 8,
            recv_window_size: 2048,
            send_window_size: 2048,
            burst_bandwidth: 2048,
            init_send: 1024,
            wakeup_time: Duration::from_millis(50),
            initial_rtt: Duration::from_millis(200),
            // Info requests time out after 10 seconds
            max_rtt: Duration::from_secs(10),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 128,
    },
    message_buffer_size: 8,
    packet_buffer_size: 8,
};

const REQUEST_MAP_OBJECT_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 6,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 1024,
            recv_window_size: 2048,
            send_window_size: 2048,
            burst_bandwidth: 2048,
            init_send: 1024,
            wakeup_time: Duration::from_millis(50),
            initial_rtt: Duration::from_millis(200),
            // Info requests time out after 10 seconds
            max_rtt: Duration::from_secs(10),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 1024,
    },
    message_buffer_size: 4096,
    packet_buffer_size: 4096,
};

const SEND_MAP_OBJECT_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 7,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 1024,
            recv_window_size: 2048,
            send_window_size: 2048,
            burst_bandwidth: 2048,
            init_send: 1024,
            wakeup_time: Duration::from_millis(50),
            initial_rtt: Duration::from_millis(200),
            // Info requests time out after 10 seconds
            max_rtt: Duration::from_secs(10),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 1024,
    },
    message_buffer_size: 8192,
    packet_buffer_size: 8192,
};

const MAP_METADATA_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 8,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 1024,
            recv_window_size: 2048,
            send_window_size: 2048,
            burst_bandwidth: 2048,
            init_send: 1024,
            wakeup_time: Duration::from_millis(50),
            initial_rtt: Duration::from_millis(200),
            // Info requests time out after 10 seconds
            max_rtt: Duration::from_secs(10),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 1024,
    },
    message_buffer_size: 2048,
    packet_buffer_size: 2048,
};


#[cfg(feature = "web")]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// A timer of around 15 miliseconds, thatshould be sent (instead of flooding)
pub struct ReadyToSendPacket(pub Timer);

// A resource stating whether or not the player is hosting
pub struct Hosting(pub bool);

pub struct SetAbility(bool);

#[cfg(feature = "native")]
pub fn setup_listening(mut net: ResMut<NetworkResource>, hosting: Res<Hosting>) {
    // Currently, only PC builds can host
    #[cfg(feature = "native")]
    if hosting.0 {
        //let ip_address = bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");

        // let socket_address = SocketAddr::new(ip_address, SERVER_PORT);
        println!("Listening on {:?}", *SERVER_ADDRESS);

        // The WebRTC listening address just picks a random port
        let webrtc_listen_addr = {
            let webrtc_listen_ip: IpAddr = SERVER_ADDRESS.ip();

            let webrtc_listen_port = get_available_port(webrtc_listen_ip.to_string().as_str()).expect("No available port");

            SocketAddr::new(webrtc_listen_ip, webrtc_listen_port)
        };

        net.listen(*SERVER_ADDRESS, Some(webrtc_listen_addr), Some(webrtc_listen_addr));

    }
}

pub fn setup_networking(mut commands: Commands, mut net: ResMut<NetworkResource>, _hosting: Res<Hosting>, mut _app_state: ResMut<State<AppState>>) {
    // Registers message types
    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            .register::<(u8, [f32; 2], [f32; 4])>(CLIENT_STATE_MESSAGE_SETTINGS)
            .unwrap();

        builder
            .register::<ShootEvent>(PROJECTILE_MESSAGE_SETTINGS)
            .unwrap();

        builder
            .register::<([u8; 2], f32)>(DAMAGE_MESSAGE_SETTINGS)
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
            // Using a u64 instead of a usize since I'm pretty sure WASM is 32 bit
            // First item of the tuple is the crc32 of the map object, the second item is the index of the map object in the vector
            .register::<(u32, u64)>(REQUEST_MAP_OBJECT_SETTINGS)
            .unwrap();

        builder
            // Index 0 is the crc32, index 1 is the index of the map object, and index 2 is the map object as binary
            .register::<(u32, u64, [u8; 32])>(SEND_MAP_OBJECT_SETTINGS)
            .unwrap();

        builder
            // Index 0 is the map name,, index 1 is the length of the map objects vector, index 2 is the background color, 3 is the map size, 4 is the crc32
            .register::<(String, u64, [f32; 3], [f32; 2], u32)>(MAP_METADATA_SETTINGS)
            .unwrap();



    });

    commands.insert_resource(ReadyToSendPacket(Timer::new(Duration::from_millis(15), false)));
    commands.insert_resource(SetAbility(false));

    #[cfg(feature = "native")]
     _app_state.set(AppState::InGame).unwrap();

    // Currently, only web builds can join games (until we add UDP servers or something)
    #[cfg(feature = "web")]
    if !_hosting.0 {
        println!("Connecting to {:?}", *SERVER_ADDRESS);

        #[cfg(feature = "web")]
        console_log!("Net: Connecting to {:?}", *SERVER_ADDRESS);

        net.connect(*SERVER_ADDRESS);

    }
}

pub fn send_stats(mut net: ResMut<NetworkResource>, players: Query<&Transform>, mut ready_to_send_packet: ResMut<ReadyToSendPacket>, my_player_id: Res<MyPlayerID>, player_entity: Res<HashMap<u8, Entity>>) {
    // Only start sending packets when your ID is set
    if let Some(my_id) = &my_player_id.0 {
        // Rate limiting so that the game sends 66 updates every second
        if ready_to_send_packet.0.finished() {
            let transform = players.get(*player_entity.get(&my_id.0).unwrap()).unwrap();
            let quat_xyzw: [f32; 4] = transform.rotation.into();

            net.broadcast_message((my_id.0, [transform.translation.x, transform.translation.y], quat_xyzw));

            ready_to_send_packet.0.reset();
        }
    }
}

pub fn handle_stat_packets(mut net: ResMut<NetworkResource>, mut players: Query<&mut Transform>, my_player_id: Res<MyPlayerID>, _hosting: Res<Hosting>, mut online_player_ids: ResMut<OnlinePlayerIDs>, mut deathmatch_score: ResMut<DeathmatchScore>, player_entity: Res<HashMap<u8, Entity>>) {
    #[cfg(feature = "native")]
    let mut messages_to_send: Vec<(u8, [f32; 2], [f32; 4])> = Vec::with_capacity(255);

    if let Some(my_id) = &my_player_id.0 {
        for (_handle, connection) in net.connections.iter_mut() {
            if let Some(channels) = connection.channels() {
                while let Some((player_id, [x, y], [rot_x, rot_y, rot_z, rot_w])) = channels.recv::<(u8, [f32; 2], [f32; 4])>() {
                    // The host broadcasts the locations of all other players
                    #[cfg(feature = "native")]
                    if _hosting.0 {
                        messages_to_send.push((player_id, [x, y], [rot_x, rot_y, rot_z, rot_w]))

                    }

                    make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, player_id);

                    // Set the location of any local players to the location given
                    let mut transform = players.get_mut(*player_entity.get(&player_id).unwrap()).unwrap();

                    transform.rotation = Quat::from_xyzw(rot_x, rot_y, rot_z, rot_w);

                    if player_id != my_id.0 {
                        transform.translation.x = x;
                        transform.translation.y = y;

                    }

                }
            }
        }

    }

    // Broadcast the location of all players to everyone
    #[cfg(feature = "native")]
    for m in messages_to_send.iter() {
        net.broadcast_message(*m);

    }
}

pub fn handle_ability_packets(mut net: ResMut<NetworkResource>, mut players: Query<(&mut AmmoInMag, &mut Transform, &mut RequestedMovement, &mut Ability)>, my_player_id: Res<MyPlayerID>, _hosting: Res<Hosting>,  mut ev_use_ability: EventWriter<AbilityEvent>, mut online_player_ids: ResMut<OnlinePlayerIDs>, mut deathmatch_score: ResMut<DeathmatchScore>, player_entity: Res<HashMap<u8, Entity>>) {
    #[cfg(feature = "native")]
    let mut messages_to_send: Vec<([u8; 2], [f32; 3])> = Vec::with_capacity(255);

    if let Some(my_id) = &my_player_id.0 {
        for (_handle, connection) in net.connections.iter_mut() {
            if let Some(channels) = connection.channels() {
                while let Some(([player_id, ability], [player_x, player_y, angle])) = channels.recv::<([u8; 2], [f32; 3])>() {
                    if player_id != my_id.0 {
                        // The host broadcasts the locations of all other players
                        #[cfg(feature = "native")]
                        if _hosting.0 {
                            messages_to_send.push(([player_id, ability], [player_x, player_y, angle]))

                        }

                    }

                    let ability: Ability = ability.into();

                    if player_id != my_id.0 || ability == Ability::Hacker {
                        make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, player_id);
                        let (mut ammo_in_mag, mut transform, mut requested_movement, mut old_ability) = players.get_mut(*player_entity.get(&player_id).unwrap()).unwrap();

                        *old_ability = ability;

                        if ability != Ability::Hacker {
                            transform.translation.x = player_x;
                            transform.translation.y = player_y;

                            requested_movement.angle = angle;

                            #[cfg(feature = "web")]
                            console_log!("{:?}", old_ability);

                            if ability == Ability::Wall || ability == Ability::Cloak || ability == Ability::Ghost {
                                ev_use_ability.send(AbilityEvent(player_id));

                            }

                        } else {
                            // The hacker forces a player to use their ability and halves their ammo count
                            ev_use_ability.send(AbilityEvent(my_id.0));
                            ammo_in_mag.0 = (ammo_in_mag.0 as f32 / 2.0).ceil() as u8;

                        }
                    }
                }
            }
        }
    }

    // Broadcast the location of all players to everyone
    #[cfg(feature = "native")]
    for m in messages_to_send.iter() {
        net.broadcast_message(*m);

    }
}

pub fn handle_projectile_packets(mut net: ResMut<NetworkResource>, mut shoot_event: EventWriter<ShootEvent>, mut players: Query<&mut Transform>, _hosting: Res<Hosting>, my_player_id: Res<MyPlayerID>, mut online_player_ids: ResMut<OnlinePlayerIDs>, mut deathmatch_score: ResMut<DeathmatchScore>, player_entity: Res<HashMap<u8, Entity>>) {
    #[cfg(feature = "native")]
    let mut messages_to_send: Vec<ShootEvent> = Vec::with_capacity(255);

    if let Some(my_id) = &my_player_id.0 {
        for (_handle, connection) in net.connections.iter_mut() {
            if let Some(channels) = connection.channels() {
                while let Some(event) = channels.recv::<ShootEvent>() {
                    if my_id.0 !=  event.player_id {
                        make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, event.player_id);

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
            for m in messages_to_send.iter() {
                net.broadcast_message((*m).clone());

            }
        }
    }
}

pub fn handle_damage_packets(mut net: ResMut<NetworkResource>, mut players: Query<&mut Health>, _hosting: Res<Hosting>, my_player_id: Res<MyPlayerID>, mut deathmatch_score: ResMut<DeathmatchScore>, mut death_event: EventWriter<DeathEvent>, mut online_player_ids: ResMut<OnlinePlayerIDs>, player_entity: Res<HashMap<u8, Entity>>) {
    #[cfg(feature = "native")]
    let mut messages_to_send: Vec<([u8; 2], f32)> = Vec::with_capacity(255);

    if let Some(my_id) = &my_player_id.0 {
        for (_handle, connection) in net.connections.iter_mut() {
            if let Some(channels) = connection.channels() {
                while let Some(([player_who_took_damage, player_who_fired_shot], damage)) = channels.recv::<([u8; 2], f32)>() {
                    // This isn't allowed to happen, since if if you receive a message saying that your player took damage, you sent said message
                    if my_id.0 != player_who_took_damage {
                        make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, player_who_took_damage);
                        make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, player_who_fired_shot);

                        let mut health = players.get_mut(*player_entity.get(&player_who_took_damage).unwrap()).unwrap();

                        if (health.0 - damage) <= 0.0 && health.0 != 0.0 {
                            health.0 = 0.0;
                            death_event.send(DeathEvent(player_who_took_damage));
                            // The player who shot the bullet has their score increased 
                            *deathmatch_score.0.get_mut(&player_who_fired_shot).unwrap() += 1;


                        } else {
                            health.0 -= damage;

                        }

                    }

                    // The host broadcasts the shots fired of all other players
                    #[cfg(feature = "native")]
                    if _hosting.0 {
                        messages_to_send.push(([player_who_took_damage, player_who_fired_shot], damage));

                    }

                }
            }
        }

        #[cfg(feature = "native")]
        if _hosting.0 {
            for m in messages_to_send.iter() {
                net.broadcast_message(*m);

            }
        }
    }
}


#[cfg(feature = "web")]
pub fn request_player_info(hosting: Res<Hosting>, my_player_id: Res<MyPlayerID>, my_ability: Res<Ability>, mut net: ResMut<NetworkResource>, mut ready_to_send_packet: ResMut<ReadyToSendPacket>, ability_set: Res<SetAbility>, mut app_state: ResMut<State<AppState>>) {
    // Every few seconds, the client requests an ID from the host server until it gets one
    console_log!("Net: Ability set: {}", my_player_id.0.is_some());

    if !hosting.0 && my_player_id.0.is_none() && ready_to_send_packet.0.finished() && !ability_set.0 {
        let request_id_message: [u8; 3] = [0; 3];
        net.broadcast_message(request_id_message);

        ready_to_send_packet.0.set_duration(Duration::from_secs(11));
        ready_to_send_packet.0.reset();

    } else if my_player_id.0.is_some() && !ability_set.0 {
        let set_ability_message: [u8; 3] = [1, (*my_ability).into(), my_player_id.0.as_ref().unwrap().0];
        net.broadcast_message(set_ability_message);

        ready_to_send_packet.0.reset();

    } else if my_player_id.0.is_some() && ability_set.0 {
        // Once the client gets an ID and an ability, it starts sending location data every 15 miliseconds
        ready_to_send_packet.0.set_duration(Duration::from_millis(15));
        app_state.set(AppState::InGame).unwrap();
  
    }
}

#[cfg(feature = "native")]
pub fn handle_server_commands(mut net: ResMut<NetworkResource>, mut available_ids: ResMut<Vec<PlayerID>>, hosting: Res<Hosting>, mut players: Query<(&PlayerID, &mut Ability, &mut HelmetColor, &mut InnerSuitColor)>, mut online_player_ids: ResMut<OnlinePlayerIDs>, mut log_event: EventWriter<LogEvent>, mut deathmatch_score: ResMut<DeathmatchScore>, map_crc32: Res<MapCRC32>) {
    if hosting.0 {
        // First item is the handle, the second is the ID
        let mut messages_to_send: Vec<(u32, [u8; 3])> = Vec::with_capacity(255);

        for (handle, connection) in net.connections.iter_mut() {
            let channels = connection.channels().unwrap();

            while let Some(command) = channels.recv::<[u8; 3]>() {
                // Send a player ID as well as an ability back
                if command[0] == 0 {
                    if let Some(player_id) = available_ids.last() {
                        make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, player_id.0);

                        println!("Player {} has joined!", player_id.0 + 1);
                        log_event.send(LogEvent(format!("Player {} has joined!", player_id.0 + 1)));

                        // Sending back the player id
                        messages_to_send.push((*handle, [0, player_id.0, 0]));

                        available_ids.pop();

                    } else {
                        println!("Lobby full");

                    }
                // Respond to the player's ability request
                } else if command[0] == 1 {
                    let player_id = command[2];
                    let player_ability: Ability = command[1].into();

                    for (id, mut ability, mut helmet_color, mut inner_suit_color) in players.iter_mut() {
                        if id.0 == player_id {
                            *ability.deref_mut() = player_ability;

                            let (new_helmet_color, new_inner_suit_color) = set_player_colors(&player_ability);

                            *helmet_color.deref_mut() = new_helmet_color;
                            *inner_suit_color.deref_mut() = new_inner_suit_color;

                        }

                        // Send the abilities of all players
                        messages_to_send.push((*handle, [1, (*ability).into(), player_id]));
                    }
                }
            }
        }

        messages_to_send.shrink_to_fit();

        for (handle, message) in messages_to_send.into_iter() {
            net.send_message(handle, message).unwrap();
            net.send_message(handle, map_crc32.0).unwrap();

        }

    }
}

#[cfg(feature = "web")]
pub fn handle_client_commands(mut net: ResMut<NetworkResource>, hosting: Res<Hosting>, mut my_player_id: ResMut<MyPlayerID>, mut players: Query<(&PlayerID, &mut Ability, &mut HelmetColor, &mut InnerSuitColor, &mut Handle<ColorMaterial>)>, mut ability_set: ResMut<SetAbility>, mut online_player_ids: ResMut<OnlinePlayerIDs>, mut deathmatch_score: ResMut<DeathmatchScore>, mut map_crc32: ResMut<MapCRC32>, player_entity: Res<HashMap<u8, Entity>>, materials: Res<Skin>, mut maps: ResMut<Maps>, mut app_state: ResMut<State<AppState>>) {
    if !hosting.0 {
        for (_handle, connection) in net.connections.iter_mut() {
            let channels = connection.channels().unwrap();

            while let Some(command) = channels.recv::<[u8; 3]>() {
                // The set player ID command
                if command[0] == 0 {
                    let id = command[1];

                    my_player_id.0 = Some(PlayerID(id));
                    make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, id);

                    /*let (_id, mut _ability, mut _helmet_color, mut _inner_suit_color, mut sprite) = players.get_mut(*player_entity.get(&id).unwrap()).unwrap();

                    *sprite.deref_mut() = materials.player.clone();*/


                // The set player ability command
                } else if command[0] == 1 {
                    let [_, ability, player_id] = command;
                    let player_ability: Ability = ability.into();

                    make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, command[2]);

                    players.for_each_mut(|(id, mut ability, mut helmet_color, mut inner_suit_color, _sprite)| {
                        if id.0 == player_id {
                            *ability = player_ability;

                            let (new_helmet_color, new_inner_suit_color) = set_player_colors(&player_ability);

                            *helmet_color = new_helmet_color;
                            *inner_suit_color = new_inner_suit_color;

                            if let Some(my_player_id) = &my_player_id.0 {
                                if id.0 == my_player_id.0 {
                                    ability_set.0 = true;

                                }
                            }

                        }

                    });

                }
            }

            while let Some(new_crc32) = channels.recv::<u32>() {

                if new_crc32 != map_crc32.0 {
                    // If the map doensn't currently exist yet, start downloading a new one
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
                }

                map_crc32.0 = new_crc32;

            }
        }
    }
}

pub fn handle_map_object_request(mut net: ResMut<NetworkResource>, maps: Res<Maps>) {
    let mut messages_to_send = Vec::with_capacity(50);

    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();

        while let Some((crc32, index)) = channels.recv::<(u32, u64)>() {
            // Normally, I just unwrap when getting a map object, but in this case, sending a fake CRC32 would be a really easy way to crash any host or client, so I'm just preparing for that
            //TODO: Add sending network error messages for bad requests
            if let Some(map) = maps.0.get(&crc32) {
                let index_usize: usize = index.try_into().unwrap();
                let map_object = &map.objects[index_usize];
                // Basically just makes sure that the map object is a u8 array
                let bin = vec_to_array::<u8, 32>( map_object.to_bin() );

                messages_to_send.push((*handle, (crc32, index, bin)));
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

    let mut messages_to_send = Vec::with_capacity(50);

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
                        if map.objects.len() != 0 {
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

// This function makes players that aren't online, online, if they aren't already
// Basically, the function just checks if the player is in online_player_ids, and if not, it inserts them into that and deathmatch score
// This function should be run on pretty much any net function that receives an ID
#[inline]
pub fn make_player_online(deathmatch_score: &mut HashMap<u8, u8>, online_player_ids: &mut BTreeSet<u8>, player_id: u8) {
    if !deathmatch_score.contains_key(&player_id) {
        deathmatch_score.insert(player_id, 0);

    }

    online_player_ids.insert(player_id);

}

// Literally just removes all connections from the connections HashMap.
#[inline(always)]
pub fn disconnect(mut net: ResMut<NetworkResource>) {
    net.connections.clear();

}
