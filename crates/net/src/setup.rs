// Setup stuff that takes a long time to compile due to large use of const_generics
// By putting it in a seperate module, I'm hoping the compiler will realize it is a candidate for incremental compilation

use std::sync::Arc;
use std::net::SocketAddr;
use std::net::{IpAddr, Ipv4Addr, UdpSocket};
use super_net::*;

use bevy::core::Timer;
use bevy::tasks::IoTaskPool;
use bevy::ecs::schedule::State;
use bevy::ecs::system::{Commands, Res, ResMut};
use bevy::utils::Duration;
use bevy_networking_turbulence::*;

use single_byte_hashmap::HashMap;
use game_types::*;

use helper_functions::get_available_port;

pub(crate) const CLIENT_STATE_MESSAGE_CHANNEL: MessageChannelID = MessageChannelID::new(0);

// Location data is unreliable, since its okay if we skip a few frame updates
pub const CLIENT_STATE_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: CLIENT_STATE_MESSAGE_CHANNEL.id,
    channel_mode: MessageChannelMode::Unreliable,
    // The message buffer size is kind of overkill, but it lets the game lag and not process a good amount of messages for a few seconds and still not be overwhelmed
    message_buffer_size: 1024,
    packet_buffer_size: 1024,
};

pub const PROJECTILE_MESSAGE_CHANNEL: MessageChannelID = MessageChannelID::new(1);
// Projectile updates are reliable, since when someone shoots a bullet, the server *must* shoot
pub const PROJECTILE_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: PROJECTILE_MESSAGE_CHANNEL.id,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 8192,
            recv_window_size: 8192,
            send_window_size: 8192,
            burst_bandwidth: 8192,
            init_send: 1024,
            wakeup_time: Duration::from_millis(15),
            initial_rtt: Duration::from_millis(80),
            // Bullet shots won't register if ping is above 10 seconds
            max_rtt: Duration::from_secs(10),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 1024,
    },
    //channel_mode: MessageChannelMode::Unreliable,
    message_buffer_size: 2048,
    packet_buffer_size: 4096,
};

pub const ABILITY_MESSAGE_CHANNEL: MessageChannelID = MessageChannelID::new(2);
// Some abilities, such as the wall, need to send a message over the network, so this does that here
pub const ABILITY_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: ABILITY_MESSAGE_CHANNEL.id,
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

// When requesting or sending metadata about the game, such as the assigned player ids or abilities, it's fine to have up to a 10 second delay before getting a response
pub(crate) const INFO_MESSAGE_CHANNEL: MessageChannelID = MessageChannelID::new(3);

pub const INFO_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: INFO_MESSAGE_CHANNEL.id,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 2048,
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
    message_buffer_size: 1024,
    packet_buffer_size: 1024,
};

pub const SCORE_MESSAGE_CHANNEL: MessageChannelID = MessageChannelID::new(4);

pub const SCORE_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 4,
    channel_mode: MessageChannelMode::Unreliable,
    message_buffer_size: 8192,
    packet_buffer_size: 8192,
};

pub(crate) const SET_MAP_CHANNEL: MessageChannelID = MessageChannelID::new(5);

pub const SET_MAP_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: SET_MAP_CHANNEL.id,
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

pub const REQUEST_MAP_OBJECT_CHANNEL: MessageChannelID = MessageChannelID::new(6);
pub const REQUEST_MAP_OBJECT_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: REQUEST_MAP_OBJECT_CHANNEL.id,
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

pub const SEND_MAP_OBJECT_SETTINGS: MessageChannelSettings = MessageChannelSettings {
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

pub const MAP_METADATA_SETTINGS: MessageChannelSettings = MessageChannelSettings {
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

pub const TEXT_MESSAGE_CHANNEL: MessageChannelID = MessageChannelID::new(9);
pub const TEXT_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: TEXT_MESSAGE_CHANNEL.id,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 2048,
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
        max_message_len: 512,
    },
    message_buffer_size: 2048,
    packet_buffer_size: 512,
};

// Type aliases for net messages
// (Player ID, [X, y], [Rotation; 4], health, damage_source, (gun_model, ability), name
pub type ClientStateMessage = (u8, [f32; 2], [f32; 4], f32, f32, Option<u8>, (u8, u8), PlayerName); 

// Various ways of sending some game settings between client and server
pub(crate) type InfoMessage = [u8; 3];

// ([player_id, ability], [player_x, player_y, angle])
pub type AbilityMessage = ([u8; 2], [f32; 3]);

pub type TextMessage = (u8, String, u64);


// A timer of around 15 miliseconds, thatshould be sent (instead of flooding)
pub struct ReadyToSendPacket(pub Timer);

pub struct SetAbility(pub bool);


pub fn setup_networking(mut commands: Commands, mut _app_state: Option<ResMut<State<AppState>>>, server_addr: Option<Res<SocketAddr>>, hosting: Res<Hosting>, tokio_rt: Res<Runtime>, task_pool: Res<IoTaskPool>) {
    #[cfg(feature = "native")]
    let mut net = match hosting.0 {
        true => SuperNetworkResource::new_server(Some(Arc::clone(&tokio_rt)), task_pool.0.clone()),
        false => SuperNetworkResource::new_client(Some(Arc::clone(&tokio_rt)), task_pool.0.clone()),
    };

    #[cfg(feature = "web")]
    let mut net = SuperNetworkResource::new_client(None, task_pool.0.clone());

    // Currently, only PC builds can host
    #[cfg(feature = "native")]
    if hosting.0 {
        // The WebRTC listening address just picks a random port
        let webrtc_listen_socket = {
            let webrtc_listen_ip = match bevy_networking_turbulence::find_my_ip_address() {
                Some(ip) => ip,
                None => {
                    println!("Couldn't find IP address, using 127.0.0.1");
                    println!("Warning: Firefox doesn't allow WebRTC connections to 127.0.0.1, but Chromium does");

                    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))

                },

            };
            let webrtc_listen_port = get_available_port(webrtc_listen_ip.to_string().as_str()).expect("No available port");

            SocketAddr::new(webrtc_listen_ip, webrtc_listen_port)
        };

        const IP_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        const GAME_PORT: u16 = 9363;

        let socket_addr_webrtc = SocketAddr::new(IP_ADDR, GAME_PORT);

        let socket_addr_tcp = SocketAddr::new(IP_ADDR, GAME_PORT + 1);
        let socket_addr_udp = SocketAddr::new(IP_ADDR, GAME_PORT + 2);

        net.listen(socket_addr_tcp, socket_addr_udp, Some((socket_addr_webrtc, webrtc_listen_socket, webrtc_listen_socket)));

    }

    // Registers message types
    // Because of using many generics, this takes a long time to compile
    net.register_message_channel::<ClientStateMessage>(CLIENT_STATE_MESSAGE_SETTINGS, &CLIENT_STATE_MESSAGE_CHANNEL).unwrap();
    net.register_message_channel::<ShootEvent>(PROJECTILE_MESSAGE_SETTINGS, &PROJECTILE_MESSAGE_CHANNEL).unwrap();
    net.register_message_channel::<HashMap<u8, u8>>(SCORE_MESSAGE_SETTINGS, &SCORE_MESSAGE_CHANNEL).unwrap();
    net.register_message_channel::<InfoMessage>(INFO_MESSAGE_SETTINGS, &INFO_MESSAGE_CHANNEL).unwrap();
    net.register_message_channel::<AbilityMessage>(ABILITY_MESSAGE_SETTINGS, &ABILITY_MESSAGE_CHANNEL).unwrap();
    net.register_message_channel::<u32>(SET_MAP_SETTINGS, &SET_MAP_CHANNEL).unwrap();
    net.register_message_channel::<(u32, u64)>(REQUEST_MAP_OBJECT_SETTINGS, &REQUEST_MAP_OBJECT_CHANNEL).unwrap();
    net.register_message_channel::<TextMessage>(TEXT_MESSAGE_SETTINGS, &TEXT_MESSAGE_CHANNEL).unwrap();
    
    commands.insert_resource(ReadyToSendPacket(Timer::new(Duration::from_millis(15), true)));
    #[cfg(feature = "graphics")]
    commands.insert_resource(SetAbility(false));
    

    // If we've previously connected to a server, just connect automatically without prompt
    if !hosting.0 {
        if let Some(server_addr) = server_addr {
            net.connect(*server_addr, Some(SocketAddr::new(server_addr.ip(), 9365)));
        }
    } else {
        _app_state.unwrap().set(AppState::InGame).unwrap();
    }

    commands.insert_resource(net);

}

