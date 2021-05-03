use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::{PlayerID, ShootEvent};

use bevy_networking_turbulence::*;
use bevy::prelude::*;
use bevy::utils::Duration;

const SERVER_PORT: u16 = 9363;

const CLIENT_STATE_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 0,
    channel_mode: MessageChannelMode::Unreliable,
    // The message buffer size is kind of overkill, but it lets the game lag and not process a good amount of messages for a few seconds and still not be overwhelmed
    message_buffer_size: 1024,
    packet_buffer_size: 1024,
};

const PROJECTILE_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 1,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 8192,
            recv_window_size: 2048,
            send_window_size: 2048,
            burst_bandwidth: 2048,
            init_send: 1024,
            wakeup_time: Duration::from_millis(5),
            initial_rtt: Duration::from_millis(200),
            max_rtt: Duration::from_secs(2),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 128,
    },
    // The message buffer size is kind of overkill, but it lets the game lag and not process a good amount of messages for a few seconds and still not be overwhelmed
    message_buffer_size: 64,
    packet_buffer_size: 64,
};

pub struct ReadyToSendPacket(pub Timer);

pub struct Hosting(pub bool);

pub fn setup_networking(mut commands: Commands, mut net: ResMut<NetworkResource>, hosting: Res<Hosting>) {
    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            .register::<[f32; 2]>(CLIENT_STATE_MESSAGE_SETTINGS)
            .unwrap();

        builder
            .register::<ShootEvent>(PROJECTILE_MESSAGE_SETTINGS)
            .unwrap();

    });

    commands.insert_resource(ReadyToSendPacket(Timer::new(Duration::from_millis(15), false)));

    let socket_address: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), SERVER_PORT);

    #[cfg(feature = "native")]
    if hosting.0 {
        //let ip_address = bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");

        // let socket_address = SocketAddr::new(ip_address, SERVER_PORT);

        println!("Listening on {:?}", &socket_address);
        net.listen(socket_address);

    }

    #[cfg(feature = "web")]
    if !hosting.0 {
        log::info!("Net: Connecting to {:?}", socket_address);
        println!("Connecting to {:?}", socket_address);

        net.connect(socket_address);

    }
}

#[cfg(feature = "web")]
pub fn send_location(mut net: ResMut<NetworkResource>, players: Query<(&Transform, &PlayerID)>, mut ready_to_send_packet: ResMut<ReadyToSendPacket>, hosting: Res<Hosting>) {
    if !hosting.0 {
        // Rate limiting so that the game sends 66 updates every second
        if ready_to_send_packet.0.finished() {
            log::info!("Net: Sending packet!");
            println!("Sending packet");

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

            net.broadcast_message([x, y]);

            ready_to_send_packet.0.reset();

        }
    }

}

pub fn handle_movement_packets(mut net: ResMut<NetworkResource>, mut players: Query<(&mut Transform, &PlayerID)>, mut ready_to_send_packet: ResMut<ReadyToSendPacket>) {
    for (_handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();

        while let Some(m) = channels.recv::<[f32; 2]>() {
            let x = m[0];
            let y = m[1];

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
        for (transform, id) in players.iter_mut() {
            if *id == PlayerID(0) {
                //net.send_message(handle_2, [transform.translation.x, transform.translation.y]).unwrap();
                net.broadcast_message([transform.translation.x, transform.translation.y]);
                break;

            }
        }

        ready_to_send_packet.0.reset();

    }


}

pub fn handle_projectile_packets(mut net: ResMut<NetworkResource>, mut shoot_event: EventWriter<ShootEvent>) {
    for (_handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();

        while let Some(mut event) = channels.recv::<ShootEvent>() {
            event.player_id = 1;
            shoot_event.send(event);

        }

    }

}
