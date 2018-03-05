extern crate reqwest;
extern crate serde_json;

use config::Config;
use reqwest::header::Accept;

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

    pub fn send(&self, config: &Config) -> Result<App, reqwest::Error> {
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

impl App {
    pub fn list(config: &Config) -> Result<Vec<App>, reqwest::Error> {
        let mut response = reqwest::Client::new()
            .get(&format!("{}feeds", config.host))
            .header(Accept::json())
            .send()?;
        response.json::<Vec<App>>()
    }
}

#[derive(Serialize, Debug)]
pub struct EntryRequest {
    #[serde(skip)]
    app_id: i32,
    title: String,
    summary: String,
    link: String,
    updated: String,
}

impl EntryRequest {
    pub fn new(app_id: i32, title: String) -> Self {
        Self {
            app_id: app_id,
            title: title,
            summary: "LIPSUM".to_string(),
            link: DUMMY_LINK.to_string(),
            updated: "2012-04-23T18:25:43.511Z".to_string(),
        }
    }

    pub fn send(&self, config: &Config) -> Result<(), reqwest::Error> {
        let test = serde_json::ser::to_string(&vec![self]);
        println!("test {:?}", test);
        println!("url : {}feed/{}", config.host, self.app_id);
        let mut response = reqwest::Client::new()
            .post(&format!("{}feed/{}", config.host, self.app_id))
            .header(Accept::json())
            .json(&vec![self])
            .send()?;

        let res = response.text()?;
        println!("res {:?}", res);
        Ok(())
    }
}

const  LIPSUM : &str = "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
const DUMMY_LINK: &str = "http://example.com/roar";
