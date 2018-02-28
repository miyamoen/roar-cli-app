mod config;
mod app;

#[macro_use]
extern crate clap;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

use app::Cmd;
use app::ConfigCmd;

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
        Cmd::List => {
            let config = config::read().map_err(|err| CliError::Config(err))?;
            let mut resp = reqwest::get("https://www.rust-lang.org")?;

            let body = resp.text().unwrap();
            println!["{:?}", body];
        }
    };
    Ok(())
}

#[derive(Debug)]
enum CliError {
    Config(config::ConfigError),
}
