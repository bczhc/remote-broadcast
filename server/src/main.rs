use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread::spawn;

use clap::Parser;
use log::{debug, info};

use server::cli::Args;
use server::protocol::{
    DeviceId, Message, PingResult, ReceiverCommand, SenderCommand, ServerRequest, ServerResponse,
};
use server::{setup_logger, RwBincode, RECEIVERS};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    setup_logger(args.log_level.log_level)?;

    debug!("Args: {:?}", args);

    let port = args.listen_port;

    let listener = TcpListener::bind(SocketAddr::V4(format!("0.0.0.0:{port}").parse().unwrap()))?;

    loop {
        let (stream, source) = listener.accept()?;
        spawn(move || {
            info!("Accept: {}", source);
            handle_stream(stream).unwrap();
        });
    }
}

fn handle_stream(mut stream: TcpStream) -> anyhow::Result<()> {
    let message = stream.read_bincode::<Message>()?;

    debug!("Message: {:?}", message);

    match message {
        Message::Sender(c) => match c {
            SenderCommand::Ping(id) => {
                let mut guard = RECEIVERS.lock().unwrap();
                match guard.get_mut(&id) {
                    None => {
                        stream.write_bincode(ServerResponse::Ping(PingResult::Offline))?;
                    }
                    Some(receiver) => {
                        if receiver.write_bincode(ServerRequest::Ping(id)).is_ok() {
                            match receiver.read_bincode::<Message>() {
                                Ok(Message::Receiver(ReceiverCommand::Pong(id))) => {
                                    stream.write_bincode(ServerResponse::Ping(
                                        PingResult::Pong(id),
                                    ))?;
                                }
                                Err(_) => {
                                    stream
                                        .write_bincode(ServerResponse::Ping(PingResult::Failed))?;
                                }
                                _ => {
                                    stream.write_bincode(ServerResponse::Ping(
                                        PingResult::UnexpectedResult,
                                    ))?;
                                }
                            }
                        } else {
                            stream.write_bincode(ServerResponse::Ping(PingResult::Failed))?;
                        }
                    }
                }
            }
        },
        Message::Receiver(c) => match c {
            ReceiverCommand::Connect(id) => {
                info!("Connected: {}", id);
                stream.write_bincode(ServerResponse::Connected)?;
                let mut guard = RECEIVERS.lock().unwrap();
                guard.insert(id, stream);
                let ids = guard.keys().map(DeviceId::to_string).collect::<Vec<_>>();
                debug!("Current receivers: {:?}", ids);
            }
            ReceiverCommand::Pong(x) => {
                stream.write_bincode(ServerResponse::Ping(PingResult::Pong(x)))?;
            }
        },
    }

    Ok(())
}
