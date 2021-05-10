use std::net::{IpAddr, Ipv4Addr, SocketAddr};
#[cfg(feature = "web")]
use std::ops::DerefMut;

use crate::{Ability, MyPlayerID, PlayerID, ShootEvent};

#[cfg(feature = "native")]
use crate::LogEvent;

#[cfg(feature = "web")]
use crate::{Skins, log};

#[cfg(feature = "native")]
use crate::helper_functions::get_available_port;

use bevy_networking_turbulence::*;
use bevy::prelude::*;
use bevy::utils::Duration;

use serde::{Deserialize, Serialize};

const SERVER_PORT: u16 = 9363;

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

// When requesting or sending meta data about the game, such as the assigned player ids or abilities, it's fine to have up to a 10 second delay before getting a response
const INFO_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 2,
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

#[cfg(feature = "web")]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// A timer of around 15 miliseconds, thatshould be sent (instead of flooding)
pub struct ReadyToSendPacket(pub Timer);

// A resource stating whether or not the player is hosting
pub struct Hosting(pub bool);

pub struct SetAbility(bool);

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
enum GameCommand {
    RequestID,

}

pub fn setup_networking(mut commands: Commands, mut net: ResMut<NetworkResource>, hosting: Res<Hosting>) {
    // Registers message types
    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            .register::<(u8, [f32; 2])>(CLIENT_STATE_MESSAGE_SETTINGS)
            .unwrap();

        builder
            .register::<ShootEvent>(PROJECTILE_MESSAGE_SETTINGS)
            .unwrap();

        builder
            .register::<[u8; 2]>(INFO_MESSAGE_SETTINGS)
            .unwrap();

    });

    commands.insert_resource(ReadyToSendPacket(Timer::new(Duration::from_millis(15), false)));
    commands.insert_resource(SetAbility(false));

    let socket_address: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), SERVER_PORT);

    // Currently, only PC builds can host
    #[cfg(feature = "native")]
    if hosting.0 {
        //let ip_address = bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");

        // let socket_address = SocketAddr::new(ip_address, SERVER_PORT);
        println!("Listening on {:?}", &socket_address);

        // The WebRTC listening address just picks a random port
        let webrtc_listen_addr = {
            let webrtc_listen_ip: IpAddr = socket_address.ip();

            let webrtc_listen_port = get_available_port(webrtc_listen_ip.to_string().as_str())
                .expect("No available port");

            SocketAddr::new(webrtc_listen_ip, webrtc_listen_port)
        };

        net.listen(socket_address, Some(webrtc_listen_addr), Some(webrtc_listen_addr));

    }

    // Currently, only web builds can join games (until we add UDP servers)
    #[cfg(feature = "web")]
    if !hosting.0 {
        println!("Connecting to {:?}", socket_address);
        console_log!("Net: Connecting to {:?}", socket_address);

        net.connect(socket_address);

    }
}

pub fn send_location(mut net: ResMut<NetworkResource>, players: Query<(&Transform, &PlayerID)>, mut ready_to_send_packet: ResMut<ReadyToSendPacket>, my_player_id: Res<MyPlayerID>) {
    if let Some(my_id) = &my_player_id.0 {
        // Rate limiting so that the game sends 66 updates every second
        // Only start sending packets when your ID is set
        if ready_to_send_packet.0.finished() {
            for (transform, id) in players.iter() {
                if id.0 == my_id.0 {
                    net.broadcast_message((my_id.0, [transform.translation.x, transform.translation.y]));

                    break;

                }

            }

            ready_to_send_packet.0.reset();
        }
    }
}

pub fn handle_movement_packets(mut net: ResMut<NetworkResource>, mut players: Query<(&mut Transform, &PlayerID)>, my_player_id: Res<MyPlayerID>, _hosting: Res<Hosting>) {
    #[cfg(feature = "native")]
    let mut messages_to_send: Vec<(u8, [f32; 2])> = Vec::with_capacity(255);

    if let Some(my_id) = &my_player_id.0 {
        for (_handle, connection) in net.connections.iter_mut() {
            let channels = connection.channels().unwrap();

            while let Some((player_id, [x, y])) = channels.recv::<(u8, [f32; 2])>() {
                if player_id != my_id.0 {
                    // The host broadcasts the locations of all other players
                    #[cfg(feature = "native")]
                    if _hosting.0 {
                        messages_to_send.push((player_id, [x, y]))

                    }

                    // Set the location of any local players to the location given
                    for (mut transform, id) in players.iter_mut() {
                        if id.0 == player_id {
                            transform.translation.x = x;
                            transform.translation.y = y;

                            break;

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

pub fn handle_projectile_packets(mut net: ResMut<NetworkResource>, mut shoot_event: EventWriter<ShootEvent>, mut players: Query<(&mut Transform, &PlayerID)>, _hosting: Res<Hosting>, my_player_id: Res<MyPlayerID>) {
    #[cfg(feature = "native")]
    let mut messages_to_send: Vec<ShootEvent> = Vec::with_capacity(255);

    if let Some(my_id) = &my_player_id.0 {
        for (_handle, connection) in net.connections.iter_mut() {
            let channels = connection.channels().unwrap();

            while let Some(event) = channels.recv::<ShootEvent>() {
                if my_id.0 !=  event.player_id {
                    for (mut transform, id) in players.iter_mut() {
                        if event.player_id == id.0 {
                            transform.translation = event.start_pos;
                            break;

                        }
                    }

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


#[cfg(feature = "web")]
pub fn request_player_info(hosting: Res<Hosting>, my_player_id: Res<MyPlayerID>, mut net: ResMut<NetworkResource>, mut ready_to_send_packet: ResMut<ReadyToSendPacket>, ability_set: Res<SetAbility>) {
    // Every 5 seconds, the client requests an ID from the host server until it gets one
    console_log!("Net: Ability set: {}", my_player_id.0.is_some());

    if !hosting.0 && my_player_id.0.is_none() && ready_to_send_packet.0.finished() && !ability_set.0 {
        console_log!("Net: Sending command");


        let request_id_message: [u8; 2] = [0; 2];
        net.broadcast_message(request_id_message);

        ready_to_send_packet.0.set_duration(Duration::from_secs(7));
        ready_to_send_packet.0.reset();

    } else if my_player_id.0.is_some() && ability_set.0 {
        // Once the client gets an ID and an ability, it starts sending location data every 15 miliseconds
        ready_to_send_packet.0.set_duration(Duration::from_millis(15));


    }
}

#[cfg(feature = "native")]
pub fn handle_server_commands(mut net: ResMut<NetworkResource>, mut available_ids: ResMut<Vec<PlayerID>>, hosting: Res<Hosting>, mut log_event: EventWriter<LogEvent>, players: Query<(&PlayerID, &Ability)>) {
    if hosting.0 {
        // First item is the handle, the second is the ID
        let mut messages_to_send: Vec<(u32, [u8; 2])> = Vec::with_capacity(255);

        for (handle, connection) in net.connections.iter_mut() {
            let channels = connection.channels().unwrap();

            while let Some(command) = channels.recv::<[u8; 2]>() {
                // Send a player ID as well as an ability back
                if command[0] == 0 {
                    if let Some(player_id) = available_ids.last() {
                        println!("Player {} has joined!", player_id.0 + 1);
                        log_event.send(LogEvent(format!("Player {} has joined!", player_id.0 + 1)));

                        // Sending back the player id
                        messages_to_send.push((*handle, [0, player_id.0]));

                        // Sending back the player ability
                        for (id, ability) in players.iter() {
                            if id.0 == player_id.0 {
                                println!("Sending ability");
                                messages_to_send.push((*handle, [1, (*ability).into()]));

                                break;

                            }
                        }

                        available_ids.pop();

                    } else {
                        println!("Lobby full");

                    }
                // Respond to the player's ability request
                } else if command[0] == 1 {
                    let player_id = command[1];

                    for (id, ability) in players.iter() {
                        if id.0 == player_id {
                            messages_to_send.push((*handle, [1, (*ability).into()]));

                        }
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
pub fn handle_client_commands(mut net: ResMut<NetworkResource>, hosting: Res<Hosting>, mut my_player_id: ResMut<MyPlayerID>, mut players: Query<(&PlayerID, &mut Ability, &mut Handle<ColorMaterial>)>, mut ability_set: ResMut<SetAbility>, materials: Res<Skins>) {
    if !hosting.0 {
        for (_handle, connection) in net.connections.iter_mut() {
            let channels = connection.channels().unwrap();

            while let Some(command) = channels.recv::<[u8; 2]>() {
                // The set player ID command
                if command[0] == 0 {
                    let id = command[1];

                    my_player_id.0 = Some(PlayerID(id));


                } else if command[0] == 1 {
                    let player_ability: Ability = command[1].into();

                    for (id, mut ability, mut color) in players.iter_mut() {
                        if id.0 == my_player_id.0.as_ref().unwrap().0 {
                            *ability.deref_mut() = player_ability;
                            ability_set.0 = true;

                            *color.deref_mut() = match player_ability {
                                Ability::Phase => materials.phase.clone(),
                                Ability::Engineer => materials.engineer.clone(),
                                Ability::Stim => materials.stim.clone(),
                                Ability::Wall => materials.wall.clone(),

                            };

                            break;

                        }

                    }

                }
            }
        }
    }
}
