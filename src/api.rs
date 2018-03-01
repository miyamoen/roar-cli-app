extern crate reqwest;

use config::Config;

pub fn list(config: Config) -> Result<String, reqwest::Error> {
    let mut response = reqwest::get(&(config.host + "feeds"))?;

    let body = response.text();
    body
}

pub struct App {
    name: String,
    iconUrl: String,
}

impl App {
    fn new(name: String) -> Self {
        Self {
            iconUrl: format!("http://flathash.com/{}", name),
            name: name,
        }
    }
}
