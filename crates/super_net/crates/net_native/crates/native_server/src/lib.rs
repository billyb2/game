use std::sync::Arc;
use std::fmt::Debug;
use parking_lot::Mutex;

use dashmap::DashMap;

use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, ToSocketAddrs, UdpSocket};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::JoinHandle;

use native_shared::*;

// Basically, there's the top level hashmap of each messagechannel, with a vec below that consisting of every message recipient and a binary version of the packet
pub struct NativeServer {
    pub task_pool: Arc<Runtime>,
    pub tcp_connection_handler: Option<JoinHandle<()>>,
    pub udp_connection_handler: Option<JoinHandle<()>>,
    pub tcp_connected_clients: Arc<DashMap<ConnID, TcpCliConn>>,
    pub udp_connected_clients: Arc<DashMap<ConnID, UdpCliConn>>,
    pub registered_channels: Arc<DashMap<MessageChannelID, ChannelType>>,
    pub unprocessed_message_recv_queue: RecvQueue,
    pub server_handle: Option<JoinHandle<()>>,
    pub next_uuid: Arc<Mutex<u32>>,

}

impl NativeServer {
    pub fn new(task_pool: Arc<Runtime>) -> Self {
        Self {
            task_pool,
            tcp_connected_clients: Arc::new(DashMap::new()),
            udp_connected_clients: Arc::new(DashMap::new()),
            server_handle: None,
            tcp_connection_handler: None,
            udp_connection_handler: None,
            unprocessed_message_recv_queue: Arc::new(DashMap::new()),
            registered_channels: Arc::new(DashMap::new()),
            next_uuid: Arc::new(Mutex::new(0)),
        }
    }
}

impl NativeResourceTrait for NativeServer {
    fn setup<const MAX_PACKET_SIZE: usize>(&mut self, tcp_addr: impl ToSocketAddrs + Send + 'static, udp_addr: impl ToSocketAddrs + Send + 'static) {
        let (reliable_conn_send, mut reliable_conn_recv) = unbounded_channel();

        // Arc clones of some of self to prevent moving
        let task_pool = Arc::clone(&self.task_pool);
        let task_pool_clone = Arc::clone(&self.task_pool);

        let tcp_connected_clients_clone = Arc::clone(&self.tcp_connected_clients);
        let udp_connected_clients_clone = Arc::clone(&self.udp_connected_clients);

        let msg_rcv_queue = Arc::clone(&self.unprocessed_message_recv_queue);
        let msg_rcv_queue_2 = Arc::clone(&self.unprocessed_message_recv_queue);
        let next_uuid = Arc::clone(&self.next_uuid);
        let next_uuid_2 = Arc::clone(&self.next_uuid);

        let tcp_listen_loop = async move {
            let listener = TcpListener::bind(tcp_addr).await.unwrap();

            while let Ok(socket_and_addr) = listener.accept().await {
                reliable_conn_send.send(socket_and_addr).unwrap();

            }

        };

        //TODO: Remove connection after 15 seconds of inactivity
        let udp_listen_loop = async move {
            let sock = UdpSocket::bind(udp_addr).await.unwrap();
            let arc_sock = Arc::new(sock);

            let mut buffer: [u8; MAX_PACKET_SIZE] = [0; MAX_PACKET_SIZE];

            while let Ok((num_bytes_read, recv_addr)) = arc_sock.recv_from(&mut buffer).await {
                // Checks to see if the addr is in the connected_clients dashmap, and if it isn't, it adds it
                let not_connected = udp_connected_clients_clone.iter().find(|key_val| {
                    let conn_id = key_val.key();
                    conn_id.addr == recv_addr

                }).is_none();

                // If the address isn't already in the DashMap, add it
                if not_connected {
                    let mut next_uuid = next_uuid_2.lock();
                    let (msg_send, mut msg_to_send) = unbounded_channel::<Vec<u8>>();

                    let arc_sock_clone = Arc::clone(&arc_sock);

                    udp_connected_clients_clone.insert(ConnID::new(*next_uuid, recv_addr.clone()), UdpCliConn {
                        send_task: task_pool.spawn(async move {
                            let recv_addr = recv_addr.clone();

                            while let Some(msg) = msg_to_send.recv().await {
                                arc_sock_clone.send_to(&msg, recv_addr.clone()).await.unwrap();

                            }

                        }),
                        send_message: msg_send,
                    });

                    println!("Added new uuid {}", *next_uuid);
                    *next_uuid += 1;

                }

                let msg_len: usize = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]).try_into().unwrap();
                let channel_id = MessageChannelID::new(buffer[4]);

                if msg_len > MAX_PACKET_SIZE {
                    eprintln!("Received a packet that was too big!\nPacket was {msg_len} bytes");
                    break;
                }

                let msg_buffer = &mut buffer[5..msg_len + 5];

                let num_bytes_read = num_bytes_read - 4;

                // If these differ, we read a corrupted message
                // TODO: Error something
                //assert_eq!(msg_len, num_bytes_read);

                let mut key_val_pair = msg_rcv_queue_2.get_mut(&channel_id).unwrap();
                let messages = key_val_pair.value_mut();

                let byte_vec = msg_buffer.to_vec();

                messages.push(byte_vec);

            }

        };

        let handle_connections = async move {
            let task_pool = task_pool_clone;
            let connected_clients = tcp_connected_clients_clone;

            while let Some((socket, addr)) = reliable_conn_recv.recv().await {
                let task_pool = Arc::clone(&task_pool);

                let msg_rcv_queue = Arc::clone(&msg_rcv_queue);
                let (tcp_read_socket, mut tcp_write_socket) = socket.into_split();

                let (tcp_message_sender, mut tcp_messages_to_send) = unbounded_channel::<Vec<u8>>();

                task_pool.spawn(tcp_add_to_msg_queue::<MAX_PACKET_SIZE>(tcp_read_socket, msg_rcv_queue));

                let mut next_uuid = next_uuid.lock();

                connected_clients.insert(
                    ConnID {
                        uuid: *next_uuid,
                        addr,
                    },

                    TcpCliConn {
                        send_task: task_pool.spawn(async move {
                            while let Some(message) = tcp_messages_to_send.recv().await {
                                tcp_write_socket.write_all(&message).await.unwrap();

                            }

                        }),
                        send_message: tcp_message_sender,
                    }
                );

                *next_uuid += 1;


            }

        };      

        self.udp_connection_handler = Some(self.task_pool.spawn(udp_listen_loop));
        self.tcp_connection_handler = Some(self.task_pool.spawn(handle_connections));
        self.server_handle = Some(self.task_pool.spawn(tcp_listen_loop));

    }

    fn send_message<T>(&self, message: &T, channel: &MessageChannelID) -> Result<(), SendMessageError> where T: ChannelMessage + Debug + Clone {
        let message_bin = generate_message_bin(message, channel)?;

        // TODO: Return an error
        let key_val_pair = self.registered_channels.get(channel).unwrap();
        let mode = key_val_pair.value();

        match mode {
            ChannelType::Reliable => {
                for key_val_pair in self.tcp_connected_clients.iter() {
                    let conn = key_val_pair.value();
                    conn.send_message.send(message_bin.clone())?;

                }

            },
            ChannelType::Unreliable => {
                for key_val_pair in self.udp_connected_clients.iter() {
                    let conn = key_val_pair.value();
                    conn.send_message.send(message_bin.clone())?;

                }
            },
        };

        Ok(())


    }

    fn register_message(&self, channel: &MessageChannelID, mode: ChannelType) -> Result<(), ChannelAlreadyRegistered> {
        match self.unprocessed_message_recv_queue.contains_key(channel) {
            false => {
                self.registered_channels.insert(channel.clone(), mode);
                self.unprocessed_message_recv_queue.insert(channel.clone(), Vec::with_capacity(5));

            },
            true => return Err(ChannelAlreadyRegistered::Channel),
        };

        Ok(())
        

    }

}
