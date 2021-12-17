//TODO: Eventually use Bevy taskpool instead of tokio runtime
//TODO: Maybe using ArrayVec isn't the best idea (because of allocating MAX_PACKET_SIZE per message)

use std::fmt::Debug;
use std::sync::Arc;

pub use tokio::io::AsyncWriteExt;
pub use tokio::sync::mpsc::unbounded_channel;
use tokio::net::ToSocketAddrs;

pub use crate::types::*;

use tcp_client::TcpClient;
use tcp_server::TcpServer;

pub use tcp_shared::{ClientConnection, ConnID, ChannelMessage, ChannelProcessingError, MessageChannelID, SendMessageError, TcpResourceTrait};

pub enum TcpResourceWrapper {
    Server(TcpServer),
    Client(TcpClient),

}

//TODO: Generic new fn?
impl TcpResourceWrapper {
    pub fn new_server(task_pool: Runtime) -> Self {
        TcpResourceWrapper::Server(TcpServer::new(task_pool))

    }

    pub fn new_client(task_pool: Runtime) -> Self {
        TcpResourceWrapper::Client(TcpClient::new(task_pool))
    }

    pub fn process_message_channel<T>(&self, channel_id: &MessageChannelID) -> Result<Vec<T>, ChannelProcessingError> where T: ChannelMessage + Debug + Clone {
        let unprocessed_messages_recv_queue = match self {
            TcpResourceWrapper::Server(res) => Arc::clone(&res.unprocessed_message_recv_queue),
            TcpResourceWrapper::Client(res) => Arc::clone(&res.unprocessed_recv_messages_queue),


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
            TcpResourceWrapper::Server(res) => res.connected_clients.len() > 0,
            TcpResourceWrapper::Client(res) => res.message_sender.is_some(),
        }

    }

    pub fn is_server(&self) -> bool {
        #[cfg(feature = "native")]
        match self {
            TcpResourceWrapper::Server(_) => true,
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

impl TcpResourceTrait for TcpResourceWrapper {
    fn setup(&mut self, addr: impl ToSocketAddrs + Send + 'static) {
        match self {
            TcpResourceWrapper::Server(tcp_res) => tcp_res.setup(addr),
            TcpResourceWrapper::Client(tcp_res) => tcp_res.setup(addr),
        }
    }

    fn send_message<M>(&self, message: &M, channel: &MessageChannelID) -> Result<(), SendMessageError>
    where M: ChannelMessage + Debug + Clone {
        match self {
            TcpResourceWrapper::Server(tcp_res) => tcp_res.send_message(message, channel),
            TcpResourceWrapper::Client(tcp_res) => tcp_res.send_message(message, channel),
        }
    }

}

pub type Runtime = Arc<tokio::runtime::Runtime>;

