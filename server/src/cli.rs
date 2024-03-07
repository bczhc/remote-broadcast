#[derive(clap::Parser, Debug)]
pub struct Args {
    #[arg(short = 'p', long, default_value = "7000")]
    pub listen_port: u16,
    #[command(flatten)]
    pub log_level: LogLevelArg,
}

#[derive(clap::Args, Debug)]
pub struct LogLevelArg {
    #[arg(
        short,
        long,
        help = "Log level. [off, error, warn, info, debug, trace]",
        default_value = "debug"
    )]
    pub log_level: log::LevelFilter,
}
