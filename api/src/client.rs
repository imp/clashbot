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

    pub fn clan(&self, clan: impl AsRef<str>) -> Result<Clan, ClientError> {
        let clan = urlencoding::encode(clan.as_ref());
        let url = self.url("clans", &clan)?;
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
            .error_for_status2()?
            .json()?;
        Ok(player)
    }
}

trait ResponseExt: Sized {
    fn error_for_status2(self) -> Result<Self, ClientError>;
}

impl ResponseExt for reqwest::blocking::Response {
    fn error_for_status2(self) -> Result<Self, ClientError> {
        if self.status().is_success() {
            Ok(self)
        } else {
            let error = self.json()?;
            Err(error)
        }
    }
}
