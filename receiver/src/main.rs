use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;

use anyhow::anyhow;
use clap::Parser;
use log::{debug, info};

use receiver::cli::Args;
use receiver::{read_local_config, user_config, write_local_config};
use server::protocol::{Message, ReceiverCommand, ServerRequest, ServerResponse};
use server::RwBincode;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    server::setup_logger(args.log_level.log_level)?;
    let mut local_config = read_local_config()?;
    debug!("Args: {:?}", args);
    debug!("Local config: {:?}", local_config);

    let id = match local_config.id {
        None => {
            println!("This is the first run. An ID will be generated");
            let id = uuid::Uuid::new_v4().to_string();
            local_config.id = Some(id.clone());
            write_local_config(&local_config)?;
            id
        }
        Some(a) => a,
    };

    println!(
        "The receiver ID is: {}; you can bind this device on the sender using this.",
        id
    );
    qr2term::print_qr(&id)?;
    local_config.id = Some(id.clone());
    write_local_config(&local_config)?;

    let user_config = user_config(args.config_file.config_file)?;
    debug!("User config: {:?}", user_config);

    info!("Connecting to the server...");
    connect_to_server(&id, &user_config.server.addr)?;

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

    // after connecting to the server, wait for the directives
    loop {
        match stream.read_bincode::<ServerRequest>()? {
            ServerRequest::Ping(x) => {
                debug!("Get ping: {:?}", x);
                stream.write_bincode(Message::Receiver(ReceiverCommand::Pong(x)))?;
            }
        }
    }
}
