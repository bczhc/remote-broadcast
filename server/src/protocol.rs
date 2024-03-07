use bincode::{Decode, Encode};

pub type DeviceId = String;

#[derive(Encode, Decode, Debug, Eq, PartialEq)]
pub enum SenderCommand {
    /// Check if the specified receiver is online.
    Ping(DeviceId),
}

#[derive(Encode, Decode, Debug, Eq, PartialEq)]
pub enum ReceiverCommand {
    /// Connect to the server.
    Connect(DeviceId),
    Pong(DeviceId),
}

#[derive(Encode, Decode, Debug, Eq, PartialEq)]
pub enum PingResult {
    Pong(DeviceId),
    Offline,
    UnexpectedResult,
    Failed,
}

#[derive(Encode, Decode, Debug, Eq, PartialEq)]
pub enum ServerResponse {
    Ping(PingResult),
    Connected,
}

#[derive(Encode, Decode, Debug, Eq, PartialEq)]
pub enum ServerRequest {
    Ping(DeviceId),
}

#[derive(Encode, Decode, Debug, Eq, PartialEq)]
pub enum Message {
    Sender(SenderCommand),
    Receiver(ReceiverCommand),
}
