use std::sync::Arc;
use std::net::SocketAddr;

use dashmap::DashMap;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;

use crate::shared::*;
pub use crate::logic::*;
pub use helper_functions::get_available_port;

// Lists of binary messages, along with the handle of who sent it
pub type RecvQueue = Arc<DashMap<MessageChannelID, Vec<(SuperConnectionHandle, Vec<u8>)>>>;

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