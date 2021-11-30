//TODO: Eventually use Bevy taskpool instead of tokio runtime
//TODO: Maybe using ArrayVec isn't the best idea (because of allocating MAX_PACKET_SIZE per message)

use std::result::Result;
use std::sync::Arc;
use std::net::SocketAddr;

use arrayvec::ArrayVec;

use bytes::{BytesMut, BufMut};

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;

use dashmap::DashMap;

use futures::{SinkExt, StreamExt};

use tokio::io::*;
use tokio::runtime::{Runtime, Builder};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio::task::JoinHandle;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender, UnboundedReceiver};

use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

// 1 KB max packet size (TODO: move into NetworkSettings struct)
const MAX_PACKET_SIZE: usize = 1024;

pub struct TcpNetworkingPlugin;

impl Plugin for TcpNetworkingPlugin {
    fn build(&self, app: &mut App) {
        let tokio_rt = Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        app.insert_resource(Server::new(
            tokio_rt
        ))
        .insert_resource(NextUUID(0))
        .add_startup_system(start_server)
        .add_system(handle_new_connections);
    }
}

pub struct Server {
    task_pool: Runtime,
    connection_handler: Option<UnboundedReceiver<(TcpStream, SocketAddr)>>,
    connected_clients: Arc<DashMap<ConnID, ClientConnection>>,
    // A hashmap of hashmaps
    // Basically, there's the top level hashmap of each messagechannel, with a vec below that consisting of every message recipient and a binary version of the packet
    unprocessed_message_recv_queue: 
        Arc<
            DashMap<
                MessageChannelID, 
                Vec<(MessageRecipient, ArrayVec<u8, MAX_PACKET_SIZE>)>
            >,
        >,
    server_handle: Option<JoinHandle<()>>,

}

impl Server {
    fn new(task_pool: Runtime) -> Self {
        Self {
            task_pool,
            connected_clients: Arc::new(DashMap::new()),
            server_handle: None,
            connection_handler: None,
            unprocessed_message_recv_queue: Arc::new(DashMap::new())
        }
    }

    fn listen(&mut self, addr: impl ToSocketAddrs + Send + 'static) {
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

        self.task_pool.spawn(async move {
            let mut socket = TcpStream::connect("127.0.0.1:9364").await.unwrap();
            socket.set_nodelay(true).unwrap();

            let mut slice1 = [0, 0, 0, 0, 0, 0, 0, 0, 1, 9, 9, 9, 9];
            let mut slice2 = [0, 0, 0, 0, 0, 0, 0, 0, 1, 9, 9, 9, 8];

            let slice1_len = slice1.len();
            let slice2_len = slice2.len();

            let len_slice1 = &mut slice1[..4];
            let len_slice2 = &mut slice2[..4];

            // Each slice should have 4 bytes allocated in the beginning for it's size
            len_slice1.copy_from_slice(&((slice1_len - 4) as u32).to_be_bytes());
            len_slice2.copy_from_slice(&((slice2_len - 4) as u32).to_be_bytes());


            socket.write_all(&slice1).await.unwrap();   
            //std::thread::sleep(std::time::Duration::from_secs_f32(1.0 / 60.0));

            socket.write_all(&slice2).await.unwrap();



        });

    }

    pub fn process_message_channel<T>(&self, message_channel: &MessageChannelID) -> Result<Vec<T>, bincode::Error> where T: serde::de::DeserializeOwned {        
        let mut unprocessed_channel = self.unprocessed_message_recv_queue.get_mut(&message_channel).unwrap();

        let processed_messages = unprocessed_channel.iter().map(|(_recipient, message_bin)| {
            bincode::deserialize::<T>(&message_bin)

        }).collect::<Result<Vec<T>, bincode::Error>>()?;

        // Since we've processed that message channel queue, we should clear it
        unprocessed_channel.clear();

        Ok(processed_messages)

    }

}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ConnID {
    uuid: u32,
    addr: SocketAddr,
}

impl ConnID {
    fn new(uuid: u32, addr: SocketAddr) -> Self {
        Self {
            uuid,
            addr,
        }
    }
}

pub struct ClientConnection {
    receive_task: JoinHandle<()>,
    //send_task: JoinHandle<()>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MessageChannelID {
    pub id: u8,
}

struct NextUUID(u32);

impl MessageChannelID {
    pub fn new(id: u8) -> Self {
        Self {
            id,

        }
    }
}

#[derive(Debug)]
enum MessageRecipient {
    All,
    Single(ConnID),
}

pub fn start_server(mut server: ResMut<Server>) {
    server.listen("127.0.0.1:9364");
}

fn handle_new_connections(mut server: ResMut<Server>, mut next_uuid: ResMut<NextUUID>) {
    if server.connection_handler.as_ref().is_none() {
        return;
    }

    // Hope the compiler optimizes out that unwrap lol
    while let Ok((socket, addr)) = server.connection_handler.as_mut().unwrap().try_recv() {
        let conn_id = ConnID {
            uuid: next_uuid.0,
            addr: addr.clone(),

        };

        next_uuid.0 += 1;

        let (read, write) = socket.into_split();

        let unprocessed_message_recv_queue = server.unprocessed_message_recv_queue.clone();

        let client_connection = ClientConnection {
            receive_task: server.task_pool.spawn(async move {
                // Move the read half (the extra let binding isn't really necessary tbh)
                let mut read_socket = FramedRead::new(read, BytesCodec::new());

                // Wait until there is data to read
                while let Some(Ok(buffer)) = read_socket.next().await {
                    let mut index = 0;

                    // Mulitple messages inside one received packet
                    loop {
                        let message_length: usize = u32::from_be_bytes(buffer[index..index + 4].try_into().unwrap()).try_into().unwrap();

                        // Just ignore packets larger than MAX_PACKET_SIZE
                        if message_length > MAX_PACKET_SIZE {
                            eprintln!("Received a packet that was too big!");
                            break;

                        }

                        let channel_id = MessageChannelID::new(buffer[index + 4]);
                        let recipient_uuid = u32::from_be_bytes(buffer[index + 5..index + 9].try_into().unwrap());

                        let msg_buffer = &buffer[index + 9..message_length + index + 4];

                        let mut key_val_pair = unprocessed_message_recv_queue.entry(channel_id).or_insert(Vec::with_capacity(1));
                        let messages = key_val_pair.value_mut();

                        let mut array_vec = ArrayVec::new();
                        array_vec.try_extend_from_slice(msg_buffer).unwrap();

                        messages.push(
                            (
                                MessageRecipient::Single(ConnID::new(recipient_uuid, addr)),
                                array_vec,
                            )
                        );


                        if message_length + 4 + index < buffer.len() {
                            index += message_length + 4;

                        } else {
                            break;

                        }

                    }


                }

            }),

        };

        server.connected_clients.insert(conn_id, client_connection);
    }

}
