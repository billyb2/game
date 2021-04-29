use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::PlayerID;
//use crate::helper_functions::slice_to_u32;

use serde::{Deserialize, Serialize};

use bevy_networking_turbulence::*;
use bevy::prelude::*;
use bevy::utils::Duration;

const SERVER_PORT: u16 = 9363;

const CLIENT_STATE_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 0,
    /*channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 8192,
            recv_window_size: 2048,
            send_window_size: 2048,
            burst_bandwidth: 2048,
            init_send: 1024,
            wakeup_time: Duration::from_millis(100),
            initial_rtt: Duration::from_millis(200),
            max_rtt: Duration::from_secs(2),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 64,
    },*/
    channel_mode: MessageChannelMode::Unreliable,
    // The message buffer size is kind of overkill, but it lets the game lag and not process a good amount of messages for a few seconds and still not be overwhelmed
    message_buffer_size: 1024,
    packet_buffer_size: 1024,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ClientMessage([f32; 2]);

pub struct ReadyToSendPacket(pub Timer);

pub struct OtherPlayerHandle(Option<ConnectionHandle>);

pub fn setup_networking(mut commands: Commands, mut net: ResMut<NetworkResource>) {
    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            .register::<ClientMessage>(CLIENT_STATE_MESSAGE_SETTINGS)
            .unwrap();

    });

    commands.insert_resource(ReadyToSendPacket(Timer::new(Duration::from_millis(15), false)));

    commands.spawn().insert(OtherPlayerHandle(None));

    #[cfg(feature = "native")]
    {
        //let ip_address = bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");

        // let socket_address = SocketAddr::new(ip_address, SERVER_PORT);
        let socket_address: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), SERVER_PORT);

        println!("Listening on {:?}", &socket_address);
        net.listen(socket_address);

    };

    #[cfg(feature = "web")]
    {
        let socket_address: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), SERVER_PORT);
        log::info!("Net: Connecting to {:?}", socket_address);
        println!("Connecting to {:?}", socket_address);


        //while net.connections.len() < 1 {
            net.connect(socket_address);

        //}

    };
}

#[cfg(feature = "web")]
pub fn send_packets(mut net: ResMut<NetworkResource>, players: Query<(&Transform, &PlayerID)>, mut ready_to_send_packet: ResMut<ReadyToSendPacket>) {
    // Rate limiting so that the game sends 66 updates every second
    if ready_to_send_packet.0.finished() {
        log::info!("Net: Sending packet!");

        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;

        for (transform, id) in players.iter() {
            if *id == PlayerID(0) {
                x = transform.translation.x;
                y = transform.translation.y;

                break;

            }

        }

        //let bytes = [x.to_be_bytes(), y.to_be_bytes()].concat();

        net.broadcast_message(ClientMessage([x, y]));
        ready_to_send_packet.0.reset();

    }

}

pub fn handle_packets(mut net: ResMut<NetworkResource>, mut players: Query<(&mut Transform, &PlayerID)>, mut other_player_handle: Query<&mut OtherPlayerHandle>, mut ready_to_send_packet: ResMut<ReadyToSendPacket>) {
    for (handle, connection) in net.connections.iter_mut() {
        other_player_handle.single_mut().unwrap().0 = Some(*handle);

        let channels = connection.channels().unwrap();

        while let Some(m) = channels.recv::<ClientMessage>() {
                let x = m.0[0];
                let y = m.0[1];

                for (mut transform, id) in players.iter_mut() {
                    if *id == PlayerID(1) {
                        transform.translation.x = x;
                        transform.translation.y = y;

                        break;

                    }

                }

        }

    }

    // Rate limiting so that the game sends 66 updates every second
    if ready_to_send_packet.0.finished() {
        if let Some(handle_2) = other_player_handle.single_mut().unwrap().0 {
            for (transform, id) in players.iter_mut() {
                if *id == PlayerID(0) {
                    net.send_message(handle_2, ClientMessage([transform.translation.x, transform.translation.y])).unwrap();

                }
            }
        }

        ready_to_send_packet.0.reset();

    }


}
