use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use figment::providers::{Env, Format, Toml};
use figment::Figment;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub mod cli;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct LocalConfig {
    pub id: Option<String>,
}

static LOCAL_CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let config_dir = dirs::config_dir().expect("Cannot get OS's config dir");
    let app_dir = config_dir.join("remote-broadcast");
    let id_path = app_dir.join("id");
    create_dir_all(&app_dir).unwrap_or_else(|_| panic!("Cannot create dir: {}", app_dir.display()));
    id_path
});

pub fn read_local_config() -> anyhow::Result<LocalConfig> {
    if !LOCAL_CONFIG_PATH.exists() {
        let default = LocalConfig::default();
        write_local_config(&default)?;
    }

    let mut file = File::open(LOCAL_CONFIG_PATH.as_path())?;
    let config = serde_json::from_reader::<_, LocalConfig>(&mut file)?;
    Ok(config)
}

pub fn write_local_config(config: &LocalConfig) -> anyhow::Result<()> {
    let json = serde_json::to_string(config)?;
    File::create(LOCAL_CONFIG_PATH.as_path())?.write_all(json.as_bytes())?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserServerConfig {
    pub addr: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserConfig {
    pub server: UserServerConfig,
}

pub fn user_config(config_file: impl AsRef<Path>) -> figment::Result<UserConfig> {
    Figment::new()
        .merge(Toml::file(config_file))
        .merge(Env::prefixed("RB_APP_"))
        .extract()
}
