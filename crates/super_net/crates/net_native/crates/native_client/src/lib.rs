use std::net::{SocketAddrV4, Ipv4Addr};
use std::sync::Arc;
use std::fmt::Debug;

use dashmap::DashMap;

use tokio::io::AsyncWriteExt;
use tokio::net::{TcpStream, ToSocketAddrs, UdpSocket, lookup_host};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::task::JoinHandle;

use native_shared::*;

pub struct NativeClient {
    pub task_pool: Arc<Runtime>,
    write_task_handle: Option<JoinHandle<()>>,
    read_task_handle: Option<JoinHandle<()>>,
    pub tcp_msg_sender: Option<UnboundedSender<Vec<u8>>>,
    pub udp_msg_sender: Option<UnboundedSender<Vec<u8>>>,
    pub unprocessed_messages: RecvQueue,
    pub registered_channels: Arc<DashMap<MessageChannelID, ChannelType>>,

}

impl NativeClient {
    pub fn new(task_pool: Arc<Runtime>) -> Self {
        Self {
            task_pool,
            write_task_handle: None,
            read_task_handle: None,
            tcp_msg_sender: None,
            udp_msg_sender: None,
            unprocessed_messages: Arc::new(DashMap::new()),
            registered_channels: Arc::new(DashMap::new()),
        }
    }
}

impl NativeResourceTrait for NativeClient {
    fn setup<const MAX_PACKET_SIZE: usize>(&mut self, tcp_addr: impl ToSocketAddrs + Send + Clone + 'static, udp_addr: impl ToSocketAddrs + Send + Clone + 'static) {
        let m_queue = Arc::clone(&self.unprocessed_messages);
        let m_queue_2 = Arc::clone(&self.unprocessed_messages);

        let task_pool = Arc::clone(&self.task_pool);
        let task_pool_2 = Arc::clone(&self.task_pool);

        let (udp_message_sender, mut udp_message_receiver) = unbounded_channel::<Vec<u8>>();
        let (tcp_message_sender, mut tcp_message_receiver) = unbounded_channel::<Vec<u8>>();

        let msg_rcv_queue = Arc::clone(&self.unprocessed_messages);

        self.task_pool.spawn(async move {
            let socket = TcpStream::connect(tcp_addr).await.unwrap();
            let peer_addr = socket.peer_addr();

            let (read_socket, mut write_socket) = socket.into_split();

            let m_queue_clone = Arc::clone(&m_queue);

            let send_loop = async move {
                while let Some(message) = tcp_message_receiver.recv().await {
                    write_socket.write_all(&message).await.unwrap();

                }
            };

            let write_handle = task_pool.spawn(send_loop);
            // Only one possible connection means that I can just make SuperConnectionHandle a constant
            let handle = SuperConnectionHandle::new_native(ConnID {
                uuid: 0,
                addr: peer_addr.unwrap(),
                mode: NativeConnectionType::Tcp,
            });

            let read_handle = task_pool.spawn(tcp_add_to_msg_queue::<MAX_PACKET_SIZE>(read_socket, m_queue_clone, handle));
        });

        self.task_pool.spawn(async move {
            let port = get_available_port("0.0.0.0").unwrap();
            let socket_addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);

            let socket = UdpSocket::bind(socket_addr).await.unwrap();
            let udp_addr_clone = udp_addr.clone();

            socket.connect(udp_addr).await.unwrap();

            let socket = Arc::new(socket);
            let socket_write_clone = Arc::clone(&socket);
            let socket_read_clone = Arc::clone(&socket);

            let m_queue_clone = Arc::clone(&m_queue_2);

            let send_loop = async move {
                while let Some(message) = udp_message_receiver.recv().await {
                    socket_write_clone.send(&message).await.unwrap();

                }
            };

            let write_handle = task_pool_2.spawn(send_loop);

            let read_handle = task_pool_2.spawn(async move {
                let sock = socket_read_clone;

                let mut buffer: [u8; MAX_PACKET_SIZE] = [0; MAX_PACKET_SIZE];
                let udp_addr = lookup_host(udp_addr_clone).await.unwrap().nth(0).unwrap();

                while let Ok(total_num_bytes_read) = sock.recv(&mut buffer).await {
                    let msg_len: usize = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]).try_into().unwrap();
                    let channel_id = MessageChannelID::new(buffer[4]);

                    if msg_len > MAX_PACKET_SIZE {
                        eprintln!("Received a packet that was too big!\nPacket was {msg_len} bytes");
                        break;
                    }

                    let msg_buffer = &mut buffer[5..msg_len + 5];

                    let msg_num_bytes_read = total_num_bytes_read - 4;

                    // If these differ, we read a corrupted message
                    // TODO: Error something
                    //assert_eq!(msg_len, num_bytes_read);

                    let mut key_val_pair = msg_rcv_queue.get_mut(&channel_id).unwrap();
                    let messages = key_val_pair.value_mut();

                    let byte_vec = msg_buffer.to_vec();

                    messages.push((SuperConnectionHandle::new_native(ConnID::new(0, udp_addr.clone(), NativeConnectionType::Udp)), byte_vec));


                }
            });


        });

        self.tcp_msg_sender = Some(tcp_message_sender);
        self.udp_msg_sender = Some(udp_message_sender);

    }

    fn broadcast_message<T>(&self, message: &T, channel: &MessageChannelID) -> Result<(), SendMessageError> where T: ChannelMessage + Debug + Clone {
        let message_bin = generate_message_bin(message, channel)?;

        // TODO: Return an error
        let key_val_pair = self.registered_channels.get(channel).unwrap();
        let mode = key_val_pair.value();

        let message_sender = match mode {
            ChannelType::Reliable => &self.tcp_msg_sender,
            ChannelType::Unreliable => &self.udp_msg_sender,
        };

        message_sender.as_ref().unwrap().send(message_bin)?;

        Ok(())

    }

    // Since native clients can currently only have one connnection (currently), we can just reuse the broadcast_message code
    fn send_message<T>(&self, message: &T, channel: &MessageChannelID, _conn_id: &ConnID) -> Result<(), SendMessageError> where T: ChannelMessage + Debug {
        let message_bin = generate_message_bin(message, channel)?;

        // TODO: Return an error
        let key_val_pair = self.registered_channels.get(channel).unwrap();
        let mode = key_val_pair.value();

        let message_sender = match mode {
            ChannelType::Reliable => &self.tcp_msg_sender,
            ChannelType::Unreliable => &self.udp_msg_sender,
        };

        message_sender.as_ref().unwrap().send(message_bin)?;

        Ok(())        
        
    }

    fn register_message(&self, channel: &MessageChannelID, mode: ChannelType) -> Result<(), ChannelAlreadyRegistered> {
        if self.registered_channels.contains_key(channel) {
            Err(ChannelAlreadyRegistered::Channel)

        } else {
            self.registered_channels.insert(channel.clone(), mode);
            self.unprocessed_messages.insert(channel.clone(), Vec::with_capacity(5));

            Ok(())

        }
    }

    fn disconnect_from_all(&mut self) {
        if let Some(write_handle) = self.write_task_handle.as_ref() {
            write_handle.abort();

        }

        if let Some(read_handle) = self.read_task_handle.as_ref() {
            read_handle.abort();

        }

        self.read_task_handle = None;
        self.write_task_handle = None;
        self.tcp_msg_sender = None;
        self.udp_msg_sender = None;
        self.unprocessed_messages.clear();

    }

    fn disconnect_from(&mut self, _conn_id: &ConnID) -> Result<(), DisconnectError> {
        self.disconnect_from_all();

        Ok(())

    }
}
