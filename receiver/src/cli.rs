use server::cli::LogLevelArg;

#[derive(clap::Parser, Debug)]
pub struct Args {
    #[command(flatten)]
    pub log_level: LogLevelArg,
    // TODO: use `figment`
    pub server_addr: String,
}
