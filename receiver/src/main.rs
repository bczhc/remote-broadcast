use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;

use anyhow::anyhow;
use clap::Parser;
use log::{debug, info};

use receiver::cli::Args;
use receiver::{read_config, write_config};
use server::protocol::{Message, ReceiverCommand, ServerRequest, ServerResponse};
use server::RwBincode;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    server::setup_logger(args.log_level.log_level)?;
    let mut config = read_config()?;
    debug!("Args: {:?}", args);
    debug!("Config: {:?}", config);

    let id = match config.id {
        None => {
            println!("This is the first run. An ID will be generated");
            let id = uuid::Uuid::new_v4().to_string();
            config.id = Some(id.clone());
            write_config(&config)?;
            id
        }
        Some(a) => a,
    };

    println!(
        "The receiver ID is: {}; you can bind this device on the sender using this.",
        id
    );
    qr2term::print_qr(&id)?;
    config.id = Some(id.clone());
    write_config(&config)?;

    info!("Connecting to the server...");
    connect_to_server(&id, &args.server_addr)?;

    Ok(())
}

fn connect_to_server(id: &str, addr: &str) -> anyhow::Result<()> {
    let mut stream = TcpStream::connect(SocketAddr::from_str(addr)?)?;

    stream.write_bincode(Message::Receiver(ReceiverCommand::Connect(id.into())))?;
    if stream.read_bincode::<ServerResponse>()? == ServerResponse::Connected {
        info!("Connected");
    } else {
        return Err(anyhow!("Failed to connect to the server"));
    }

    match stream.read_bincode::<ServerRequest>()? {
        ServerRequest::Ping(x) => {
            debug!("Get ping: {:?}", x);
            stream.write_bincode(Message::Receiver(ReceiverCommand::Pong(x)))?;
        }
    }

    Ok(())
}
