use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread::spawn;

use clap::Parser;
use log::{debug, info};

use server::cli::Args;
use server::protocol::{Message, ReceiverMessage, ReceiverResponse, SenderType, ServerResponse};
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
        Message::Sender(m) => {
            assert_eq!(m.r#type, SenderType::Redirect);

            let target = &m.target;
            match RECEIVERS.lock().unwrap().get_mut(target) {
                None => {
                    stream.write_bincode(ServerResponse::DeviceOffline)?;
                }
                Some(s) => {
                    // redirect
                    s.write_bincode(&m.command)?;
                    let r = s.read_bincode::<ReceiverResponse>()?;
                    stream.write_bincode(ServerResponse::Response(r))?;
                }
            }
        }
        Message::Receiver(m) => match m {
            ReceiverMessage::Connect(x) => {
                stream.write_bincode(ServerResponse::Connected)?;
                RECEIVERS.lock().unwrap().insert(x, stream);
            }
        },
    }

    Ok(())
}
