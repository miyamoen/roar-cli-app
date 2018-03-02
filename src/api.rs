extern crate reqwest;

use config::Config;
use reqwest::header::Accept;

pub fn list(config: &Config) -> Result<String, reqwest::Error> {
    let mut response = reqwest::get(&format!("{}feeds", config.host))?;
    let body = response.text();
    body
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Debug)]
pub struct AppRequest {
    name: String,
    icon_url: String,
}

impl AppRequest {
    pub fn new(name: String) -> Self {
        Self {
            icon_url: format!("http://flathash.com/{}", name),
            name: name,
        }
    }

    pub fn create(&self, config: &Config) -> Result<App, reqwest::Error> {
        let mut response = reqwest::Client::new()
            .post(&format!("{}feeds", config.host))
            .header(Accept::json())
            .json(self)
            .send()?;

        response.json::<App>()
    }
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct App {
    id: i32,
    name: String,
    icon_url: String,
}
