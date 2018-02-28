extern crate toml;
use std::string::String;
use std::fs::File;
use std::io::{BufReader, Read};
use std::io;
use std::env;

pub fn read() -> Result<Config, ConfigError> {
    let default = Config {
        host: "localhost:3009/roar/".to_string(),
    };

    let config = match open_file() {
        Ok(file) => parse(file)?,
        Err(e) => {
            println!("An error occurred while opening file : {}", e);
            println!("run with default config");
            default
        }
    };
    Ok(config)
}

fn open_file() -> Result<File, io::Error> {
    let mut path = env::home_dir().unwrap();
    path.push(".roar.toml");
    File::open(path)
}

fn parse(file: File) -> Result<Config, ConfigError> {
    let mut buf = String::new();
    BufReader::new(file)
        .read_to_string(&mut buf)
        .map_err(|e| ConfigError::IOError(e))?;
    let res = toml::from_str::<Config>(&buf).map_err(|e| ConfigError::ParseError(e))?;
    Ok(res)
}

#[derive(Debug)]
pub enum ConfigError {
    IOError(io::Error),
    ParseError(toml::de::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub host: String,
}
