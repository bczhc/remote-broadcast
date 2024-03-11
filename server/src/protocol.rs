use bincode::{Decode, Encode};

pub type DeviceId = String;

#[derive(Encode, Decode, Debug, Eq, PartialEq)]
pub enum SenderType {
    Redirect,
}

#[derive(Encode, Decode, Debug, Eq, PartialEq)]
pub struct SenderMessage {
    pub target: DeviceId,
    pub r#type: SenderType,
    pub command: SenderCommand,
}

#[derive(Encode, Decode, Debug, Eq, PartialEq)]
pub enum SenderCommand {
    /// Check if the receiver is online.
    Ping(String /* payload */),
}

#[derive(Encode, Decode, Debug, Eq, PartialEq)]
pub enum ReceiverMessage {
    /// Connect to the server.
    Connect(DeviceId),
}

#[derive(Encode, Decode, Debug, Eq, PartialEq)]
pub enum Message {
    Sender(SenderMessage),
    Receiver(ReceiverMessage),
}

#[derive(Encode, Decode, Debug, Eq, PartialEq)]
pub enum ReceiverResponse {
    Pong(String),
}

#[derive(Encode, Decode, Debug, Eq, PartialEq)]
pub enum ServerResponse {
    Connected,
    DeviceOffline,
    Response(ReceiverResponse),
}
