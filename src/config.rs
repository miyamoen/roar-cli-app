extern crate toml;
use std::string::String;
use std::fs::File;
use std::path::PathBuf;
use std::io::{BufReader, BufWriter, Read, Write};
use std::io;
use std::env;

fn config_path() -> PathBuf {
    let mut path = env::home_dir().unwrap();
    path.push(".roar.toml");
    path
}

pub fn read() -> Result<Config, ConfigError> {
    let config = match File::open(config_path()) {
        Ok(file) => parse(file)?,
        Err(_) => Config::default(),
    };
    Ok(config)
}

fn parse(file: File) -> Result<Config, ConfigError> {
    let mut buf = String::new();
    BufReader::new(file)
        .read_to_string(&mut buf)
        .map_err(|e| ConfigError::IO(e))?;
    let res = toml::from_str::<Config>(&buf).map_err(|e| ConfigError::Deserialize(e))?;
    Ok(res)
}

pub fn write(config: Config) -> Result<(), ConfigError> {
    let serialized = toml::to_string(&config).map_err(|e| ConfigError::Serialize(e))?;
    let file = File::create(config_path()).map_err(|e| ConfigError::IO(e))?;
    BufWriter::new(file)
        .write_all(serialized.as_bytes())
        .map_err(|e| ConfigError::IO(e))
}

#[derive(Debug)]
pub enum ConfigError {
    IO(io::Error),
    Deserialize(toml::de::Error),
    Serialize(toml::ser::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub host: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "http://localhost:3004/roar/".to_string(),
        }
    }
}
