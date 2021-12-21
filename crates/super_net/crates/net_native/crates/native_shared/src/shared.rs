// Some code that's needed by both net_native and super_net
use std::net::SocketAddr;

pub use turbulence::message_channels::{ChannelAlreadyRegistered, ChannelMessage};
pub use game_types::{ConnID, SuperConnectionHandle};

use turbulence::message_channels::MessageTypeUnregistered;

#[cfg(feature = "native")]
use tokio::sync::mpsc::error::SendError;

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
pub enum SendMessageError {
    Bincode(bincode::Error),
    #[cfg(feature = "native")]
    Mpsc(SendError<Vec<u8>>),
    NotConnected,
    Turbulence(MessageTypeUnregistered),

}


impl From<bincode::Error> for SendMessageError {
    fn from(error: bincode::Error) -> Self {
        Self::Bincode(error)
    }
}


#[cfg(feature = "native")]
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
