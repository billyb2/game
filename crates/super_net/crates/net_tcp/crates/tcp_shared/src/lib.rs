mod logic;

use std::sync::Arc;
use std::net::SocketAddr;

use dashmap::DashMap;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;

use turbulence::message_channels::MessageTypeUnregistered;

pub use logic::*;

// 1 KB max packet size (TODO: move into NetworkSettings struct)
pub const MAX_PACKET_SIZE: usize = 1024;

// Lists of binary messages
pub type RecvQueue = Arc<DashMap<MessageChannelID, Vec<Vec<u8>>>>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ConnID {
    pub uuid: u32,
    pub addr: SocketAddr,
}

impl ConnID {
    pub fn new(uuid: u32, addr: SocketAddr) -> Self {
        Self {
            uuid,
            addr,
        }
    }
}

pub struct ClientConnection {
    pub send_task: JoinHandle<()>,
    pub send_message: UnboundedSender<Vec<u8>>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MessageChannelID {
    pub id: u8,
}

impl MessageChannelID {
    pub const fn new(id: u8) -> Self {
        Self {
            id,

        }
    }
}

#[derive(Debug)]
pub enum MessageRecipient {
    All,
    Single(ConnID),
}

#[derive(Debug)]
pub enum SendMessageError {
    Bincode(bincode::Error),
    Mpsc(SendError<Vec<u8>>),
    NotConnected,
    Turbulence(MessageTypeUnregistered),

}



impl From<bincode::Error> for SendMessageError {
    fn from(error: bincode::Error) -> Self {
        Self::Bincode(error)
    }
}


impl From<SendError<Vec<u8>>> for SendMessageError {
    fn from(error: SendError<Vec<u8>>) -> Self {
        Self::Mpsc(error)
    }
}

impl From<MessageTypeUnregistered> for SendMessageError {
    fn from(error: MessageTypeUnregistered) -> Self {
        Self::Turbulence(error)
    }
}


#[derive(Debug)]
pub enum ChannelProcessingError {
    Bincode(bincode::Error),
    ChannelNotFound,
    Turbulence(MessageTypeUnregistered),
}

impl From<bincode::Error> for ChannelProcessingError {
    fn from(error: bincode::Error) -> Self {
        Self::Bincode(error)
    }
}

impl From<MessageTypeUnregistered> for ChannelProcessingError {
    fn from(error: MessageTypeUnregistered) -> Self {
        Self::Turbulence(error)
    }
}