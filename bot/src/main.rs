#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(clippy::use_self)]
#![warn(clippy::map_flatten)]
#![warn(clippy::map_unwrap_or)]
#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
#![deny(warnings)]

use std::fs;

use anyhow::Context;
use toml_edit::easy;

use store::Store;

mod bot;
mod store;

fn main() -> anyhow::Result<()> {
    let config = Config::load()?;

    let token = config
        .clashofclans()
        .context("No Clash Of Clans API token")?;

    let mut bot = match config.store_type().context("No store type")? {
        "mongo" => {
            let mongo = config.mongodb().context("No MongoDB credentials")?;
            let store = store::Mongo::new(mongo);
            bot::Bot::new(token, store)
        }
        "files" => {
            let base = config.files().context("No files section in config")?;
            let store = store::Files::new(base);
            bot::Bot::new(token, store)
        }
        other => {
            anyhow::bail!("Unknown store type: {other}");
        }
    };

    if let Some(seed) = config.seed_player() {
        bot.seed_players(seed);
    } else if let Some(seed) = config.seed_clan() {
        bot.seed_clans(seed);
    } else {
        bot.load();
    }

    bot.collect_new();
    bot.update();
    bot.checkpoint();

    Ok(())
}

struct Config {
    config: easy::Value,
}

impl Config {
    fn load() -> anyhow::Result<Self> {
        let base = directories::BaseDirs::new().context("Finding BaseDir")?;
        let config = base.home_dir().join(".clashbot").join("config.toml");
        let text = fs::read_to_string(config)?;
        let config: easy::Value = easy::from_str(&text)?;

        Ok(Self { config })
    }

    fn mongodb(&self) -> Option<String> {
        let mongodb = self.config.get("mongodb")?;
        let user = mongodb.get("user")?.as_str()?;
        let password = mongodb.get("password")?.as_str()?;
        let connection = mongodb
            .get("connection")?
            .as_str()?
            .replace("<user>", user)
            .replace("<password>", password);
        Some(connection)
    }

    fn clashofclans(&self) -> Option<&str> {
        self.config.get("clashofclans")?.get("token")?.as_str()
    }

    fn store_type(&self) -> Option<&str> {
        self.config.get("store")?.get("type")?.as_str()
    }

    fn files(&self) -> Option<&str> {
        self.config.get("files")?.get("path")?.as_str()
    }

    fn seed_player(&self) -> Option<Vec<&str>> {
        self.config
            .get("seed")?
            .get("player")
            .and_then(text_or_array)
    }

    fn seed_clan(&self) -> Option<Vec<&str>> {
        self.config.get("seed")?.get("clan").and_then(text_or_array)
    }
}

fn text_or_array(value: &easy::Value) -> Option<Vec<&str>> {
    let text = value.as_str().map(|text| vec![text]);
    let array = value
        .as_array()
        .map(|array| array.iter().filter_map(|v| v.as_str()).collect());
    text.or(array)
}
