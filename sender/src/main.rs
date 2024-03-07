use std::net::TcpStream;

use server::protocol::{Message, SenderCommand, ServerResponse};
use server::RwBincode;

/// This main function is only for testing.
fn main() {
    let server_addr = "localhost:7000";
    let device_id = "3eefd746-821e-422f-a695-84e7e2ce457a";
    let mut stream = TcpStream::connect(server_addr).unwrap();
    stream
        .write_bincode(Message::Sender(SenderCommand::Ping(device_id.into())))
        .unwrap();
    let response = stream.read_bincode::<ServerResponse>().unwrap();
    match response {
        ServerResponse::Ping(a) => {
            println!("{:?}", a);
        }
        ServerResponse::Connected => {
            println!("Connected");
        }
    }
}
