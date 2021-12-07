// Setup stuff that takes a long time to compile due to large use of const_generics
// By putting it in a seperate module, I'm hoping the compiler will realize it is a candidate for incremental compilation

use std::sync::Arc;
use std::net::SocketAddr;
use std::net::{IpAddr, Ipv4Addr, UdpSocket};
use net_tcp::*;

use bevy::core::Timer;
use bevy::ecs::schedule::State;
use bevy::ecs::system::{Commands, Res, ResMut};
use bevy::utils::Duration;
use bevy_networking_turbulence::*;

use single_byte_hashmap::HashMap;
use game_types::*;

pub(crate) const CLIENT_STATE_MESSAGE_CHANNEL: MessageChannelID = MessageChannelID::new(0);

// Location data is unreliable, since its okay if we skip a few frame updates
pub const CLIENT_STATE_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: CLIENT_STATE_MESSAGE_CHANNEL.id,
    channel_mode: MessageChannelMode::Unreliable,
    // The message buffer size is kind of overkill, but it lets the game lag and not process a good amount of messages for a few seconds and still not be overwhelmed
    message_buffer_size: 1024,
    packet_buffer_size: 1024,
};

pub(crate) const PROJECTILE_MESSAGE_CHANNEL: MessageChannelID = MessageChannelID::new(1);

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

// Some abilities, such as the wall, need to send a message over the network, so this does that here
pub const ABILITY_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
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

// When requesting or sending metadata about the game, such as the assigned player ids or abilities, it's fine to have up to a 10 second delay before getting a response
pub(crate) const INFO_MESSAGE_CHANNEL: MessageChannelID = MessageChannelID::new(4);

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

pub const REQUEST_MAP_OBJECT_SETTINGS: MessageChannelSettings = MessageChannelSettings {
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

pub const DEBUG_TEXT: MessageChannelSettings = MessageChannelSettings {
    channel: 9,
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

pub const TEXT_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 10,
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


pub fn setup_networking(mut commands: Commands, mut net: ResMut<NetworkResource>, mut _app_state: Option<ResMut<State<AppState>>>, _server_addr: Option<Res<SocketAddr>>, hosting: Res<Hosting>, tokio_rt: Res<Runtime>, mut next_uuid: ResMut<NextUUID>) {
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

        net.listen(SocketAddr::new(IP_ADDR, 9363), Some(webrtc_listen_socket), Some(webrtc_listen_socket));

        let mut tcp_res = TcpResourceWrapper::new_server(Arc::clone(&tokio_rt));
        tcp_res.setup(SocketAddr::new(IP_ADDR, 9364));

        commands.insert_resource(tcp_res);

    }

    // Registers message types
    // Because of using many generics, this takes a long time to compile
    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            .register::<ClientStateMessage>(CLIENT_STATE_MESSAGE_SETTINGS)
            .unwrap();

        builder
            .register::<ShootEvent>(PROJECTILE_MESSAGE_SETTINGS)
            .unwrap();

        builder
            .register::<HashMap<u8, u8>>(SCORE_MESSAGE_SETTINGS)
            .unwrap();

        builder
            .register::<InfoMessage>(INFO_MESSAGE_SETTINGS)
            .unwrap();

        builder
            .register::<AbilityMessage>(ABILITY_MESSAGE_SETTINGS)
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
            // Index 0 is the map name, index 1 is the length of the map objects vector, index 2 is the background color, 3 is the map size, 4 is the crc32
            .register::<(String, u64, [f32; 3], [f32; 2], u32)>(MAP_METADATA_SETTINGS)
            .unwrap();

        builder
            .register::<String>(DEBUG_TEXT)
            .unwrap();

        builder
            .register::<TextMessage>(TEXT_MESSAGE_SETTINGS)
            .unwrap();

    });

    commands.insert_resource(ReadyToSendPacket(Timer::new(Duration::from_millis(15), true)));
    #[cfg(feature = "graphics")]
    commands.insert_resource(SetAbility(false));
    

    // If we've previously connected to a server, just connect automatically without prompt
    #[cfg(feature = "web")]
    if let Some(server_addr) = _server_addr {
        net.connect(*server_addr);
    }

    #[cfg(feature = "native")]
    if !hosting.0 {
        let mut tcp_res = TcpResourceWrapper::new_client(Arc::clone(&tokio_rt));

        if let Some(server_addr) = _server_addr.as_ref() {
            tcp_res.setup(*_server_addr.unwrap());

        }

        commands.insert_resource(tcp_res);

    } else {
        _app_state.unwrap().set(AppState::InGame).unwrap();

    }

}


#[cfg(not(target_arch = "wasm32"))]
#[inline]
pub fn get_available_port(ip: &str) -> Option<u16> {
    (8000..9000).into_iter().find(|port| port_is_available(ip, *port))
}

#[cfg(not(target_arch = "wasm32"))]
#[inline(always)]
fn port_is_available(ip: &str, port: u16) -> bool {
    UdpSocket::bind((ip, port)).is_ok()
}
