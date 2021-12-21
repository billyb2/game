//TODO: Eventually use Bevy taskpool instead of tokio runtime

use std::fmt::Debug;
use std::sync::Arc;

pub use tokio::io::AsyncWriteExt;
pub use tokio::sync::mpsc::unbounded_channel;
use tokio::net::ToSocketAddrs;

pub use crate::types::*;

use native_client::NativeClient;
use native_server::NativeServer;

pub use native_shared::{TcpCliConn, ConnID, ChannelMessage, ChannelType, ChannelProcessingError, MessageChannelID, SendMessageError, NativeResourceTrait};

pub enum NativeNetResourceWrapper {
    Server(NativeServer),
    Client(NativeClient),

}

//TODO: Generic new fn?
impl NativeNetResourceWrapper {
    pub fn new_server(task_pool: Runtime) -> Self {
        NativeNetResourceWrapper::Server(NativeServer::new(task_pool))

    }

    pub fn new_client(task_pool: Runtime) -> Self {
        NativeNetResourceWrapper::Client(NativeClient::new(task_pool))
    }

    pub fn process_message_channel<T>(&self, channel_id: &MessageChannelID) -> Result<Vec<T>, ChannelProcessingError> where T: ChannelMessage + Debug + Clone {
        let unprocessed_messages_recv_queue = match self {
            NativeNetResourceWrapper::Server(res) => Arc::clone(&res.unprocessed_message_recv_queue),
            NativeNetResourceWrapper::Client(res) => Arc::clone(&res.unprocessed_messages),


        };

        let result = match unprocessed_messages_recv_queue.get_mut(channel_id) {
            Some(mut unprocessed_channel) => {
                let processed_messages = unprocessed_channel.iter().map(|message_bin| {
                    bincode::deserialize::<T>(&message_bin)

                }).collect::<Result<Vec<T>, bincode::Error>>()?;

                // Since we've processed that message channel queue, we should clear it
                unprocessed_channel.clear();

                Ok(processed_messages)
                
            },
            None => {
                unprocessed_messages_recv_queue.insert(channel_id.clone(), Vec::with_capacity(1));

                Ok(Vec::new())

            },

        };

        result

    }

    pub fn is_connected(&self) -> bool {
        match self {
            NativeNetResourceWrapper::Server(res) => res.tcp_connected_clients.len() > 0,
            NativeNetResourceWrapper::Client(res) => res.tcp_msg_sender.is_some(),
        }

    }

    pub fn is_server(&self) -> bool {
        #[cfg(feature = "native")]
        match self {
            NativeNetResourceWrapper::Server(_) => true,
            _ => false,
        }

        // Web builds are never servers (for now)
        #[cfg(feature = "web")]
        false
    }

    pub fn is_client(&self) -> bool {
        !self.is_server()

    }
}

impl NativeResourceTrait for NativeNetResourceWrapper {
    fn setup<const MAX_PACKET_SIZE: usize>(&mut self, udp_addr: impl ToSocketAddrs + Send + 'static, tcp_addr: impl ToSocketAddrs + Send + 'static) {
        match self {
            NativeNetResourceWrapper::Server(res) => res.setup::<MAX_PACKET_SIZE>(udp_addr, tcp_addr),
            NativeNetResourceWrapper::Client(res) => res.setup::<MAX_PACKET_SIZE>(udp_addr, tcp_addr),
        }
    }

    fn send_message<M>(&self, message: &M, channel: &MessageChannelID) -> Result<(), SendMessageError>
        where M: ChannelMessage + Debug + Clone {
        match self {
            NativeNetResourceWrapper::Server(res) => res.send_message(message, channel),
            NativeNetResourceWrapper::Client(res) => res.send_message(message, channel),
        }
    }

    fn register_message(&self, channel: &MessageChannelID, mode: native_shared::ChannelType) -> Result<(), native_shared::ChannelAlreadyRegistered> {
        match self {
            NativeNetResourceWrapper::Server(res) => res.register_message(channel, mode),
            NativeNetResourceWrapper::Client(res) => res.register_message(channel, mode),
        }
    }

}

pub type Runtime = Arc<tokio::runtime::Runtime>;

