use std::sync::Arc;
use std::net::SocketAddr;

use dashmap::DashMap;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;

use crate::shared::*;
pub use crate::logic::*;
pub use helper_functions::get_available_port;

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

pub struct TcpCliConn {
    pub send_task: JoinHandle<()>,
    pub send_message: UnboundedSender<Vec<u8>>,
}

pub struct UdpCliConn {
    pub send_task: JoinHandle<()>,
    pub send_message: UnboundedSender<Vec<u8>>,
}

pub enum ChannelType {
    Reliable, 
    Unreliable,
}