use server::cli::{ConfigFile, LogLevelArg};

#[derive(clap::Parser, Debug)]
pub struct Args {
    #[command(flatten)]
    pub log_level: LogLevelArg,
    #[command(flatten)]
    pub config_file: ConfigFile,
}
