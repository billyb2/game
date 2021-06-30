#![deny(clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::collections::BTreeSet;

use crate::*;
use single_byte_hashmap::HashMap;
use crate::components::{AbilityEvent, RequestedMovement};

#[cfg(feature = "native")]
use crate::LogEvent;

#[cfg(feature = "web")]
use crate::log;
#[cfg(feature = "web")]
use crate::setup_systems::set_player_colors;

#[cfg(feature = "native")]
use crate::helper_functions::get_available_port;

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
            bandwidth: 256,
            recv_window_size: 2048,
            send_window_size: 2048,
            burst_bandwidth: 2048,
            init_send: 1024,
            wakeup_time: Duration::from_millis(15),
            initial_rtt: Duration::from_millis(160),
            // Bullet shots won't register if ping is above 4 seconds
            max_rtt: Duration::from_secs(4),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 128,
    },
    message_buffer_size: 64,
    packet_buffer_size: 64,
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
    message_buffer_size: 64,
    packet_buffer_size: 64,
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
    message_buffer_size: 8,
    packet_buffer_size: 8,
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
    message_buffer_size: 64,
    packet_buffer_size: 64,
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
            .register::<(u8, bool, [f32; 2])>(CLIENT_STATE_MESSAGE_SETTINGS)
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

    });

    commands.insert_resource(ReadyToSendPacket(Timer::new(Duration::from_millis(15), false)));
    commands.insert_resource(SetAbility(false));

    #[cfg(feature = "native")]
     _app_state.set(AppState::InGame).unwrap();

    // Currently, only web builds can join games (until we add UDP servers)
    #[cfg(feature = "web")]
    if !_hosting.0 {
        println!("Connecting to {:?}", *SERVER_ADDRESS);
        console_log!("Net: Connecting to {:?}", *SERVER_ADDRESS);

        net.connect(*SERVER_ADDRESS);

    }
}

pub fn send_stats(mut net: ResMut<NetworkResource>, players: Query<(&Transform, &Sprite)>, mut ready_to_send_packet: ResMut<ReadyToSendPacket>, my_player_id: Res<MyPlayerID>, player_entity: Res<HashMap<u8, Entity>>) {
    // Only start sending packets when your ID is set
    if let Some(my_id) = &my_player_id.0 {
        // Rate limiting so that the game sends 66 updates every second
        if ready_to_send_packet.0.finished() {
            let (transform, sprite) = players.get(*player_entity.get(&my_id.0).unwrap()).unwrap();
            net.broadcast_message((my_id.0, sprite.flip_x, [transform.translation.x, transform.translation.y]));

            ready_to_send_packet.0.reset();
        }
    }
}

pub fn handle_stat_packets(mut net: ResMut<NetworkResource>, mut players: Query<(&mut Transform, &mut Sprite)>, my_player_id: Res<MyPlayerID>, _hosting: Res<Hosting>, mut online_player_ids: ResMut<OnlinePlayerIDs>, mut deathmatch_score: ResMut<DeathmatchScore>, player_entity: Res<HashMap<u8, Entity>>) {
    #[cfg(feature = "native")]
    let mut messages_to_send: Vec<(u8, bool, [f32; 2])> = Vec::with_capacity(255);

    if let Some(my_id) = &my_player_id.0 {
        for (_handle, connection) in net.connections.iter_mut() {
            let channels = connection.channels().unwrap();

            while let Some((player_id, flip_x, [x, y])) = channels.recv::<(u8, bool, [f32; 2])>() {
                // The host broadcasts the locations of all other players
                #[cfg(feature = "native")]
                if _hosting.0 {
                    messages_to_send.push((player_id, flip_x, [x, y]))

                }

                make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, player_id);

                // Set the location of any local players to the location given
                let (mut transform, mut sprite) = players.get_mut(*player_entity.get(&player_id).unwrap()).unwrap();

                sprite.flip_x = flip_x;

                if player_id != my_id.0 {
                    transform.translation.x = x;
                    transform.translation.y = y;

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

pub fn handle_ability_packets(mut net: ResMut<NetworkResource>, mut players: Query<(&mut AmmoInMag, &mut Transform, &mut RequestedMovement)>, my_player_id: Res<MyPlayerID>, _hosting: Res<Hosting>,  mut ev_use_ability: EventWriter<AbilityEvent>, mut online_player_ids: ResMut<OnlinePlayerIDs>, mut deathmatch_score: ResMut<DeathmatchScore>, player_entity: Res<HashMap<u8, Entity>>) {
    #[cfg(feature = "native")]
    let mut messages_to_send: Vec<([u8; 2], [f32; 3])> = Vec::with_capacity(255);

    if let Some(my_id) = &my_player_id.0 {
        for (_handle, connection) in net.connections.iter_mut() {
            let channels = connection.channels().unwrap();

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
                    let (mut ammo_in_mag, mut transform, mut requested_movement) = players.get_mut(*player_entity.get(&player_id).unwrap()).unwrap();

                    if ability != Ability::Hacker {
                        transform.translation.x = player_x;
                        transform.translation.y = player_y;

                        requested_movement.angle = angle;

                        if ability == Ability::Wall || ability == Ability::Cloak {
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
            let channels = connection.channels().unwrap();

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
            let channels = connection.channels().unwrap();

            while let Some(([player_who_took_damage, player_who_fired_shot], damage)) = channels.recv::<([u8; 2], f32)>() {
                // This isn't allowed to happen, since if if you receive a message saying that your player took damage, you sent said message
                if my_id.0 != player_who_took_damage {
                    make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, player_who_took_damage);
                    make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, player_who_fired_shot);

                    let mut health = players.get_mut(*player_entity.get(&player_who_took_damage).unwrap()).unwrap();

                    if (health.0 - damage) <= 0.0 {
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
pub fn handle_server_commands(mut net: ResMut<NetworkResource>, mut available_ids: ResMut<Vec<PlayerID>>, hosting: Res<Hosting>, mut players: Query<(&PlayerID, &mut Ability, &mut HelmetColor, &mut InnerSuitColor)>, mut online_player_ids: ResMut<OnlinePlayerIDs>, mut log_event: EventWriter<LogEvent>, mut deathmatch_score: ResMut<DeathmatchScore>) {
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

        for (handle, message) in messages_to_send.iter() {
            net.send_message(*handle, *message).unwrap();

        }
    }
}

#[cfg(feature = "web")]
pub fn handle_client_commands(mut net: ResMut<NetworkResource>, hosting: Res<Hosting>, mut my_player_id: ResMut<MyPlayerID>, mut players: Query<(&PlayerID, &mut Ability, &mut HelmetColor, &mut InnerSuitColor)>, mut ability_set: ResMut<SetAbility>, mut online_player_ids: ResMut<OnlinePlayerIDs>, mut deathmatch_score: ResMut<DeathmatchScore>) {
    if !hosting.0 {
        for (_handle, connection) in net.connections.iter_mut() {
            let channels = connection.channels().unwrap();

            while let Some(command) = channels.recv::<[u8; 3]>() {
                // The set player ID command
                if command[0] == 0 {
                    let id = command[1];

                    my_player_id.0 = Some(PlayerID(id));
                    make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, id);

                    break;

                // The set player ability command
                } else if command[0] == 1 {
                    let player_ability: Ability = command[1].into();

                    make_player_online(&mut deathmatch_score.0, &mut online_player_ids.0, command[2]);

                    players.for_each_mut(|(id, mut ability, mut helmet_color, mut inner_suit_color)| {
                        if id.0 == command[2] {
                            *ability.deref_mut() = player_ability;

                            let (new_helmet_color, new_inner_suit_color) = set_player_colors(&player_ability);

                            *helmet_color.deref_mut() = new_helmet_color;
                            *inner_suit_color.deref_mut() = new_inner_suit_color;

                            if let Some(my_player_id) = &my_player_id.0 {
                                if id.0 == my_player_id.0 {
                                    ability_set.0 = true;

                                }
                            }

                        }

                    });

                }
            }
        }
    }
}

// This function makes players that aren't online, online, if they aren't already
// Basically, the function just checks if the player is in the online_player_ids BTreeSet, and if not, it inserts them into that and deathmatch score
// This function should be run on pretty much any net function that receives an ID
pub fn make_player_online(deathmatch_score: &mut HashMap<u8, u8>, online_player_ids: &mut BTreeSet<u8>, player_id: u8) {
    if !online_player_ids.contains(&player_id) {
        deathmatch_score.insert(player_id, 0);
        online_player_ids.insert(player_id);

    }
}

// Literally just removes all connections from the connections HashMap.
pub fn disconnect(mut net: ResMut<NetworkResource>) {
    net.connections.clear();

}
