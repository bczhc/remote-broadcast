use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread::spawn;

use clap::Parser;
use log::{debug, info};

use server::cli::Args;
use server::setup_logger;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    setup_logger(args.log_level)?;

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

fn handle_stream(stream: TcpStream) -> anyhow::Result<()> {
    Ok(())
}
