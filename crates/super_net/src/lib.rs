use std::sync::{Arc, atomic};
use std::fmt::Debug;
use std::net::{SocketAddr, ToSocketAddrs};

use turbulence::{
    buffer::BufferPacketPool,
    message_channels::ChannelMessage,
    packet::{Packet as PoolPacket, PacketPool, MAX_PACKET_LEN},
    packet_multiplexer::{IncomingTrySendError, MuxPacketPool},
};

use bevy_app::{App, Events, Plugin};
use bevy_ecs::system::ResMut;
use bevy_tasks::TaskPool;
use bevy_networking_turbulence::*;

#[cfg(feature = "native")]
use tokio::runtime::Builder;
#[cfg(feature = "native")]
use tokio::net::ToSocketAddrs as TokioToSocketAddrs;

#[cfg(feature = "native")]
use net_tcp::*;
#[cfg(feature = "native")]
pub use net_tcp::{ChannelProcessingError, MessageChannelID, Runtime, SendMessageError};

#[cfg(feature = "web")]
pub use net_tcp::*;

/// Stores all the networking stuff
pub struct SuperNetworkResource {
    /// Sadly, web clients can't use TCP
    #[cfg(feature = "native")]
    tcp: TcpResourceWrapper,
    /// Naia stuff isn't used for native TCP clients
    naia: Option<NetworkResource>,
    is_server: bool,

}

// Fake Tokio runtime struct used for web compat.
#[cfg(feature = "web")]
pub struct Runtime;

impl SuperNetworkResource {
    #[cfg(feature = "native")]
    pub fn new_server(tokio_rt: Option<Runtime>, task_pool: TaskPool) -> Self {
        Self {
            tcp: TcpResourceWrapper::new_server(tokio_rt.unwrap()),
            naia: Some(NetworkResource::new(task_pool, None, MessageFlushingStrategy::OnEverySend, None, None)),
            is_server: true,
        }
    }

    pub fn new_client(tokio_rt: Option<Runtime>, task_pool: TaskPool) -> Self {
        Self {
            #[cfg(feature = "native")]
            tcp: TcpResourceWrapper::new_client(tokio_rt.unwrap()),
            // The match statement should be optimized out by the compiler
            naia: match cfg!(feature = "native") {
                // Native clients should not use Naia
                true => None,
                // Web clients should use Naia
                false => Some(NetworkResource::new(task_pool, None, MessageFlushingStrategy::OnEverySend, None, None)),

            },
            is_server: false,
        } 
    }

    /// The WebRTC listen info is only necessary for naia 
    #[cfg(feature = "native")]
    pub fn listen(&mut self, tcp_addr: impl TokioToSocketAddrs + Send + 'static, webrtc_listen_info: Option<(impl ToSocketAddrs + Send + 'static, impl ToSocketAddrs + Send + 'static, impl ToSocketAddrs + Send + 'static)>) {
        if self.is_server() {
            self.tcp.setup(tcp_addr);

            let naia = self.naia.as_mut().unwrap();

            let (naia_addr, webrtc_listen_addr, public_webrtc_listen_addr) = webrtc_listen_info.unwrap();

            let naia_addr = naia_addr.to_socket_addrs().unwrap().next().unwrap();
            let webrtc_listen_addr = webrtc_listen_addr.to_socket_addrs().unwrap().next().unwrap();
            let public_webrtc_listen_addr = public_webrtc_listen_addr.to_socket_addrs().unwrap().next().unwrap();

            naia.listen(naia_addr, Some(webrtc_listen_addr), Some(public_webrtc_listen_addr));


        } else {
            panic!("Tried to listen while client");

        }

    }

    pub fn connect(&mut self, addr: SocketAddr) {
        if self.is_client() {
            #[cfg(feature = "native")]
            self.tcp.setup(addr);

            if let Some(naia) = self.naia.as_mut() {
                naia.connect(addr);

            }

        } else {
            panic!("Tried to connect while server");

        }

    }

    pub fn view_messages<M>(&mut self, channel: &MessageChannelID) -> Result<Vec<M>, ChannelProcessingError> 
        where M: ChannelMessage + Debug + Clone {
        let mut messages: Vec<M> = Vec::new();

        #[cfg(feature = "native")]
        {
            let mut tcp_messages = self.tcp.process_message_channel(channel)?;
            messages.append(&mut tcp_messages);
        }

        #[cfg(debug_assertions)]
        if self.is_server() && self.naia.is_none() {
            panic!("Server without naia");
        }

        if let Some(naia) = self.naia.as_mut() {
            println!("Server has naia!");

            for (_handle, connection) in naia.connections.iter_mut() {
                println!("Is connected");

                let channels = connection.channels().unwrap();

               println!("Trying to recv msg");

                while let Some(message) = channels.try_recv::<M>()? {
                    println!("Received msg");
                    messages.push(message);

                }
            }

        }

        Ok(messages)

    }

    pub fn broadcast_message<M>(&mut self, message: &M, channel: &MessageChannelID) -> Result<(), SendMessageError>
        where M: ChannelMessage + Debug + Clone {
        #[cfg(feature = "native")]
        self.tcp.send_message(message, channel)?;

        if let Some(naia) = self.naia.as_mut() {
            // Inlined version of naia.broadcast_message(), with some modifications
            for (handle, connection) in naia.connections.iter_mut() {
                use std::any::type_name;

                let channels = connection.channels().unwrap();
                // If the result is Some(msg), that means that the message channel is full, which is no bueno. 
                //  There's probably a better way to do this (TODO?) but since I haven't run into this issue yet, 
                //  I don't care lol
                if channels.try_send(message.clone())?.is_some() {
                    panic!("Message channel full for type: {}", type_name::<M>());

                }

                // Since we're using OnEverySend channel flushing, we don't need the if statement in the normal fn
                channels.try_flush::<M>()?;

            }

        }

        Ok(())

    }

    // A function that only exists for Naia compatibility
    pub fn set_channels_builder<F>(&mut self, builder: F) where F: Fn(&mut ConnectionChannelsBuilder) + Send + Sync + 'static {
        if let Some(naia) = self.naia.as_mut() {
            naia.set_channels_builder(builder);

        }

    }

    pub fn is_connected(&self) -> bool {
        let naia_connected = match self.naia.as_ref() {
            Some(naia) => naia.connections.len() > 0,
            None => false,
        };


        let tcp_connected = {
            #[cfg(feature = "native")]
            let connected = self.tcp.is_connected();

            #[cfg(feature = "web")]
            let connected = false;

            connected

        };

        tcp_connected || naia_connected

    }

    pub fn as_naia_mut(&mut self) -> Option<&mut NetworkResource> {
        self.naia.as_mut()
    }

    pub fn is_server(&self) -> bool {
        self.is_server

    }

    pub fn is_client(&self) -> bool {
        !self.is_server()
    }
}

pub struct SuperNetworkingPlugin;

impl Plugin for SuperNetworkingPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "native")]
        let tokio_rt = Arc::new(Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap());

        #[cfg(feature = "native")]
        app.insert_resource(tokio_rt);

        #[cfg(feature = "web")]
        app.insert_resource(Runtime);

        app
        .add_event::<NetworkEvent>()
        .add_system(rcv_naia_packets);

    }
}

fn rcv_naia_packets(super_net: Option<ResMut<SuperNetworkResource>>, mut network_events: ResMut<Events<NetworkEvent>>) {
    let mut net = match super_net {
        Some(it) => it,
        _ => return,
    };

    let naia = net.as_naia_mut();

    if naia.is_none() {
        return;

    }

    let net = naia.unwrap();

    let pending_connections: Vec<Box<dyn Connection>> = net.pending_connections.lock().unwrap().drain(..).collect();
    
    for mut conn in pending_connections {
        println!("New pending connection!");

        let handle: ConnectionHandle = net
            .connection_sequence
            .fetch_add(1, atomic::Ordering::Relaxed);

        if let Some(channels_builder_fn) = net.channels_builder_fn.as_ref() {
            conn.build_channels(
                channels_builder_fn,
                net.runtime.clone(),
                net.packet_pool.clone(),
            );
        }

        net.connections.insert(handle, conn);
        network_events.send(NetworkEvent::Connected(handle));

    }

    let packet_pool = net.packet_pool.clone();
    for (handle, connection) in net.connections.iter_mut() {
        while let Some(result) = connection.receive() {
            match result {
                Ok(packet) => {
                    // heartbeat packets are empty
                    if packet.len() == 0 {
                        // discard without sending a NetworkEvent
                        continue;
                    }

                    if let Some(channels_rx) = connection.channels_rx() {
                        let mut pool_packet = packet_pool.acquire();
                        pool_packet.resize(packet.len(), 0);
                        pool_packet[..].copy_from_slice(&*packet);

                        if let Err(err) = channels_rx.try_send(pool_packet) {
                           network_events.send(NetworkEvent::Error(
                                *handle,
                                NetworkError::TurbulenceChannelError(err),
                            ));
                        }

                    } else {
                        network_events.send(NetworkEvent::Packet(*handle, packet));
                    }
                }
                Err(err) => {
                    network_events.send(NetworkEvent::Error(*handle, err));
                }
            }
        }
    }
}
