use std::sync::Arc;
use std::fmt::Debug;
use std::net::{SocketAddr, ToSocketAddrs};

use bevy_app::{App, Plugin};
use bevy_tasks::TaskPool;
use bevy_networking_turbulence::*;

use tokio::runtime::Builder;
use tokio::net::ToSocketAddrs as TokioToSocketAddrs;

use net_tcp::*;
pub use net_tcp::{ChannelProcessingError, MessageChannelID, Runtime, SendMessageError};

/// Stores all the networking stuff
pub struct SuperNetworkResource {
    /// Sadly, web clients can't use TCP
    #[cfg(feature = "native")]
    tcp: TcpResourceWrapper,
    /// Naia stuff isn't used for native TCP clients
    naia: Option<NetworkResource>,

}


impl SuperNetworkResource {
    #[cfg(feature = "native")]
    pub fn new_server(tokio_rt: Runtime, task_pool: TaskPool) -> Self {
        Self {
            tcp: TcpResourceWrapper::new_server(tokio_rt),
            naia: Some(NetworkResource::new(task_pool, None, MessageFlushingStrategy::OnEverySend, None, None)),
        }
    }

    pub fn new_client(tokio_rt: Runtime, task_pool: TaskPool) -> Self {
        Self {
            #[cfg(feature = "native")]
            tcp: TcpResourceWrapper::new_client(tokio_rt),
            // The match statement should be optimized out by the compiler
            naia: match cfg!(feature = "native") {
                // Native clients should not use Naia
                true => None,
                // Web clients should use Naia
                false => Some(NetworkResource::new(task_pool, None, MessageFlushingStrategy::OnEverySend, None, None)),

            },
        } 
    }

    /// The WebRTC listen info is only necessary for naia 
    #[cfg(feature = "native")]
    pub fn listen(&mut self, addr: impl TokioToSocketAddrs + Send + 'static, webrtc_listen_info: Option<(impl ToSocketAddrs + Send + 'static, impl ToSocketAddrs + Send + 'static, impl ToSocketAddrs + Send + 'static)>) {
        self.tcp.setup(addr);

        if let Some(naia) = self.naia.as_mut() {
            let (addr, webrtc_listen_addr, public_webrtc_listen_addr) = webrtc_listen_info.unwrap();

            let addr = addr.to_socket_addrs().unwrap().next().unwrap();
            let webrtc_listen_addr = webrtc_listen_addr.to_socket_addrs().unwrap().next().unwrap();
            let public_webrtc_listen_addr = public_webrtc_listen_addr.to_socket_addrs().unwrap().next().unwrap();

            naia.listen(addr, Some(webrtc_listen_addr), Some(public_webrtc_listen_addr));

        }

    }

    pub fn connect(&mut self, addr: SocketAddr) {
        #[cfg(feature = "native")]
        self.tcp.setup(addr);

        if let Some(naia) = self.naia.as_mut() {
            naia.connect(addr);

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

        if let Some(naia) = self.naia.as_mut() {
            for (_handle, connection) in naia.connections.iter_mut() {
                if let Some(channels) = connection.channels() {
                    while let Some(message) = channels.try_recv::<M>()? {
                        messages.push(message);

                    }
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
            // Inlined version of naia.broadcast_message()
            for (handle, connection) in naia.connections.iter_mut() {
                use std::any::type_name;

                let channels = connection.channels().unwrap();
                let result = channels.try_send(message.clone())?;

                if let Some(msg) = result {
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

        #[cfg(feature = "web")]
        let tcp_connected = false;

        #[cfg(feature = "native")]
        let tcp_connected = self.tcp.is_connected();

        tcp_connected || naia_connected

    }

    pub fn as_naia_mut(&mut self) -> Option<&mut NetworkResource> {
        self.naia.as_mut()
    }

}

pub struct SuperNetworkingPlugin;

impl Plugin for SuperNetworkingPlugin {
    fn build(&self, app: &mut App) {
        let tokio_rt = Arc::new(Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap());

        app
        .insert_resource(tokio_rt)
        .add_event::<NetworkEvent>();

    }
}
