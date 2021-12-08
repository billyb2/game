use std::sync::Arc;
use std::fmt::Debug;

use dashmap::DashMap;

use tokio::io::AsyncWriteExt;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::task::JoinHandle;

use tcp_shared::*;

pub struct TcpClient {
    pub task_pool: Arc<Runtime>,
    write_handle: Option<JoinHandle<()>>,
    read_handle: Option<JoinHandle<()>>,
    pub message_sender: Option<UnboundedSender<Vec<u8>>>,
    pub unprocessed_recv_messages_queue: RecvQueue,

}

impl TcpClient {
    pub fn new(task_pool: Arc<Runtime>) -> Self {
        Self {
            task_pool,
            write_handle: None,
            read_handle: None,
            message_sender: None,
            unprocessed_recv_messages_queue: Arc::new(DashMap::new()),
        }
    }
}

impl TcpResourceTrait for TcpClient {
    fn setup(&mut self, addr: impl ToSocketAddrs + Send + 'static) {
        let m_queue = Arc::clone(&self.unprocessed_recv_messages_queue);
        let task_pool = Arc::clone(&self.task_pool);

        let (message_sender, mut message_receiver) = unbounded_channel::<Vec<u8>>();

        self.task_pool.spawn(async move {
            let socket = TcpStream::connect(addr).await.unwrap();
            let (read_socket, mut write_socket) = socket.into_split();

            let m_queue_clone = Arc::clone(&m_queue);

            let send_loop = async move {
                while let Some(message) = message_receiver.recv().await {
                    write_socket.write_all(&message).await.unwrap();

                }
            };

            let write_handle = task_pool.spawn(send_loop);
            let read_handle = task_pool.spawn(add_to_message_queue(read_socket, m_queue_clone));
        });

        self.message_sender = Some(message_sender);


    }

    fn send_message<T>(&self, message: &T, channel: &MessageChannelID) -> Result<(), SendMessageError> where T: ChannelMessage + Debug + Clone {
        let message_bin = generate_message_bin(message, channel)?;

        self.message_sender.as_ref().unwrap().send(message_bin)?;

        Ok(())

    }
}
