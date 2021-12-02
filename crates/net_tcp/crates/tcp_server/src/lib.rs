use std::net::SocketAddr;
use std::sync::Arc;

use dashmap::DashMap;

use serde::ser::Serialize;

use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio::task::JoinHandle;

use tcp_shared::*;

// Basically, there's the top level hashmap of each messagechannel, with a vec below that consisting of every message recipient and a binary version of the packet
pub struct TcpServer {
    pub task_pool: Arc<Runtime>,
    pub connection_handler: Option<UnboundedReceiver<(TcpStream, SocketAddr)>>,
    pub connected_clients: Arc<DashMap<ConnID, ClientConnection>>,
    pub unprocessed_message_recv_queue: RecvQueue,
    pub server_handle: Option<JoinHandle<()>>,

}

impl TcpServer {
    pub fn new(task_pool: Arc<Runtime>) -> Self {
        Self {
            task_pool,
            connected_clients: Arc::new(DashMap::new()),
            server_handle: None,
            connection_handler: None,
            unprocessed_message_recv_queue: Arc::new(DashMap::new())
        }
    }

}

impl TcpResource for TcpServer {
    fn setup(&mut self, addr: SocketAddr) {
        let (conn_send, conn_recv) = unbounded_channel();

        let listen_loop = async move {
            let listener = TcpListener::bind(addr).await.unwrap();

            loop {
                if let Ok(socket_and_addr) = listener.accept().await {
                    conn_send.send(socket_and_addr).unwrap();

                }

            }

        };

        self.connection_handler = Some(conn_recv);
        self.server_handle = Some(self.task_pool.spawn(listen_loop));

    }

    fn send_message<T>(&self, message: T, channel: &MessageChannelID) -> Result<(), SendMessageError> where T: Serialize {
        let message_bin = generate_message_bin(message, channel)?;

        for key_val_pair in self.connected_clients.iter() {
            let conn = key_val_pair.value();
            conn.send_message.send(message_bin.clone())?;

        }

        Ok(())


    }

}
