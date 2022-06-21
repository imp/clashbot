use serde::de;

use super::*;

const API_URL: &str = "https://api.clashofclans.com/v1/";

#[derive(Debug)]
pub struct Client {
    token: String,
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn player(&self, player: impl AsRef<str>) -> Result<Player, ClientError> {
        let player = urlencoding::encode(player.as_ref());
        let url = self.url("players", &player)?;
        self.get(url)
    }

    fn url(&self, path: &str, arg: &str) -> Result<reqwest::Url, ClientError> {
        let url = format!("{API_URL}{path}/{arg}");
        reqwest::Url::parse(&url).map_err(ClientError::parse_error)
    }

    fn get<T>(&self, url: reqwest::Url) -> Result<T, ClientError>
    where
        T: de::DeserializeOwned,
    {
        let player = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .send()?
            .json()?;
        Ok(player)
    }
}
