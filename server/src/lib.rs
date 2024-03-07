use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Mutex;
use std::time::SystemTime;

use bincode::config::Configuration;
use bincode::{Decode, Encode};
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use log::LevelFilter;
use once_cell::sync::Lazy;

use crate::protocol::DeviceId;

pub mod cli;
pub mod protocol;

const DEVICE_ID_LENGTH: usize = 36 /* UUID v4 */;

pub static RECEIVERS: Lazy<Mutex<HashMap<DeviceId, TcpStream>>> =
    Lazy::new(|| Mutex::new(Default::default()));

pub fn setup_logger(level: LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(level)
        .chain(io::stdout())
        .apply()?;
    Ok(())
}

pub fn bincode_config() -> Configuration {
    bincode::config::standard()
}

/// Format: \[ length (u32le) | bincode steam \]
pub trait RwBincode {
    fn write_bincode<S: Encode>(&mut self, obj: S) -> io::Result<usize>;

    fn read_bincode<S: Decode>(&mut self) -> io::Result<S>;
}

impl<Rw> RwBincode for Rw
where
    Rw: Write + Read,
{
    fn write_bincode<S: Encode>(&mut self, obj: S) -> io::Result<usize> {
        let vec = bincode::encode_to_vec(obj, bincode_config()).expect("Bincode encode error");
        self.write_u32::<LE>(vec.len() as u32)?;
        self.write_all(&vec)?;
        Ok(vec.len())
    }

    fn read_bincode<S: Decode>(&mut self) -> io::Result<S> {
        let length = self.read_u32::<LE>()?;
        let mut reader = self.take(length as u64);
        bincode::decode_from_std_read(&mut reader, bincode_config())
            .map_err(|e| io::Error::other(format!("Bincode error: {}", e)))
    }
}
