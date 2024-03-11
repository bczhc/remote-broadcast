use std::net::TcpStream;

use server::protocol::{Message, SenderCommand, SenderMessage, SenderType, ServerResponse};
use server::RwBincode;

/// This main function is only for testing.
fn main() {
    let server_addr = "localhost:7000";
    let device_id = "f71e5591-76e5-4a59-b62d-cf3e7c3d6b78";
    let mut stream = TcpStream::connect(server_addr).unwrap();

    stream
        .write_bincode(Message::Sender(SenderMessage {
            target: device_id.into(),
            r#type: SenderType::Redirect,
            command: SenderCommand::Ping("haha".into()),
        }))
        .unwrap();

    let response = stream.read_bincode::<ServerResponse>().unwrap();
    println!("{:?}", response);
}
