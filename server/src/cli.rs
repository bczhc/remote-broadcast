#[derive(clap::Parser, Debug)]
pub struct Args {
    #[arg(short = 'p', long, default_value = "7000")]
    pub listen_port: u16,
    #[arg(
        short,
        long,
        help = "Log level. [off, error, warn, info, debug, trace]",
        default_value = "info"
    )]
    pub log_level: log::LevelFilter,
}
