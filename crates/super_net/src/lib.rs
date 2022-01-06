#![feature(explicit_generic_args_with_impl_trait)]

use std::any::type_name;
use std::sync::{Arc, atomic};
use std::fmt::Debug;
use std::net::{SocketAddr, ToSocketAddrs};

use turbulence::message_channels::ChannelAlreadyRegistered;
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
use net_native::*;
#[cfg(feature = "native")]
pub use net_native::{ChannelProcessingError, MessageChannelID, Runtime, SendMessageError, SuperConnectionHandle};

#[cfg(feature = "web")]
pub use net_native::*;

/// Stores all the networking stuff
pub struct SuperNetworkResource {
    /// Sadly, web clients can't use TCP
    #[cfg(feature = "native")]
    native: NativeNetResourceWrapper,
    /// Naia stuff is used for native WebRTC servers and wasm clients
    naia: Option<NetworkResource>,
    is_server: bool,
    is_setup: bool,

}

// Fake Tokio runtime struct used for web compat.
#[cfg(feature = "web")]
pub struct Runtime;

impl SuperNetworkResource {
    #[cfg(feature = "native")]
    pub fn new_server(tokio_rt: Option<Runtime>, task_pool: TaskPool) -> Self {
        Self {
            native: NativeNetResourceWrapper::new_server(tokio_rt.unwrap()),
            naia: Some(NetworkResource::new(task_pool, None, MessageFlushingStrategy::OnEverySend, None, None)),
            is_server: true,
            is_setup: false,
        }
    }

    pub fn new_client(tokio_rt: Option<Runtime>, task_pool: TaskPool) -> Self {
        Self {
            #[cfg(feature = "native")]
            native: NativeNetResourceWrapper::new_client(tokio_rt.unwrap()),
            // The match statement should be optimized out by the compiler
            #[cfg(feature = "native")]
            // Native clients should not use Naia
            naia: None,
            // Web clients should
            #[cfg(feature = "web")]
            naia: Some(NetworkResource::new(task_pool, None, MessageFlushingStrategy::OnEverySend, None, None)),
            is_server: false,
            is_setup: false,
        } 
    }

    /// The WebRTC listen info is only necessary for naia 
    #[cfg(feature = "native")]
    pub fn listen<const MAX_NATIVE_PACKET_SIZE: usize>(&mut self, tcp_addr: impl TokioToSocketAddrs + Send + Clone + 'static, udp_addr: impl TokioToSocketAddrs + Send + Clone + 'static, webrtc_listen_info: Option<(impl ToSocketAddrs + Send + 'static, impl ToSocketAddrs + Send + 'static, impl ToSocketAddrs + Send + 'static)>) {
        if self.is_server() {
            #[cfg(feature = "native")]
            self.native.setup::<MAX_NATIVE_PACKET_SIZE>(tcp_addr, udp_addr);

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

    // TODO: Make this an impl ToSocketAddr
    pub fn connect<const MAX_NATIVE_PACKET_SIZE: usize>(&mut self, addr: SocketAddr, udp_addr: Option<SocketAddr>) {
        if self.is_client() {
            #[cfg(feature = "native")]
            self.native.setup::<MAX_NATIVE_PACKET_SIZE>(addr, udp_addr.unwrap());

            #[cfg(feature = "web")]
            if let Some(naia) = self.naia.as_mut() {
                naia.connect(addr);

            }

        } else {
            panic!("Tried to connect while server");

        }

    }

    pub fn view_messages<M>(&mut self, channel: &MessageChannelID) -> Result<Vec<(SuperConnectionHandle, M)>, ChannelProcessingError> 
        where M: ChannelMessage + Debug + Clone {
        let mut messages: Vec<(SuperConnectionHandle, M)> = Vec::new();

        #[cfg(feature = "native")]
        {
            let mut tcp_messages = self.native.process_message_channel(channel)?;
            messages.append(&mut tcp_messages);
        }

        if let Some(naia) = self.naia.as_mut() {
            for (handle, connection) in naia.connections.iter_mut() {
                let channels = connection.channels().unwrap();

                while let Some(message) = channels.try_recv::<M>()? {
                    messages.push((SuperConnectionHandle::new_naia(*handle), message));

                }
            }

        }

        Ok(messages)

    }

    pub fn broadcast_message<M>(&mut self, message: &M, channel: &MessageChannelID) -> Result<(), SendMessageError>
        where M: ChannelMessage + Debug + Clone {
        #[cfg(feature = "native")]
        self.native.broadcast_message(message, channel)?;

        if let Some(naia) = self.naia.as_mut() {
            // Inlined version of naia.broadcast_message(), with some modifications
            if naia.connections.len() > 0 {
                for (_handle, connection) in naia.connections.iter_mut() {
                    let channels = connection.channels().unwrap();
                    // If the result is Some(msg), that means that the message channel is full, which is no bueno. 
                    //  There's probably a better way to do this (TODO?) but since I haven't run into this issue yet, 
                    //  I don't care lol
                    if channels.try_send(message.clone())?.is_some() {
                        panic!("Message channel full for type: {:?}", type_name::<M>());

                    }

                    // Since we're using OnEverySend channel flushing, we don't need the if statement in the normal fn
                    channels.try_flush::<M>()?;

                }
            // If there are no connections and the resource is a client
            } else if self.is_client() {
                return Err(SendMessageError::NotConnected);

            }

        }

        Ok(())

    }

    pub fn send_message<M>(&mut self, message: &M, channel: &MessageChannelID, handle: &SuperConnectionHandle) -> Result<(), SendMessageError>
        where M: ChannelMessage + Debug + Clone {
        if handle.is_native() {
            #[cfg(feature = "native")]
            self.native.send_message(message, channel, handle.native())?;

        } else {
            let naia = self.naia.as_mut().unwrap();

            // Inlined version of naia.send_message(), with some modifications
            match naia.connections.get_mut(handle.naia()) {
                Some(connection) => {
                    let channels = connection.channels().unwrap();
                    if channels.try_send(message.clone())?.is_some() {
                        panic!("Message channel full for type: {:?}", type_name::<M>());

                    }

                    channels.try_flush::<M>()?;
                }
                None => return Err(SendMessageError::NotConnected),
            }

        }

        Ok(())

    }

    pub fn register_message_channel_native(&mut self, settings: MessageChannelSettings, channel: &MessageChannelID) -> Result<(), ChannelAlreadyRegistered> {
        #[cfg(feature = "native")]
        self.native.register_message(channel, match &settings.channel_mode {
            MessageChannelMode::Unreliable => ChannelType::Unreliable,
            _ => ChannelType::Reliable,

        })?;

        Ok(())
        
    }

    // TODO: Combine register_message_channel_native with this fn
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
            let connected = self.native.is_connected();

            #[cfg(feature = "web")]
            let connected = false;

            connected

        };

        tcp_connected || naia_connected

    }

    pub fn disconnect_from(&mut self, handle: &SuperConnectionHandle) -> Result<(), DisconnectError> {
        #[cfg(feature = "native")]
        if handle.is_native() {
            let handle = handle.native();
            self.native.disconnect_from(handle)?;

        }

        if handle.is_naia() {
            let handle = handle.naia();

            match self.naia.as_mut() {
                Some(naia) => naia.disconnect(*handle),
                None => Err(DisconnectError::NotConnected)?,

            };

        }

        Ok(())

    }

    pub fn disconnect_from_all(&mut self) {
        #[cfg(feature = "native")]
        self.native.disconnect_from_all();

        if let Some(naia) = self.naia.as_mut() {
            naia.connections.clear()

        }

    }

    pub fn as_naia_mut(&mut self) -> Option<&mut NetworkResource> {
        self.naia.as_mut()
    }

    pub fn is_server(&self) -> bool {
        self.is_server

    }

    pub fn is_client(&self) -> bool {
        !self.is_server
    }

    pub fn is_setup(&self) -> bool {
        self.is_setup
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
