use std::sync::Arc;
use std::fmt::Debug;
use parking_lot::Mutex;

use dashmap::DashMap;

use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::JoinHandle;

use tcp_shared::*;

// Basically, there's the top level hashmap of each messagechannel, with a vec below that consisting of every message recipient and a binary version of the packet
pub struct TcpServer {
    pub task_pool: Arc<Runtime>,
    pub connection_handler: Option<JoinHandle<()>>,
    pub connected_clients: Arc<DashMap<ConnID, ClientConnection>>,
    pub unprocessed_message_recv_queue: RecvQueue,
    pub server_handle: Option<JoinHandle<()>>,
    pub next_uuid: Arc<Mutex<u32>>,

}

impl TcpServer {
    pub fn new(task_pool: Arc<Runtime>) -> Self {
        Self {
            task_pool,
            connected_clients: Arc::new(DashMap::new()),
            server_handle: None,
            connection_handler: None,
            unprocessed_message_recv_queue: Arc::new(DashMap::new()),
            next_uuid: Arc::new(Mutex::new(0)),
        }
    }
}

impl TcpResourceTrait for TcpServer {
    fn setup(&mut self, addr: impl ToSocketAddrs + Send + 'static) {
        let (conn_send, mut conn_recv) = unbounded_channel();

        // Arc clones of some of self to prevent moving
        let task_pool_clone = Arc::clone(&self.task_pool);
        let connected_clients_clone = Arc::clone(&self.connected_clients);
        let msg_rcv_queue = Arc::clone(&self.unprocessed_message_recv_queue);
        let next_uuid = Arc::clone(&self.next_uuid);

        let listen_loop = async move {
            let listener = TcpListener::bind(addr).await.unwrap();

            while let Ok(socket_and_addr) = listener.accept().await {
                conn_send.send(socket_and_addr).unwrap();

            }

        };

        let handle_connections = async move {
            let task_pool = task_pool_clone;
            let connected_clients = connected_clients_clone;

            while let Some((socket, addr)) = conn_recv.recv().await {
                let task_pool = Arc::clone(&task_pool);

                let msg_rcv_queue = Arc::clone(&msg_rcv_queue);
                let (read_socket, mut write_socket) = socket.into_split();

                let (message_sender, mut messages_to_send) = unbounded_channel::<Vec<u8>>();

                task_pool.spawn(add_to_message_queue(read_socket, msg_rcv_queue));

                let mut next_uuid = next_uuid.lock();

                connected_clients.insert(
                    ConnID {
                        uuid: *next_uuid,
                        addr,
                    },

                    ClientConnection {
                        send_task: task_pool.spawn(async move {
                            while let Some(message) = messages_to_send.recv().await {
                                write_socket.write_all(&message).await.unwrap();

                            }

                        }),
                        send_message: message_sender,
                    }
                );

                *next_uuid += 1;


            }

        };      

        self.connection_handler = Some(self.task_pool.spawn(handle_connections));
        self.server_handle = Some(self.task_pool.spawn(listen_loop));

    }

    fn send_message<T>(&self, message: &T, channel: &MessageChannelID) -> Result<(), SendMessageError> where T: ChannelMessage + Debug + Clone {
        let message_bin = generate_message_bin(message, channel)?;

        for key_val_pair in self.connected_clients.iter() {
            let conn = key_val_pair.value();
            conn.send_message.send(message_bin.clone())?;

        }

        Ok(())


    }

}
