use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub mod cli;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub id: Option<String>,
}

static CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let config_dir = dirs::config_dir().expect("Cannot get OS's config dir");
    let app_dir = config_dir.join("remote-broadcast");
    let config_path = app_dir.join("config.json");
    create_dir_all(&app_dir).unwrap_or_else(|_| panic!("Cannot create dir: {}", app_dir.display()));
    config_path
});

pub fn read_config() -> anyhow::Result<Config> {
    if !CONFIG_PATH.exists() {
        let default = Config::default();
        write_config(&default)?;
    }

    let mut file = File::open(CONFIG_PATH.as_path())?;
    let config = serde_json::from_reader::<_, Config>(&mut file)?;
    Ok(config)
}

pub fn write_config(config: &Config) -> anyhow::Result<()> {
    let json = serde_json::to_string(config)?;
    File::create(CONFIG_PATH.as_path())?.write_all(json.as_bytes())?;
    Ok(())
}
