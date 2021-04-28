use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy_networking_turbulence::*;
use bevy::prelude::*;

const SERVER_PORT: u16 = 9363;

pub fn setup_networking(mut net: ResMut<NetworkResource>) {
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
        log::info!("Connecting to {:?}", socket_address);
        println!("Connecting to {:?}", socket_address);

        net.connect(socket_address);

    };
}

#[cfg(feature = "web")]
pub fn send_packets(mut net: ResMut<NetworkResource>, time: Res<Time>) {
    if (time.seconds_since_startup() * 60.) as i64 % 60 == 0 {
        log::info!("Sending packet!");
        net.broadcast(Packet::from("PING"));
    
    }
}

pub fn handle_packets(mut net: ResMut<NetworkResource>, time: Res<Time>, mut reader: EventReader<NetworkEvent>) {
    for event in reader.iter() {
        log::info!("Got an event");
        println!("Got an event");

        match event {
            NetworkEvent::Packet(handle, packet) => {
                let message = String::from_utf8_lossy(&packet);
                log::info!("Got packet on [{}]: {}", handle, message);
                println!("Got packet on [{}]: {}", handle, message);

                if message == "PING" {
                    let message = format!("PONG @ {}", time.seconds_since_startup());
                    match net.send(*handle, Packet::from(message)) {
                        Ok(()) => {
                            log::info!("Sent PONG");
                            println!("Sent PONG");
                        }
                        Err(error) => {
                            log::info!("PONG send error: {}", error);
                            println!("PONG send error: {}", error);

                        }
                    }
                }
            },
            NetworkEvent::Connected(handle) => match net.connections.get_mut(handle) {
                Some(_connection) => {
                    log::info!("Connection successful");
                    println!("Connection successful");

                    //net.send_message(*handle, ClientMessage::Hello)
                    //    .expect("Could not send hello");
                }
                None => panic!("Got packet for non-existing connection [{}]", handle),
            },
            _ => {}
        }
    }
}
