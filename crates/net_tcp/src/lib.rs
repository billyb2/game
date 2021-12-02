//TODO: Eventually use Bevy taskpool instead of tokio runtime
//TODO: Maybe using ArrayVec isn't the best idea (because of allocating MAX_PACKET_SIZE per message)

mod types;

use std::sync::Arc;

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;

use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::unbounded_channel;
use tokio::runtime::Builder;

use tcp_shared::*;
use types::*;

pub use tcp_client::TcpClient;
pub use tcp_server::TcpServer;
pub use tcp_shared::{MessageChannelID, TcpResource};

pub type Runtime = Arc<tokio::runtime::Runtime>;

pub struct TcpNetworkingPlugin;

impl Plugin for TcpNetworkingPlugin {
    fn build(&self, app: &mut App) {
        let tokio_rt = Arc::new(Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap());

        app
        .insert_resource(tokio_rt)
        .insert_resource(NextUUID(0))
        .add_system(handle_new_connections_server);
    }
}

fn handle_new_connections_server(mut server: Option<ResMut<TcpServer>>, mut next_uuid: ResMut<NextUUID>) {
    if server.as_ref().is_none() {
        return;

    }

    if server.as_ref().unwrap().connection_handler.as_ref().is_none() {
        return;

    }

    let server = server.as_mut().unwrap();

    // Hope the compiler optimizes out the unwraps lol
    while let Ok((socket, addr)) = server.connection_handler.as_mut().unwrap().try_recv() {
        let conn_id = ConnID {
            uuid: next_uuid.0,
            addr: addr.clone(),

        };

        next_uuid.0 += 1;

        let (read, write) = socket.into_split();

        let server_unprocessed_messages_recv_queue = server.unprocessed_message_recv_queue.clone();

        let (message_sender, mut messages_to_send) = unbounded_channel::<Vec<u8>>();

        let client_connection = ClientConnection {
            receive_task: server.task_pool.spawn(add_to_message_queue(read, server_unprocessed_messages_recv_queue)),
            send_task: server.task_pool.spawn(async move {
                let mut write_socket = write;

                while let Some(message) = messages_to_send.recv().await {
                    write_socket.write_all(&message).await.unwrap();

                }

            }),
            send_message: message_sender,

        };

        server.connected_clients.insert(conn_id, client_connection);
    }

}
