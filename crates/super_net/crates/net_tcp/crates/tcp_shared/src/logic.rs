use crate::*;
use std::fmt::Debug;

use tokio::net::tcp::OwnedReadHalf;
use tokio::net::ToSocketAddrs;
use tokio::io::AsyncReadExt;

pub use turbulence::message_channels::ChannelMessage;

pub async fn add_to_message_queue(mut read_socket: OwnedReadHalf, unprocessed_messages_recv_queue: RecvQueue) -> std::io::Result<()>{
    let mut buffer: [u8; MAX_PACKET_SIZE] = [0; MAX_PACKET_SIZE];

    loop {
        let msg_len: usize = read_socket.read_u32().await?.try_into().unwrap();
        let channel_id = MessageChannelID::new(read_socket.read_u8().await?);

        if msg_len > MAX_PACKET_SIZE {
            eprintln!("Received a packet that was too big!\nPacket was {msg_len} bytes");
            break;
        }

        let msg_buffer = &mut buffer[..msg_len];

        let num_bytes_read = read_socket.read_exact(msg_buffer).await?;

        // If these differ, we read a corrupted message
        assert_eq!(msg_len, num_bytes_read);

        let mut key_val_pair = unprocessed_messages_recv_queue.entry(channel_id).or_insert(Vec::with_capacity(1));
        let messages = key_val_pair.value_mut();

        let byte_vec = msg_buffer.to_vec();

        messages.push(byte_vec);

    }

    Ok(())

}

pub fn generate_message_bin<T>(message: &T, channel: &MessageChannelID) -> Result<Vec<u8>, bincode::Error> where T: ChannelMessage + Debug + Clone {
    let msg_bin = bincode::serialize(message)?;
    // Add one extra byte to the message length for the channel ID
    let msg_len: u32 = msg_bin.len().try_into().unwrap();

    // 4 bytes for the length, 1 byte for the channel ID, the rest for the actual message
    let mut final_message_bin = Vec::with_capacity(4 + 1 + msg_bin.len());

    final_message_bin.extend_from_slice(&msg_len.to_be_bytes());
    final_message_bin.push(channel.id);

    final_message_bin.extend_from_slice(&msg_bin);

    debug_assert_eq!(usize::try_from(u32::from_be_bytes(final_message_bin.as_slice()[..4].try_into().unwrap())).unwrap(), final_message_bin.as_slice()[5..].len());

    Ok(final_message_bin)
}

pub trait TcpResourceTrait {
    /// The actual setup of the network, whether it's connecting or listening
    fn setup(&mut self, addr: impl ToSocketAddrs + Send + 'static);
    fn send_message<T>(&self, message: &T, channel: &MessageChannelID) -> Result<(), SendMessageError> where T: ChannelMessage + Debug + Clone;
}
