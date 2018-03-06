mod config;
mod cli;
mod api;

#[macro_use]
extern crate clap;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

use cli::{AppCmd, AppsCmd, Cmd, ConfigCmd, OverWriting};
use config::{Config, ConfigError};
use api::{App, AppRequest, EntryRequest};

fn main() {
    let cmd = cli::create_command();
    let res = run(cmd);
    println!("result : {:?}", res);
}

fn run(cmd: Cmd) -> Result<(), CliError> {
    match cmd {
        Cmd::None(msg) => println!("fail to decode a command : {:?}", msg),
        Cmd::Config(ConfigCmd::Show) => {
            let config = config::read().map_err(|err| CliError::Config(err))?;
            println!("Config : {:?}", config);
        }
        Cmd::Config(ConfigCmd::New(OverWriting::Force)) => {
            config::write(Config::default()).map_err(|err| CliError::Config(err))?
        }
        Cmd::Config(ConfigCmd::New(OverWriting::NotExists)) => {
            if config::exists() {
                println!("Config file has already exsisted.");
                println!(r#"Use "config new --force" to overwrite config file."#);
            } else {
                config::write(Config::default()).map_err(|err| CliError::Config(err))?
            }
        }
        Cmd::Apps(AppsCmd::List) => {
            let config = config::read().map_err(|err| CliError::Config(err))?;
            let list = App::list(&config).map_err(|err| CliError::Reqwest(err))?;
            println!["{:?}", list];
        }
        Cmd::Apps(AppsCmd::Create(name)) => {
            let config = config::read().map_err(|err| CliError::Config(err))?;
            let app = AppRequest::new(name)
                .send(&config)
                .map_err(|err| CliError::Reqwest(err))?;
            println!("Registered : {:?}", app);
        }
        Cmd::App(AppCmd::Send { app_id, title, n }) => {
            let config = config::read().map_err(|err| CliError::Config(err))?;
            let status = EntryRequest::new(app_id, title)
                .send(&config)
                .map_err(|err| CliError::Reqwest(err))?;
            println!("Response status code : {}", status);
        }
    };
    Ok(())
}

#[derive(Debug)]
enum CliError {
    Config(ConfigError),
    Reqwest(reqwest::Error),
}
