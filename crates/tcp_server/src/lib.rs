//TODO: Eventually use Bevy taskpool instead of tokio runtime

use std::sync::Arc;
use std::net::SocketAddr;

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use bevy_utils::Uuid;

use dashmap::DashMap;

use tokio::io::*;
use tokio::runtime::{Runtime, Builder};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio::task::JoinHandle;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender, UnboundedReceiver};

// 1 KB max packet size (TODO: move into NetworkSettings struct)
const MAX_PACKET_SIZE: usize = 1024;

pub struct TcpNetworkingPlugin;

impl Plugin for TcpNetworkingPlugin {
    fn build(&self, app: &mut App) {
        let rt = Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        app.insert_resource(Server::new(
            rt
        ))
        .add_startup_system(start_server)
        .add_system(handle_new_connections);
    }
}

pub struct Server {
    task_pool: Runtime,
    connection_handler: Option<UnboundedReceiver<(TcpStream, SocketAddr)>>,
    connected_clients: Arc<DashMap<ConnID, ClientConnection>>,
    server_handle: Option<JoinHandle<()>>,

}

impl Server {
    pub fn new(task_pool: Runtime) -> Self {
        Self {
            task_pool,
            connected_clients: Arc::new(DashMap::new()),
            server_handle: None,
            connection_handler: None,
        }
    }

    pub fn listen(&mut self, addr: impl ToSocketAddrs + Send + 'static) {
        let (conn_send, conn_recv) = unbounded_channel();

        let listen_loop = async move {
            let listener = TcpListener::bind(addr).await.unwrap();

            loop {
                if let Ok(socket_and_addr) = listener.accept().await {
                    conn_send.send(socket_and_addr).unwrap();

                }

            }

        };

        //self.task_pool.spawn(handle_new_conn_loop);
        self.connection_handler = Some(conn_recv);
        self.server_handle = Some(self.task_pool.spawn(listen_loop));

    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct ConnID {
    uuid: Uuid,
    addr: SocketAddr,
}

pub struct ClientConnection {
    receive_task: JoinHandle<()>,
    //send_task: JoinHandle<()>,
}

pub struct Message;

pub fn start_server(mut server: ResMut<Server>) {
    server.listen("127.0.0.1:9364");
}

fn handle_new_connections(mut server: ResMut<Server>) {
    if server.connection_handler.as_ref().is_none() {
        return;
    }

    // Hope the compiler optimizes out that unwrap lol
    while let Ok((socket, addr)) = server.connection_handler.as_mut().unwrap().try_recv() {
        let conn_id = ConnID {
            uuid: Uuid::new_v4(),
            addr,

        };

        let (read, write) = socket.into_split();

        // TODO: Use bincode crate for binary serialization
        let client_connection = ClientConnection {

            receive_task: server.task_pool.spawn(async move {
                // Move the read half (the extra let binding isn't really necessary tbh)
                let mut read_socket = read;

                // Empty byte buffer
                let mut buffer: [u8; MAX_PACKET_SIZE] = [0; MAX_PACKET_SIZE];
                // The number of bytes that were written into the buffer
                // Only a slice of buffer[..num_bytes_written] should be taken account of
                let mut num_bytes_written: usize = 0;

                // Wait until there is data to read
                match read_socket.readable().await {
                    Ok(()) => if let Ok(bytes_read) = read_socket.read(&mut buffer).await {
                        num_bytes_written = bytes_read;

                        println!("Received message! {:?}", &buffer[..num_bytes_written]);

                        if let Ok(str_res) = std::str::from_utf8(&buffer[..num_bytes_written]) {
                            println!("Message as string: {}", str_res);
                        }

                    },
                    Err(_) => (),
                }

            }),

        };
    }

}