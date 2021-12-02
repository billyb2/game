use crate::*;

use serde::ser::Serialize;

use tokio::runtime::Runtime;
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::ToSocketAddrs;
use tokio_util::codec::{BytesCodec, FramedRead};
use futures::StreamExt;

pub async fn add_to_message_queue(read: OwnedReadHalf, unprocessed_messages_recv_queue: RecvQueue) {
    // Move the read half (the extra let binding isn't really necessary tbh)
    let mut read_socket = FramedRead::new(read, BytesCodec::new());

    // Wait until there is data to read
    while let Some(Ok(buffer)) = read_socket.next().await {
        let mut index = 0;

        // Mulitple messages inside one received packet
        loop {
            let msg_len: usize = u32::from_be_bytes(buffer[index..index + 4].try_into().unwrap()).try_into().unwrap();

            // Just ignore packets larger than MAX_PACKET_SIZE
            if msg_len > MAX_PACKET_SIZE {
                eprintln!("Received a packet that was too big!");
                break;

            }

            let channel_id = MessageChannelID::new(buffer[index + 4]);

            let message_end_index = msg_len + index + 4;
            let msg_buffer = &buffer[index + 4 + 1..message_end_index];

            let mut key_val_pair = unprocessed_messages_recv_queue.entry(channel_id).or_insert(Vec::with_capacity(1));
            let messages = key_val_pair.value_mut();

            let mut array_vec = ArrayVec::new();
            array_vec.try_extend_from_slice(msg_buffer).unwrap();

            messages.push(array_vec);

            if message_end_index < buffer.len() {
                index += msg_len + 4;

            } else {
                break;

            }

        }


    }

}

pub trait TcpResource {
    /// The actual setup of the network, whether it's connecting or listening
    fn setup(&mut self, addr: SocketAddr);
    fn send_message<T>(&self, message: T, channel: &MessageChannelID) -> Result<(), SendMessageError> where T: Serialize;
}

pub fn generate_message_bin<T>(message: T, channel: &MessageChannelID) -> Result<Vec<u8>, bincode::Error> where T: Serialize {
        let msg_bin = bincode::serialize(&message)?;
        // Add one extra byte to the message length for the channel ID
        let msg_len: u32 = (msg_bin.len() + 1).try_into().unwrap();

        // 4 bytes for the length, 1 byte for the channel ID, the rest for the actual message
        let mut final_message_bin = Vec::with_capacity(4 + 1 + msg_bin.len());

        final_message_bin.extend_from_slice(&msg_len.to_be_bytes());
        final_message_bin.push(channel.id);

        final_message_bin.extend_from_slice(&msg_bin);

        Ok(final_message_bin)
}

pub fn process_message_channel<T>(unprocessed_messages_recv_queue: RecvQueue, message_channel: &MessageChannelID) -> Result<Vec<T>, ChannelProcessingError> where T: serde::de::DeserializeOwned {  
    if let Some(mut unprocessed_channel) = unprocessed_messages_recv_queue.get_mut(&message_channel) {      
        let processed_messages = unprocessed_channel.iter().map(|message_bin| {
            bincode::deserialize::<T>(&message_bin)

        }).collect::<Result<Vec<T>, bincode::Error>>()?;

        // Since we've processed that message channel queue, we should clear it
        unprocessed_channel.clear();

        Ok(processed_messages)

    } else {
        Err(ChannelProcessingError::ChannelNotFound)

    }

}