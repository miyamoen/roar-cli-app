mod config;
mod app;

#[macro_use]
extern crate clap;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

use app::{AppsCmd, Cmd, ConfigCmd};

fn main() {
    let cmd = app::create_command();
    let res = run(cmd);
    println!("result : {:?}", res);
}

fn run(cmd: Cmd) -> Result<(), CliError> {
    match cmd {
        Cmd::None(msg) => println!("fail to parse command : {:?}", msg),
        Cmd::Config(ConfigCmd::Show) => {
            let config = config::read().map_err(|err| CliError::Config(err))?;
            println!("Show configuration");
            println!("{:?}", config);
        }
        Cmd::Apps(AppsCmd::List) => {
            let config = config::read().map_err(|err| CliError::Config(err))?;
            let mut response =
                reqwest::get(&(config.host + "feeds")).map_err(|err| CliError::Reqwest(err))?;

            let body = response.text();
            println!["{:?}", body];
        }
    };
    Ok(())
}

#[derive(Debug)]
enum CliError {
    Config(config::ConfigError),
    Reqwest(reqwest::Error),
}
